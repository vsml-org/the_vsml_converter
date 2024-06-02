enum IVType {
    Wrp,
    Vobj,
    Aobj,
    Tobj,
}

trait Style {}

pub struct IVData {
    iv_type: IVType,
    start_time: f64,
    duration: f64,
    src: Option<String>,
    text: Option<String>,
    styles: Vec<Box<dyn Style>>,
}
