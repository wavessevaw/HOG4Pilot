//! HOG4Pilot core. Only Safety Engine can construct output frames.
pub mod fixture {
    #[derive(Clone, Debug)]
    pub enum Function { Intensity, Red, Green, Blue, Locked }
    #[derive(Clone, Debug)]
    pub struct Channel { pub function: Function, pub neutral: u8, pub min: u8, pub max: u8 }
    #[derive(Clone, Debug)]
    pub struct Profile { pub id: String, pub version: u32, pub source: String, pub verified: bool, pub channels: Vec<Channel> }
    #[derive(Clone, Debug)]
    pub struct Fixture { pub id: String, pub address: usize, pub profile: Profile }
    pub fn virtual_rgb(id: &str, address: usize) -> Fixture {
        Fixture { id: id.into(), address, profile: Profile { id: "virtual-rgbd".into(), version: 1, source: "Synthetic simulator only".into(), verified: true,
            channels: [Function::Intensity, Function::Red, Function::Green, Function::Blue].into_iter().map(|function| Channel { function, neutral: 0, min: 0, max: 255 }).collect() } }
    }
}
pub mod lighting {
    #[derive(Clone, Copy, Debug)]
    pub struct Look { pub intensity: f32, pub rgb: [f32; 3] }
    impl Look {
        pub fn validate(self) -> Result<(), String> {
            if [self.intensity, self.rgb[0], self.rgb[1], self.rgb[2]].iter().any(|v| !v.is_finite() || !(0.0..=1.0).contains(v)) { return Err("Яркость и цвет должны находиться в диапазоне 0–1".into()); } Ok(())
        }
        pub fn fade(self, next: Self, progress: f32) -> Result<Self, String> {
            self.validate()?; next.validate()?;
            if !progress.is_finite() || !(0.0..=1.0).contains(&progress) { return Err("Некорректное время перехода".into()); }
            let mix = |a, b| a + (b-a)*progress;
            Ok(Self { intensity: mix(self.intensity, next.intensity), rgb: std::array::from_fn(|i| mix(self.rgb[i], next.rgb[i])) })
        }
    }
}
pub mod safety {
    use super::{fixture::{Fixture, Function}, lighting::Look};
    pub struct ValidatedFrame { pub(crate) values: [u8; 512], pub(crate) universe: u16 }
    impl ValidatedFrame { pub fn values(&self) -> &[u8;512] { &self.values } }
    /// Full universe ownership and a verified baseline are mandatory, including unused slots.
    pub struct SafetyEngine { fixtures: Vec<Fixture>, baseline: [u8;512], universe: u16, stopped: bool, owner: String, pub journal: Vec<String> }
    impl SafetyEngine {
        pub fn new(fixtures: Vec<Fixture>, baseline: Option<[u8;512]>, universe: u16, owner: String) -> Result<Self,String> {
            if universe > 32767 || owner.trim().is_empty() { return Err("Некорректный источник или номер пространства".into()); }
            let baseline = baseline.ok_or("Нет подтверждённого полного исходного кадра")?;
            let mut occupied = [false;512];
            let mut ids = std::collections::HashSet::new();
            for f in &fixtures {
                if f.id.is_empty() || !ids.insert(&f.id) || !f.profile.verified || f.profile.source.is_empty() || f.profile.version == 0 || f.profile.channels.is_empty() { return Err("Прибор или профиль не проверен".into()); }
                let end = f.address.checked_add(f.profile.channels.len()).ok_or("Переполнение адреса")?;
                if f.address == 0 || end > 513 { return Err("Адрес выходит за границы".into()); }
                for (i,c) in f.profile.channels.iter().enumerate() {
                    let slot = f.address-1+i;
                    if occupied[slot] { return Err("Конфликт адресов".into()); }
                    if c.min > c.max || !(c.min..=c.max).contains(&c.neutral) || baseline[slot] != c.neutral { return Err("Непроверенное нейтральное значение".into()); }
                    occupied[slot] = true;
                }
            }
            Ok(Self { fixtures, baseline, universe, stopped: false, owner, journal: vec!["Сеанс создан".into()] })
        }
        pub fn stop(&mut self) { self.stopped = true; self.journal.push("Остановлено. Физическая остановка не подтверждена".into()); }
        pub fn render(&mut self, source: &str, look: Look) -> Result<ValidatedFrame,String> {
            let result = self.render_inner(source,look);
            self.journal.push(format!("Источник={source}; {}", if result.is_ok() { "Кадр проверен" } else { "Заблокировано" })); result
        }
        fn render_inner(&mut self, source: &str, look: Look) -> Result<ValidatedFrame,String> {
            if source != self.owner { self.stop(); return Err("Конфликт источников: сеанс заблокирован".into()); }
            if self.stopped { return Err("Управление остановлено; создайте новый проверенный сеанс".into()); }
            look.validate()?;
            let mut values = self.baseline;
            for f in &self.fixtures { for (i,c) in f.profile.channels.iter().enumerate() {
                let v = match c.function { Function::Intensity => Some(look.intensity), Function::Red => Some(look.rgb[0]), Function::Green => Some(look.rgb[1]), Function::Blue => Some(look.rgb[2]), Function::Locked => None };
                values[f.address-1+i] = v.map(|v| (c.min as f32 + v*(c.max-c.min) as f32).round() as u8).unwrap_or(c.neutral);
            }}
            Ok(ValidatedFrame { values, universe: self.universe })
        }
    }
}
pub mod output {
    use super::safety::ValidatedFrame;
    use std::{io, net::{SocketAddr, UdpSocket}, time::{Duration,Instant}};
    pub trait Output { fn send(&mut self, frame: &ValidatedFrame) -> io::Result<()>; }
    #[derive(Default)]
    pub struct Simulator { pub frames: Vec<[u8;512]>, pub disconnected: bool }
    impl Output for Simulator { fn send(&mut self, frame: &ValidatedFrame) -> io::Result<()> { if self.disconnected { return Err(io::Error::new(io::ErrorKind::NotConnected,"Имитация потери связи")); } self.frames.push(frame.values); Ok(()) } }
    pub fn art_dmx(frame: &ValidatedFrame, sequence: u8) -> Vec<u8> {
        let mut packet = Vec::with_capacity(530);
        packet.extend_from_slice(b"Art-Net\0"); packet.extend_from_slice(&[0x00,0x50,0,14,sequence,0]);
        packet.extend_from_slice(&frame.universe.to_le_bytes()); packet.extend_from_slice(&[2,0]); packet.extend_from_slice(&frame.values); packet
    }
    /// UDP send success is NOT evidence of receiver execution.
    pub struct ArtNet { socket: UdpSocket, destination: SocketAddr, sequence: u8, last: Option<Instant> }
    impl ArtNet {
        pub fn connect(bind: SocketAddr, destination: SocketAddr) -> io::Result<Self> {
            if !bind.is_ipv4() || !destination.is_ipv4() || destination.port()!=6454 { return Err(io::Error::new(io::ErrorKind::InvalidInput,"Art-Net требует IPv4 и порт 6454")); }
            let socket = UdpSocket::bind(bind)?; socket.set_nonblocking(true)?;
            Ok(Self { socket, destination, sequence: 0, last: None })
        }
    }
    impl Output for ArtNet { fn send(&mut self, frame: &ValidatedFrame) -> io::Result<()> {
        if self.last.is_some_and(|t| t.elapsed()<Duration::from_millis(25)) { return Err(io::Error::new(io::ErrorKind::WouldBlock,"Ограничение 40 кадров/с")); }
        self.sequence = if self.sequence==255 {1} else {self.sequence+1};
        self.socket.send_to(&art_dmx(frame,self.sequence),self.destination)?; self.last=Some(Instant::now()); Ok(())
    }}
}
