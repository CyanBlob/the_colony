use bevy::prelude::Component;
use bevy_enum_filter::EnumFilter;
use crate::task_scorer::Task;
use strum_macros::{EnumString, IntoStaticStr, AsRefStr};


#[derive(EnumFilter, Component, Debug, Clone, Copy, EnumString, IntoStaticStr, AsRefStr)]
pub enum AllTasks {
    Wander,
    Drink,
    Eat,
    Sleep,
}

impl Default for AllTasks {
    fn default() -> Self {
        AllTasks::Wander
    }
}

#[derive(Component)]
pub struct Thirst {
    pub value: f32,
    pub drain_rate: f32,
}

#[derive(Component)]
pub struct Hunger {
    pub value: f32,
    pub drain_rate: f32,
}

pub struct Sleep {
    pub value: f32,
    pub drain_rate: f32,
}

impl Default for Thirst {
    fn default() -> Self {
        Thirst {
            value: 100.0,
            drain_rate: 1.0,
        }
    }
}

impl Default for Hunger {
    fn default() -> Self {
        Hunger {
            value: 100.0,
            drain_rate: 10.0,
        }
    }
}

impl Default for Sleep {
    fn default() -> Self {
        Sleep {
            value: 100.0,
            drain_rate: 1.0,
        }
    }
}


impl Task for Thirst {
    fn score(&self) -> f32 {
        if self.value < 50.0 {
            10.0
        } else {
            0.0
        }
    }
}

impl Task for Hunger {
    fn score(&self) -> f32 {
        if self.value < 30.0 {
            10.0
        } else {
            0.0
        }
    }
}

impl Task for Sleep {
    fn score(&self) -> f32 {
        return 10.0;
    }
}
