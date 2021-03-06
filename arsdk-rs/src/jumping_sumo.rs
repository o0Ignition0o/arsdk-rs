// use arsdk_derive::FramePart;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
// TODO: Check where and how this is encoded?
pub enum JumpType {
    LONG,    // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_PILOTING = 0,
    HIGH,    // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_PILOTING = 0,
    DEFAULT, // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_PILOTING = 0,
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// u8
pub enum Class {
    Piloting(PilotingID), // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_PILOTING = 0,
    PilotingState,        // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_PILOTINGSTATE = 1,
    Animations(Anim),     // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_ANIMATIONS = 2,
    AnimationsState,      // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_ANIMATIONSSTATE = 3,
    SettingsState,        // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_SETTINGSSTATE = 5,
    MediaRecord,          // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_MEDIARECORD = 6,
    MediaRecordState,     // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_MEDIARECORDSTATE = 7,
    NetworkSettings,      // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_NETWORKSETTINGS = 8,
    NetworkSettingsState, // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_NETWORKSETTINGSSTATE = 9,
    Network,              // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_NETWORK = 10,
    NetworkState,         // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_NETWORKSTATE = 11,
    AutioSettings,        // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_AUDIOSETTINGS = 12,
    AudioSettingsState,   // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_AUDIOSETTINGSSTATE = 13,
    Roadplan,             // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_ROADPLAN = 14,
    RoadplanState,        // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_ROADPLANSTATE = 15,
    SpeedSettings,        // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_SPEEDSETTINGS = 16,
    SpeedSettingsState,   // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_SPEEDSETTINGSSTATE = 17,
    MediaStreaming,       // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_MEDIASTREAMING = 18,
    MediaStreamingState,  // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_MEDIASTREAMINGSTATE = 19,
    MediaRecordEvent,     // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_MEDIARECORDEVENT = 20,
    VideoSettings,        // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_VIDEOSETTINGS = 21,
    VideoSettingsState,   // ARCOMMANDS_ID_JUMPINGSUMO_CLASS_VIDEOSETTINGSSTATE = 22,
}

// #[derive(Debug, PartialEq, Eq, Clone, Copy, FramePart)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Anim {
    JumpStop,        // ARCOMMANDS_ID_JUMPINGSUMO_ANIMATIONS_CMD_JUMPSTOP = 0,
    JumpCancel,      // ARCOMMANDS_ID_JUMPINGSUMO_ANIMATIONS_CMD_JUMPCANCEL = 1,
    JumpLoad,        // ARCOMMANDS_ID_JUMPINGSUMO_ANIMATIONS_CMD_JUMPLOAD = 2,
    Jump,            // ARCOMMANDS_ID_JUMPINGSUMO_ANIMATIONS_CMD_JUMP = 3,
    SimpleAnimation, // ARCOMMANDS_ID_JUMPINGSUMO_ANIMATIONS_CMD_SIMPLEANIMATION = 4,
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// u16
pub enum PilotingID {
    Pilot(PilotState), // ARCOMMANDS_ID_JUMPINGSUMO_PILOTING_CMD_PCMD = 0,
    Posture,           // ARCOMMANDS_ID_JUMPINGSUMO_PILOTING_CMD_POSTURE = 1,
    AddCapOffset,      // ARCOMMANDS_ID_JUMPINGSUMO_PILOTING_CMD_ADDCAPOFFSET = 2,
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct PilotState {
    pub flag: bool,
    pub speed: i8,
    pub turn: i8,
}

// --------------------- Conversion impls --------------------- //

impl Into<u8> for &Class {
    fn into(self) -> u8 {
        match self {
            Class::Piloting(_) => 0,
            Class::PilotingState => 1,
            Class::Animations(_) => 2,
            Class::AnimationsState => 3,
            Class::SettingsState => 5,
            Class::MediaRecord => 6,
            Class::MediaRecordState => 7,
            Class::NetworkSettings => 8,
            Class::NetworkSettingsState => 9,
            Class::Network => 10,
            Class::NetworkState => 11,
            Class::AutioSettings => 12,
            Class::AudioSettingsState => 13,
            Class::Roadplan => 14,
            Class::RoadplanState => 15,
            Class::SpeedSettings => 16,
            Class::SpeedSettingsState => 17,
            Class::MediaStreaming => 18,
            Class::MediaStreamingState => 19,
            Class::MediaRecordEvent => 20,
            Class::VideoSettings => 21,
            Class::VideoSettingsState => 22,
        }
    }
}

impl Into<u8> for Anim {
    fn into(self) -> u8 {
        match self {
            Self::JumpStop => 0,
            Self::JumpCancel => 1,
            Self::JumpLoad => 2,
            Self::Jump => 3,
            Self::SimpleAnimation => 4,
        }
    }
}

impl Into<u16> for &PilotingID {
    fn into(self) -> u16 {
        match self {
            PilotingID::Pilot(_) => 0,
            PilotingID::Posture => 1,
            PilotingID::AddCapOffset => 2,
        }
    }
}
pub mod scroll_impl {
    use super::*;
    use crate::frame::Error;
    use scroll::{ctx, Endian, Pread, Pwrite};

