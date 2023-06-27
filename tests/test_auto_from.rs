#[cfg(test)]
mod test {
    use auto_from::auto_from;
    use fake::{Dummy, Fake, Faker};
    use rstest::*;

    #[derive(PartialEq, Eq, Debug, Clone, Dummy)]
    #[auto_from(Model2)]
    struct Model1 {
        id: i32,
        name: String,
        attrs: Vec<String>,
    }

    #[auto_from(Model1)]
    #[derive(PartialEq, Eq, Debug, Clone, Dummy)]
    struct Model2 {
        id: i32,
        name: String,
        attrs: Vec<String>,
    }

    type Factory<M> = fn() -> M;

    #[fixture]
    fn model_1_factory() -> Factory<Model1> {
        || Faker.fake()
    }

    #[fixture]
    fn model_2_factory() -> Factory<Model2> {
        || Faker.fake()
    }

    #[rstest]
    fn test_auto_from(model_1_factory: Factory<Model1>, model_2_factory: Factory<Model2>) {
        // given
        let m1 = model_1_factory();
        let m2 = model_2_factory();

        // when
        let m1_2: Model2 = m1.clone().into();
        let m2_1: Model1 = m2.clone().into();

        println!("{:#?} {:#?}", m2, m1);
        println!("{:#?} {:#?}", m1_2, m2_1);

        // then
        assert!(m1.id == m1_2.id);
        assert!(m1.name == m1_2.name);
        assert!(m1.attrs == m1_2.attrs);

        assert!(m2.id == m2_1.id);
        assert!(m2.name == m2_1.name);
        assert!(m2.attrs == m2_1.attrs);
    }

    #[rstest]
    fn test_auto_from_transitive(
        model_1_factory: Factory<Model1>,
        model_2_factory: Factory<Model2>,
    ) {
        // given
        let m1 = model_1_factory();
        let m2 = model_2_factory();

        // when
        let m1_2: Model2 = m1.clone().into();
        let m2_1: Model1 = m2.clone().into();
        let m1_2_1: Model1 = m1_2.into();
        let m2_1_2: Model2 = m2_1.into();

        // then
        assert!(m1 == m1_2_1);
        assert!(m2 == m2_1_2);
    }
}
