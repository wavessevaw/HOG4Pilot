use hog4pilot::{fixture::{virtual_rgb,Function},lighting::Look,safety::SafetyEngine,output::{art_dmx,Output,Simulator,ArtNet}};
fn look()->Look { Look{intensity:0.5,rgb:[1.0,0.0,0.25]} }
fn engine()->SafetyEngine { SafetyEngine::new(vec![virtual_rgb("a",1)],Some([0;512]),0,"a".into()).unwrap() }
#[test] fn full_frame_and_encoding(){let f=engine().render("a",look()).unwrap(); assert_eq!(&f.values()[..4],&[128,255,0,64]);let p=art_dmx(&f,1);assert_eq!(p.len(),530);assert_eq!(&p[..18],b"Art-Net\0\0\x50\0\x0e\x01\0\0\0\x02\0");}
#[test] fn rejects_unknown(){let mut f=virtual_rgb("a",1);f.profile.verified=false;assert!(SafetyEngine::new(vec![f],Some([0;512]),0,"a".into()).is_err());}
#[test] fn rejects_overlap(){assert!(SafetyEngine::new(vec![virtual_rgb("a",1),virtual_rgb("b",4)],Some([0;512]),0,"a".into()).is_err());}
#[test] fn rejects_missing_baseline(){assert!(SafetyEngine::new(vec![],None,0,"a".into()).is_err());}
#[test] fn rejects_invalid_values(){for n in [f32::NAN,f32::INFINITY,-0.1,1.1]{assert!(engine().render("a",Look{intensity:n,..look()}).is_err());}}
#[test] fn conflict_latches_stop(){let mut e=engine();assert!(e.render("b",look()).is_err());assert!(e.render("a",look()).is_err());}
#[test] fn locked_nonzero_neutral(){let mut f=virtual_rgb("a",1); f.profile.channels[0].function=Function::Locked;f.profile.channels[0].neutral=42;let mut baseline=[0;512];baseline[0]=42;let mut e=SafetyEngine::new(vec![f],Some(baseline),0,"a".into()).unwrap();assert_eq!(e.render("a",look()).unwrap().values()[0],42);}
#[test] fn boundaries(){for address in [0,510,usize::MAX]{assert!(SafetyEngine::new(vec![virtual_rgb("a",address)],Some([0;512]),0,"a".into()).is_err());}assert!(SafetyEngine::new(vec![virtual_rgb("a",509)],Some([0;512]),32767,"a".into()).is_ok());}
#[test] fn fade_and_loss(){assert_eq!(look().fade(Look{intensity:1.0,..look()},0.5).unwrap().intensity,0.75);let mut sim=Simulator::default();sim.disconnected=true;assert!(sim.send(&engine().render("a",look()).unwrap()).is_err());}
#[test] fn udp_loopback(){let receiver=std::net::UdpSocket::bind("127.0.0.1:6454").unwrap();receiver.set_read_timeout(Some(std::time::Duration::from_secs(1))).unwrap();let mut out=ArtNet::connect("127.0.0.1:0".parse().unwrap(),receiver.local_addr().unwrap()).unwrap();out.send(&engine().render("a",look()).unwrap()).unwrap();let mut buf=[0;600];let(n,_)=receiver.recv_from(&mut buf).unwrap();assert_eq!(n,530);assert_eq!(buf[18],128);}