    impl<'a> ctx::TryFromCtx<'a, Endian> for Class {
        type Error = Error;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
            let mut offset = 0;

            let class = match src.gread_with::<u8>(&mut offset, ctx)? {
                0 => {
                    let pilot_state = src.gread_with(&mut offset, ctx)?;

                    Self::Piloting(pilot_state)
                }
                1 => Self::PilotingState,
                2 => {
                    let anim = src.gread_with(&mut offset, ctx)?;

                    Self::Animations(anim)
                }
                3 => Self::AnimationsState,
                5 => Self::SettingsState,
                6 => Self::MediaRecord,
                7 => Self::MediaRecordState,
                8 => Self::NetworkSettings,
                9 => Self::NetworkSettingsState,
                10 => Self::Network,
                11 => Self::NetworkState,
                12 => Self::AutioSettings,
                13 => Self::AudioSettingsState,
                14 => Self::Roadplan,
                15 => Self::RoadplanState,
                16 => Self::SpeedSettings,
                17 => Self::SpeedSettingsState,
                18 => Self::MediaStreaming,
                19 => Self::MediaStreamingState,
                20 => Self::MediaRecordEvent,
                21 => Self::VideoSettings,
                22 => Self::VideoSettingsState,
                value => {
                    return Err(Self::Error::OutOfBound {
                        value: value.into(),
                        param: "Class".to_string(),
                    })
                }
            };

            Ok((class, offset))
        }
    }

    impl<'a> ctx::TryIntoCtx<Endian> for Class {
        type Error = Error;

        fn try_into_ctx(self, this: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
            let mut offset = 0;

            this.gwrite_with::<u8>((&self).into(), &mut offset, ctx)?;
            match self {
                Self::Piloting(piloting_id) => {
                    this.gwrite_with(piloting_id, &mut offset, ctx)?;
                }
                Self::Animations(anim) => {
                    this.gwrite_with(anim, &mut offset, ctx)?;
                }
                _ => unimplemented!("Not all Class are impled"),
            }

            Ok(offset)
        }
    }

    impl<'a> ctx::TryFromCtx<'a, Endian> for PilotingID {
        type Error = Error;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
            let mut offset = 0;

            let piloting_id = match src.gread_with::<u16>(&mut offset, ctx)? {
                0 => {
                    let pilot_state = src.gread_with(&mut offset, ctx)?;

                    Self::Pilot(pilot_state)
                }
                1 => Self::Posture,
                2 => Self::AddCapOffset,
                value => {
                    return Err(Self::Error::OutOfBound {
                        value: value.into(),
                        param: "PilotingId".to_string(),
                    })
                }
            };

