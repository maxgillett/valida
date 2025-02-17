use p3_air::constraint_consumer::ConstraintConsumer;
use p3_air::types::AirTypes;
use p3_air::window::AirWindow;
use p3_air::Air;

pub trait Chip<T, W, CC>: Air<T, W, CC>
where
    T: AirTypes,
    W: AirWindow<T::Var>,
    CC: ConstraintConsumer<T>,
{
}
