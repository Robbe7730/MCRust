use crate::packets::packet_writer::PacketWriter;

use super::Clientbound;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum EntityStatus {
    TippedArrowParticles,

    RabbitRotatedJumping,
    MinecartSpawnerResetDelay,

    HurtAnimation,

    DeathSoundAnimation,
    SnowballPoof,
    EggBreaking,

    IronGolemAttachSoundAnimation,
    EvokerFangsAttackAnimation,
    RavagerAttackAnimation,

    TamableTamingFailed,

    TamableTamingSucceeded,

    WolfShakeWaterAnimation,

    PlayerItemUseFinished,

    SheepGrassAnimation,
    MinecartTNTIgniteAnimation,

    IronGolemHoldPoppy,

    VillagerMatingComplete,

    VillagerAngry,

    VillagerHappy,

    WitchMagicParticles,

    ZombieVillagerCured,

    FireworkExplosion,

    AnimalLoveModeParticles,

    SquidResetRotation,

    MobExplosionParticle,

    GuardianAttackSound,

    PlayerReducedDebugInfoEnabled,

    PlayerReducedDebugInfoDisabled,

    PlayerOPLevel0,

    PlayerOPLevel1,

    PlayerOPLevel2,

    PlayerOPLevel3,

    PlayerOPLevel4,

    LivingEntityShieldBlock,

    LivingEntityShieldBreak,

    FishingHookToPlayer,

    ArmorStandHit,

    LivingEntityThorns,

    IronGolemRemovePoppy,

    EntityTotemOfUndying,

    LivingEntityHurtAndDrown,

    LivingEntityHurtAndBurn,

    DolphinHappy,

    RavagerStunned,

    CatTamingFailed,

    CatTamingSucceeded,

    VillagerRaidSweat,

    PlayerBadOmenCloudEffect,

    LivingEntityHurtSweetBerry,

    FoxChewingParticles,

    LivingEntityTeleportParticles,

    LivingEntityMainHandBreak,

    LivingEntityOffhandBreak,

    LivingEntityHeadBreak,

    LivingEntityChestBreak,

    LivingEntityLegsBreak,

    LivingEntityFeetBreak,
}

impl Into<u8> for EntityStatus {
    fn into(self) -> u8 {
        match self {
            EntityStatus::TippedArrowParticles => 0,
            EntityStatus::RabbitRotatedJumping => 1,
            EntityStatus::MinecartSpawnerResetDelay => 1,
            EntityStatus::HurtAnimation => 2,
            EntityStatus::DeathSoundAnimation => 3,
            EntityStatus::SnowballPoof => 3,
            EntityStatus::EggBreaking => 3,
            EntityStatus::IronGolemAttachSoundAnimation => 4,
            EntityStatus::EvokerFangsAttackAnimation => 4,
            EntityStatus::RavagerAttackAnimation => 4,
            EntityStatus::TamableTamingFailed => 6,
            EntityStatus::TamableTamingSucceeded => 7,
            EntityStatus::WolfShakeWaterAnimation => 8,
            EntityStatus::PlayerItemUseFinished => 9,
            EntityStatus::SheepGrassAnimation => 10,
            EntityStatus::MinecartTNTIgniteAnimation => 10,
            EntityStatus::IronGolemHoldPoppy => 11,
            EntityStatus::VillagerMatingComplete => 12,
            EntityStatus::VillagerAngry => 13,
            EntityStatus::VillagerHappy => 14,
            EntityStatus::WitchMagicParticles => 15,
            EntityStatus::ZombieVillagerCured => 16,
            EntityStatus::FireworkExplosion => 17,
            EntityStatus::AnimalLoveModeParticles => 18,
            EntityStatus::SquidResetRotation => 19,
            EntityStatus::MobExplosionParticle => 20,
            EntityStatus::GuardianAttackSound => 21,
            EntityStatus::PlayerReducedDebugInfoEnabled => 22,
            EntityStatus::PlayerReducedDebugInfoDisabled => 23,
            EntityStatus::PlayerOPLevel0 => 24,
            EntityStatus::PlayerOPLevel1 => 25,
            EntityStatus::PlayerOPLevel2 => 26,
            EntityStatus::PlayerOPLevel3 => 27,
            EntityStatus::PlayerOPLevel4 => 28,
            EntityStatus::LivingEntityShieldBlock => 29,
            EntityStatus::LivingEntityShieldBreak => 30,
            EntityStatus::FishingHookToPlayer => 31,
            EntityStatus::ArmorStandHit => 32,
            EntityStatus::LivingEntityThorns => 33,
            EntityStatus::IronGolemRemovePoppy => 34,
            EntityStatus::EntityTotemOfUndying => 35,
            EntityStatus::LivingEntityHurtAndDrown => 36,
            EntityStatus::LivingEntityHurtAndBurn => 37,
            EntityStatus::DolphinHappy => 38,
            EntityStatus::RavagerStunned => 39,
            EntityStatus::CatTamingFailed => 40,
            EntityStatus::CatTamingSucceeded => 41,
            EntityStatus::VillagerRaidSweat => 42,
            EntityStatus::PlayerBadOmenCloudEffect => 43,
            EntityStatus::LivingEntityHurtSweetBerry => 44,
            EntityStatus::FoxChewingParticles => 45,
            EntityStatus::LivingEntityTeleportParticles => 46,
            EntityStatus::LivingEntityMainHandBreak => 47,
            EntityStatus::LivingEntityOffhandBreak => 48,
            EntityStatus::LivingEntityHeadBreak => 49,
            EntityStatus::LivingEntityChestBreak => 50,
            EntityStatus::LivingEntityLegsBreak => 51,
            EntityStatus::LivingEntityFeetBreak => 52,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntityStatusPacket {
    pub entity_id: i32,
    pub entity_status: EntityStatus,
}

impl Clientbound for EntityStatusPacket {
    fn writer(&self) -> PacketWriter {
        let mut writer = PacketWriter::new(0x1A);

        writer.add_signed_int(self.entity_id);
        writer.add_unsigned_byte(self.entity_status.into());

        writer
    }
}
