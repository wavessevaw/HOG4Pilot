use hog4pilot::{fixture::virtual_rgb, lighting::Look, output::{Output,Simulator}, safety::SafetyEngine};
fn main() -> Result<(),String> {
    let mut safety=SafetyEngine::new(vec![virtual_rgb("Лево",1),virtual_rgb("Центр",5),virtual_rgb("Право",9)],Some([0;512]),0,"operator".into())?;
    let mut output=Simulator::default();
    let start=Look{intensity:0.0,rgb:[1.0,0.65,0.3]};
    let end=Look{intensity:0.8,..start};
    for step in 0..=80 { let frame=safety.render("operator",start.fade(end,step as f32/80.0)?)?; output.send(&frame).map_err(|e|e.to_string())?; }
    println!("HOG4Pilot 0.1 — СИМУЛЯЦИЯ. Сетевая отправка выключена.");
    println!("3 виртуальных прибора; переход 81 кадр; итог {:?}",&output.frames.last().unwrap()[..12]);
    output.disconnected=true;
    let frame=safety.render("operator",end)?;
    if output.send(&frame).is_err() { safety.stop(); }
    assert!(safety.render("operator",end).is_err());
    for entry in safety.journal.iter().rev().take(4).rev() { println!("{entry}"); }
    Ok(())
}
