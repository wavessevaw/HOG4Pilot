use hog4pilot::import::dmx_csv;
#[test]fn imported_is_unverified(){let p=dmx_csv("OEM","manual p.3","offset,function,neutral,min,max\n1,intensity,0,0,255\n2,locked,42,42,42").unwrap();assert!(!p.verified);assert_eq!(p.channels[1].neutral,42);}
#[test]fn refuses_ambiguous(){for text in ["1,reset,0,0,255","2,red,0,0,255","1,red,0,10,255","1,red,256,0,255",""]{assert!(dmx_csv("OEM","manual",text).is_err());}}
