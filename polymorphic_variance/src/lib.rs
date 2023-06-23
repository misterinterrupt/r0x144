// cars can be created with a brand name
// cars have a property that holds an engine and it has a type that is generic over the car brand name
// cars have a property that holds a transmission and it has a type that is generic over the car brand name
// engines have a property that holds a transmission and it has a type that is generic over the car brand name
// transmissions have a property that holds a brand name
// cars can be created with an engine
// I needed the ability to have the engine generate a transmission because I wanted use the engine's brand to determine the transmission's brand
// this is not related to the car theme, but I wanted to be able to create something with a thing that has an identifying type
// and have it generate another thing that has its same identifying type.
// I tried doing this with enum struct variants, but these are not types.
// thus we use empty structs for each brand. each of these implement a trait and the trait is used as a generic constraint
// we save the brand type in a phantom data field so that we can use it as a generic constraint to construct things later

struct Car<B: Brand> {
    engine: Engine<B>,
    transmission: Option<Transmission<B>>,
}

impl<B: Brand> Car<B> {
    fn new(engine: Engine<B>) -> Self {
        Self {
            transmission: None,
            engine,
        }
    }
    fn generate_transmission(&mut self) {
        self.transmission = self.engine.generate_transmission();
    }
}

struct Engine<B: Brand> {
    transmission: Option<Transmission<B>>,
    brand: std::marker::PhantomData<B>,
}

impl<B: Brand> Engine<B> {
    fn new() -> Self {
        Self {
            transmission: None,
            brand: std::marker::PhantomData,
        }
    }
    fn generate_transmission(&mut self) -> Option<Transmission<B>> {
        self.transmission = Some(Transmission::<B>::new());
        Some(Transmission::<B>::new())
    }
}

struct Transmission<B: Brand> {
    brand: std::marker::PhantomData<B>,
}

impl<B: Brand> Transmission<B> {
    fn new() -> Self {
        Self {
            brand: std::marker::PhantomData,
        }
    }
}

trait Brand {}

struct Honda;
impl Brand for Honda {}

struct Toyota;
impl Brand for Toyota {}

struct Ford;
impl Brand for Ford {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generate_branded_transmission_from_car() {
        let mut car = Car::<Honda>::new(Engine::<Honda>::new());
        car.generate_transmission();
        assert!(car.transmission.is_some());
        assert_eq!(
            car.transmission.unwrap().brand,
            Transmission::<Honda>::new().brand
        );
    }
}