            Ok((piloting_id, offset))
        }
    }

    impl<'a> ctx::TryIntoCtx<Endian> for PilotingID {
        type Error = Error;

        fn try_into_ctx(self, this: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
            let mut offset = 0;
            this.gwrite_with::<u16>((&self).into(), &mut offset, ctx)?;

            match self {
                Self::Pilot(state) => {
                    this.gwrite_with(state, &mut offset, ctx)?;
                }
                _ => unimplemented!("Not all PilotingID are impled"),
            }

            Ok(offset)
        }
    }

    impl<'a> ctx::TryFromCtx<'a, Endian> for PilotState {
        type Error = Error;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
            let mut offset = 0;

            let flag = match src.gread_with::<u8>(&mut offset, ctx)? {
                0 => false,
                1 => true,
                // @TODO: should we mention that it is for PilotState as well and how?
                value => {
                    return Err(Self::Error::OutOfBound {
                        value: value.into(),
                        param: "flag".to_string(),
                    })
                }
            };
            let speed: i8 = src.gread_with(&mut offset, ctx)?;
            let turn: i8 = src.gread_with(&mut offset, ctx)?;

            let pilot_state = PilotState { flag, speed, turn };

            Ok((pilot_state, offset))
        }
    }

    impl<'a> ctx::TryIntoCtx<Endian> for PilotState {
        type Error = Error;

        fn try_into_ctx(self, this: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
            let mut offset = 0;
            this.gwrite_with::<u8>(self.flag.into(), &mut offset, ctx)?;
            this.gwrite_with(self.speed, &mut offset, ctx)?;
            this.gwrite_with(self.turn, &mut offset, ctx)?;

            Ok(offset)
        }
    }

    impl<'a> ctx::TryFromCtx<'a, Endian> for Anim {
        type Error = Error;

        // and the lifetime annotation on `&'a [u8]` here
        fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
            let mut offset = 0;

            let anim = match src.gread_with::<u8>(&mut offset, ctx)? {
                0 => Self::JumpStop,
                1 => Self::JumpCancel,
                2 => Self::JumpLoad,
                3 => Self::Jump,
                4 => Self::SimpleAnimation,
                value => {
                    return Err(Self::Error::OutOfBound {
                        value: value.into(),
                        param: "Anim".to_string(),
                    })
                }
            };

            let mut anim_data = [0_u8; 5];
            src.gread_inout_with(&mut offset, &mut anim_data, ctx)?;

            Ok((anim, offset))
        }
    }

    impl<'a> ctx::TryIntoCtx<Endian> for Anim {
        type Error = Error;

        fn try_into_ctx(self, this: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
            let mut offset: usize = 0;
            this.gwrite_with::<u8>((self).into(), &mut offset, ctx)?;
            // TODO: FIX THIS!
            let dummy_anim = [0_u8; 5];
            this.gwrite_with(dummy_anim.as_ref(), &mut offset, ())?;

            Ok(offset)
        }
    }
}

// --------------------- Tests --------------------- //

#[cfg(test)]
mod jumping_dumo_tests {
    use super::*;
    use std::borrow::Borrow;

    #[test]
    fn test_piloting_command() {
        assert_piloting(PilotingID::Pilot(PilotState::default()), 0);
        assert_piloting(PilotingID::Posture, 1);
        assert_piloting(PilotingID::AddCapOffset, 2);
    }

    #[test]
    fn test_anim() {
        assert_anim(Anim::JumpStop, 0);
        assert_anim(Anim::JumpCancel, 1);
        assert_anim(Anim::JumpLoad, 2);
        assert_anim(Anim::Jump, 3);
        assert_anim(Anim::SimpleAnimation, 4);
    }

    #[test]
    fn test_class() {
        assert_class(Class::Piloting(PilotingID::Pilot(PilotState::default())), 0);
        assert_class(Class::PilotingState, 1);
        assert_class(Class::Animations(Anim::Jump), 2);
        assert_class(Class::AnimationsState, 3);
        assert_class(Class::SettingsState, 5);
        assert_class(Class::MediaRecord, 6);
        assert_class(Class::MediaRecordState, 7);
        assert_class(Class::NetworkSettings, 8);
        assert_class(Class::NetworkSettingsState, 9);
        assert_class(Class::Network, 10);
        assert_class(Class::NetworkState, 11);
        assert_class(Class::AutioSettings, 12);
        assert_class(Class::AudioSettingsState, 13);
        assert_class(Class::Roadplan, 14);
        assert_class(Class::RoadplanState, 15);
        assert_class(Class::SpeedSettings, 16);
        assert_class(Class::SpeedSettingsState, 17);
        assert_class(Class::MediaStreaming, 18);
        assert_class(Class::MediaStreamingState, 19);
        assert_class(Class::MediaRecordEvent, 20);
        assert_class(Class::VideoSettings, 21);
        assert_class(Class::VideoSettingsState, 22);
    }

    fn assert_class(dc: impl Borrow<Class>, v: u8) {
        let as_u8: u8 = dc.borrow().into();
        assert_eq!(v, as_u8);
    }

    fn assert_anim(a: Anim, v: u8) {
        let as_u8: u8 = a.into();
        assert_eq!(v, as_u8);
    }

    fn assert_piloting(pc: impl Borrow<PilotingID>, v: u16) {
        let as_u8: u16 = pc.borrow().into();
        assert_eq!(v, as_u8);
    }
}
