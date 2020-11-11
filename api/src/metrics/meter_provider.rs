// unimplemented!()
pub struct MeterProvider {
    pub name: &'static str,
    pub version: &'static str,
    pub meter: Vec<String>,
}

// pub struct TracerProvider {
//     pub name: &'static str,
//     pub version: &'static str,
//     pub tracer: Tracer,
// }
//
// impl TracerProvider {
//     pub fn default() -> TracerProvider {
//         TracerProvider {
//             name: env!("CARGO_PKG_NAME"),
//             version: env!("CARGO_PKG_VERSION"),
//             tracer: Tracer::init(),
//         }
//     }
//
//     pub fn get_tracer(&mut self) -> &mut Tracer {
//         &mut self.tracer
//     }
// }
impl MeterProvider {
    pub fn default() -> MeterProvider {
        MeterProvider {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            meter: Vec::new(),
        }
    }
}
