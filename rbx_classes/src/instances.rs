use super::enums;
use core::ops::{Deref, DerefMut};
use rbx_types::*;
macro_rules! impl_inherits {
    ($ class : path , $ inherits : path) => {
        impl Deref for $class {
            type Target = $inherits;
            fn deref(&self) -> &$inherits {
                &self.superclass
            }
        }
        impl DerefMut for $class {
            fn deref_mut(&mut self) -> &mut $inherits {
                &mut self.superclass
            }
        }
    };
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Accessory {
    #[doc(hidden)]
    pub superclass: Accoutrement,
    pub AccessoryType: enums::AccessoryType,
}
impl_inherits!(Accessory, Accoutrement);
impl Default for Accessory {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Accoutrement {
            superclass,
            AttachmentPoint: CFrame::identity(),
        };
        Self {
            superclass,
            AccessoryType: enums::AccessoryType::Unknown,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AccessoryDescription {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AccessoryType: enums::AccessoryType,
    pub AssetId: i64,
    pub Instance: Ref,
    pub IsLayered: bool,
    pub Order: i32,
    pub Position: Vector3,
    pub Puffiness: f32,
    pub Rotation: Vector3,
    pub Scale: Vector3,
}
impl_inherits!(AccessoryDescription, Instance);
impl Default for AccessoryDescription {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AccessoryType: enums::AccessoryType::Unknown,
            AssetId: 0i64,
            Instance: Ref::none(),
            IsLayered: false,
            Order: 0i32,
            Position: Vector3::new(0f32, 0f32, 0f32),
            Puffiness: 1f32,
            Rotation: Vector3::new(0f32, 0f32, 0f32),
            Scale: Vector3::new(1f32, 1f32, 1f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AccountService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AccountService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Accoutrement {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AttachmentPoint: CFrame,
}
impl_inherits!(Accoutrement, Instance);
impl Default for Accoutrement {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AttachmentPoint: CFrame::identity(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AchievementService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AchievementService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ActivityHistoryEventService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ActivityHistoryEventService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Actor {
    #[doc(hidden)]
    pub superclass: Model,
}
impl_inherits!(Actor, Model);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AdGui {
    #[doc(hidden)]
    pub superclass: SurfaceGuiBase,
    pub AdShape: enums::AdShape,
    pub EnableVideoAds: bool,
    pub FallbackImage: ContentId,
}
impl_inherits!(AdGui, SurfaceGuiBase);
impl Default for AdGui {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase2d {
            superclass,
            AutoLocalize: false,
            RootLocalizationTable: Ref::none(),
            SelectionBehaviorDown: enums::SelectionBehavior::Escape,
            SelectionBehaviorLeft: enums::SelectionBehavior::Escape,
            SelectionBehaviorRight: enums::SelectionBehavior::Escape,
            SelectionBehaviorUp: enums::SelectionBehavior::Escape,
            SelectionGroup: false,
        };
        let superclass = LayerCollector {
            superclass,
            Enabled: false,
            ResetOnSpawn: false,
            ZIndexBehavior: enums::ZIndexBehavior::Global,
        };
        let superclass = SurfaceGuiBase {
            superclass,
            Active: false,
            Adornee: Ref::none(),
            Face: enums::NormalId::Right,
        };
        Self {
            superclass,
            AdShape: enums::AdShape::HorizontalRectangle,
            EnableVideoAds: false,
            FallbackImage: "".into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AdPortal {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AdPortal, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AdService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AdService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AdvancedDragger {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AdvancedDragger, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AirController {
    #[doc(hidden)]
    pub superclass: ControllerBase,
    pub BalanceMaxTorque: f32,
    pub BalanceSpeed: f32,
    pub MaintainAngularMomentum: bool,
    pub MaintainLinearMomentum: bool,
    pub MoveMaxForce: f32,
    pub TurnMaxTorque: f32,
    pub TurnSpeedFactor: f32,
}
impl_inherits!(AirController, ControllerBase);
impl Default for AirController {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ControllerBase {
            superclass,
            BalanceRigidityEnabled: false,
            MoveSpeedFactor: 1f32,
        };
        Self {
            superclass,
            BalanceMaxTorque: 10000f32,
            BalanceSpeed: 100f32,
            MaintainAngularMomentum: true,
            MaintainLinearMomentum: true,
            MoveMaxForce: 1000f32,
            TurnMaxTorque: 10000f32,
            TurnSpeedFactor: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AlignOrientation {
    #[doc(hidden)]
    pub superclass: Constraint,
    pub AlignType: enums::AlignType,
    pub CFrame: CFrame,
    pub MaxAngularVelocity: f32,
    pub MaxTorque: f32,
    pub Mode: enums::OrientationAlignmentMode,
    pub PrimaryAxisOnly: bool,
    pub ReactionTorqueEnabled: bool,
    pub Responsiveness: f32,
    pub RigidityEnabled: bool,
}
impl_inherits!(AlignOrientation, Constraint);
impl Default for AlignOrientation {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Constraint {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            Color: BrickColor::from_number(23u16).unwrap(),
            Enabled: true,
            Visible: false,
        };
        Self {
            superclass,
            AlignType: enums::AlignType::AllAxes,
            CFrame: CFrame::identity(),
            MaxAngularVelocity: f32::INFINITY,
            MaxTorque: 10000f32,
            Mode: enums::OrientationAlignmentMode::TwoAttachment,
            PrimaryAxisOnly: false,
            ReactionTorqueEnabled: false,
            Responsiveness: 10f32,
            RigidityEnabled: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AlignPosition {
    #[doc(hidden)]
    pub superclass: Constraint,
    pub ApplyAtCenterOfMass: bool,
    pub ForceLimitMode: enums::ForceLimitMode,
    pub ForceRelativeTo: enums::ActuatorRelativeTo,
    pub MaxAxesForce: Vector3,
    pub MaxForce: f32,
    pub MaxVelocity: f32,
    pub Mode: enums::PositionAlignmentMode,
    pub Position: Vector3,
    pub ReactionForceEnabled: bool,
    pub Responsiveness: f32,
    pub RigidityEnabled: bool,
}
impl_inherits!(AlignPosition, Constraint);
impl Default for AlignPosition {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Constraint {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            Color: BrickColor::from_number(194u16).unwrap(),
            Enabled: true,
            Visible: false,
        };
        Self {
            superclass,
            ApplyAtCenterOfMass: false,
            ForceLimitMode: enums::ForceLimitMode::Magnitude,
            ForceRelativeTo: enums::ActuatorRelativeTo::World,
            MaxAxesForce: Vector3::new(10000f32, 10000f32, 10000f32),
            MaxForce: 10000f32,
            MaxVelocity: f32::INFINITY,
            Mode: enums::PositionAlignmentMode::TwoAttachment,
            Position: Vector3::new(0f32, 0f32, 0f32),
            ReactionForceEnabled: false,
            Responsiveness: 10f32,
            RigidityEnabled: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnalyticsService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ApiKey: String,
}
impl_inherits!(AnalyticsService, Instance);
impl Default for AnalyticsService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ApiKey: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AngularVelocity {
    #[doc(hidden)]
    pub superclass: Constraint,
    pub AngularVelocity: Vector3,
    pub MaxTorque: f32,
    pub ReactionTorqueEnabled: bool,
    pub RelativeTo: enums::ActuatorRelativeTo,
}
impl_inherits!(AngularVelocity, Constraint);
impl Default for AngularVelocity {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Constraint {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            Color: BrickColor::from_number(23u16).unwrap(),
            Enabled: true,
            Visible: false,
        };
        Self {
            superclass,
            AngularVelocity: Vector3::new(0f32, 0f32, 0f32),
            MaxTorque: 0f32,
            ReactionTorqueEnabled: false,
            RelativeTo: enums::ActuatorRelativeTo::World,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Animation {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AnimationId: ContentId,
}
impl_inherits!(Animation, Instance);
impl Default for Animation {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AnimationId: "".into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationClip {
    #[doc(hidden)]
    pub superclass: Instance,
    pub GuidBinaryString: BinaryString,
    pub Loop: bool,
    pub Priority: enums::AnimationPriority,
}
impl_inherits!(AnimationClip, Instance);
impl Default for AnimationClip {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            GuidBinaryString: b"".as_slice().into(),
            Loop: false,
            Priority: enums::AnimationPriority::Idle,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnimationClipProvider {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AnimationClipProvider, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationConstraint {
    #[doc(hidden)]
    pub superclass: Constraint,
    pub IsKinematic: bool,
    pub MaxForce: f32,
    pub MaxTorque: f32,
    pub Transform: CFrame,
}
impl_inherits!(AnimationConstraint, Constraint);
impl Default for AnimationConstraint {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Constraint {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            Color: BrickColor::from_number(23u16).unwrap(),
            Enabled: true,
            Visible: false,
        };
        Self {
            superclass,
            IsKinematic: false,
            MaxForce: 10000f32,
            MaxTorque: 10000f32,
            Transform: CFrame::identity(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnimationController {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AnimationController, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnimationFromVideoCreatorService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AnimationFromVideoCreatorService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnimationFromVideoCreatorStudioService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AnimationFromVideoCreatorStudioService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnimationGraphDefinition {
    #[doc(hidden)]
    pub superclass: AnimationClip,
}
impl_inherits!(AnimationGraphDefinition, AnimationClip);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnimationImportData {
    #[doc(hidden)]
    pub superclass: BaseImportData,
}
impl_inherits!(AnimationImportData, BaseImportData);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnimationNode {
    #[doc(hidden)]
    pub superclass: Object,
}
impl_inherits!(AnimationNode, Object);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationNodeDefinition {
    #[doc(hidden)]
    pub superclass: Instance,
    pub InputPinData: BinaryString,
    pub NodeType: enums::AnimationNodeType,
}
impl_inherits!(AnimationNodeDefinition, Instance);
impl Default for AnimationNodeDefinition {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            InputPinData: b"\x01\0\0\0\0\0\0\0".as_slice().into(),
            NodeType: enums::AnimationNodeType::InvalidNode,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationRigData {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Label: BinaryString,
    pub Name: BinaryString,
    pub Parent: BinaryString,
    pub PostTransform: BinaryString,
    pub PreTransform: BinaryString,
    pub Transform: BinaryString,
}
impl_inherits!(AnimationRigData, Instance);
impl Default for AnimationRigData {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self { superclass , Label : b"\x01\0\0\0\x01\0\0\0\0\0\0\0" . as_slice () . into () , Name : b"\x01\0\0\0\x01\0\0\0\0\0\0\0" . as_slice () . into () , Parent : b"\x01\0\0\0\x01\0\0\0\0\0" . as_slice () . into () , PostTransform : b"\x01\0\0\0\x01\0\0\0\0\0\x80?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80?\0\0\0\0\0\0\0\0\0\0\0\0" . as_slice () . into () , PreTransform : b"\x01\0\0\0\x01\0\0\0\0\0\x80?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80?\0\0\0\0\0\0\0\0\0\0\0\0" . as_slice () . into () , Transform : b"\x01\0\0\0\x01\0\0\0\0\0\x80?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80?\0\0\0\0\0\0\0\0\0\0\0\0" . as_slice () . into () }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnimationStreamTrack {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AnimationStreamTrack, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationTrack {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Priority: enums::AnimationPriority,
}
impl_inherits!(AnimationTrack, Instance);
impl Default for AnimationTrack {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Priority: enums::AnimationPriority::Idle,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Animator {
    #[doc(hidden)]
    pub superclass: Instance,
    pub PreferLodEnabled: bool,
}
impl_inherits!(Animator, Instance);
impl Default for Animator {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            PreferLodEnabled: true,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Annotation {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(Annotation, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnnotationsService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AnnotationsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AppLifecycleObserverService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AppLifecycleObserverService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AppRatingPromptService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AppRatingPromptService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AppStorageService {
    #[doc(hidden)]
    pub superclass: LocalStorageService,
}
impl_inherits!(AppStorageService, LocalStorageService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AppUpdateService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AppUpdateService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ArcHandles {
    #[doc(hidden)]
    pub superclass: HandlesBase,
    pub Axes: Axes,
}
impl_inherits!(ArcHandles, HandlesBase);
impl Default for ArcHandles {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase3d {
            superclass,
            Color3: Color3::new(0.050980393f32, 0.4117647f32, 0.6745098f32),
            Transparency: 0f32,
            Visible: true,
        };
        let superclass = PartAdornment {
            superclass,
            Adornee: Ref::none(),
        };
        let superclass = HandlesBase { superclass };
        Self {
            superclass,
            Axes: Axes::all(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AssetCounterService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AssetCounterService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetDeliveryProxy {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Interface: String,
    pub Port: i32,
    pub StartServer: bool,
}
impl_inherits!(AssetDeliveryProxy, Instance);
impl Default for AssetDeliveryProxy {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Interface: "".to_owned(),
            Port: 0i32,
            StartServer: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AssetImportService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AssetImportService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AssetImportSession {
    #[doc(hidden)]
    pub superclass: ImportSession,
}
impl_inherits!(AssetImportSession, ImportSession);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AssetManagerService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AssetManagerService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetPatchSettings {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ContentId: String,
    pub OutputPath: String,
    pub PatchId: String,
}
impl_inherits!(AssetPatchSettings, Instance);
impl Default for AssetPatchSettings {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ContentId: "".to_owned(),
            OutputPath: "".to_owned(),
            PatchId: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AllowInsertFreeAssets: bool,
}
impl_inherits!(AssetService, Instance);
impl Default for AssetService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AllowInsertFreeAssets: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AssetSoundEffect {
    #[doc(hidden)]
    pub superclass: CustomSoundEffect,
}
impl_inherits!(AssetSoundEffect, CustomSoundEffect);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Atmosphere {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Color: Color3,
    pub Decay: Color3,
    pub Density: f32,
    pub Glare: f32,
    pub Haze: f32,
    pub Offset: f32,
}
impl_inherits!(Atmosphere, Instance);
impl Default for Atmosphere {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Color: Color3::new(0.7843f32, 0.6667f32, 0.4235f32),
            Decay: Color3::new(0.3608f32, 0.2353f32, 0.0549f32),
            Density: 0.395f32,
            Glare: 0f32,
            Haze: 0f32,
            Offset: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AtmosphereSensor {
    #[doc(hidden)]
    pub superclass: SensorBase,
}
impl_inherits!(AtmosphereSensor, SensorBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Attachment {
    #[doc(hidden)]
    pub superclass: Instance,
    pub CFrame: CFrame,
    pub Visible: bool,
}
impl_inherits!(Attachment, Instance);
impl Default for Attachment {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            CFrame: CFrame::identity(),
            Visible: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioAnalyzer {
    #[doc(hidden)]
    pub superclass: Instance,
    pub SpectrumEnabled: bool,
    pub WindowSize: enums::AudioWindowSize,
}
impl_inherits!(AudioAnalyzer, Instance);
impl Default for AudioAnalyzer {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            SpectrumEnabled: true,
            WindowSize: enums::AudioWindowSize::Medium,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioChannelMixer {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Layout: enums::AudioChannelLayout,
}
impl_inherits!(AudioChannelMixer, Instance);
impl Default for AudioChannelMixer {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Layout: enums::AudioChannelLayout::Stereo,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioChannelSplitter {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Layout: enums::AudioChannelLayout,
}
impl_inherits!(AudioChannelSplitter, Instance);
impl Default for AudioChannelSplitter {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Layout: enums::AudioChannelLayout::Stereo,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioChorus {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Bypass: bool,
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
impl_inherits!(AudioChorus, Instance);
impl Default for AudioChorus {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Bypass: false,
            Depth: 0.45f32,
            Mix: 0.85f32,
            Rate: 5f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioCompressor {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Attack: f32,
    pub Bypass: bool,
    pub MakeupGain: f32,
    pub Ratio: f32,
    pub Release: f32,
    pub Threshold: f32,
}
impl_inherits!(AudioCompressor, Instance);
impl Default for AudioCompressor {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Attack: 0.1f32,
            Bypass: false,
            MakeupGain: 0f32,
            Ratio: 40f32,
            Release: 0.1f32,
            Threshold: -40f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioDeviceInput {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AccessType: enums::AccessModifierType,
    pub Active: bool,
    pub Muted: bool,
    pub Player: Ref,
    pub Volume: f32,
}
impl_inherits!(AudioDeviceInput, Instance);
impl Default for AudioDeviceInput {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AccessType: enums::AccessModifierType::Deny,
            Active: true,
            Muted: false,
            Player: Ref::none(),
            Volume: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioDeviceOutput {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Player: Ref,
}
impl_inherits!(AudioDeviceOutput, Instance);
impl Default for AudioDeviceOutput {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Player: Ref::none(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioDistortion {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Bypass: bool,
    pub Level: f32,
}
impl_inherits!(AudioDistortion, Instance);
impl Default for AudioDistortion {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Bypass: false,
            Level: 0.5f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioEcho {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Bypass: bool,
    pub DelayTime: f32,
    pub DryLevel: f32,
    pub Feedback: f32,
    pub RampTime: f32,
    pub WetLevel: f32,
}
impl_inherits!(AudioEcho, Instance);
impl Default for AudioEcho {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Bypass: false,
            DelayTime: 1f32,
            DryLevel: 0f32,
            Feedback: 0.5f32,
            RampTime: 0f32,
            WetLevel: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioEmitter {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AcousticSimulationEnabled: bool,
    pub AngleAttenuation: BinaryString,
    pub AudioInteractionGroup: String,
    pub DistanceAttenuation: BinaryString,
    pub PositionOverride: Ref,
    pub SimulationFidelity: enums::AudioSimulationFidelity,
}
impl_inherits!(AudioEmitter, Instance);
impl Default for AudioEmitter {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AcousticSimulationEnabled: true,
            AngleAttenuation: b"\0".as_slice().into(),
            AudioInteractionGroup: "".to_owned(),
            DistanceAttenuation: b"\0".as_slice().into(),
            PositionOverride: Ref::none(),
            SimulationFidelity: enums::AudioSimulationFidelity::Automatic,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioEqualizer {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Bypass: bool,
    pub HighGain: f32,
    pub LowGain: f32,
    pub MidGain: f32,
    pub MidRange: NumberRange,
}
impl_inherits!(AudioEqualizer, Instance);
impl Default for AudioEqualizer {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Bypass: false,
            HighGain: 0f32,
            LowGain: 0f32,
            MidGain: 0f32,
            MidRange: NumberRange::new(400f32, 4000f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioFader {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Bypass: bool,
    pub Volume: f32,
}
impl_inherits!(AudioFader, Instance);
impl Default for AudioFader {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Bypass: false,
            Volume: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioFilter {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Bypass: bool,
    pub FilterType: enums::AudioFilterType,
    pub Frequency: f32,
    pub Gain: f32,
    pub Q: f32,
}
impl_inherits!(AudioFilter, Instance);
impl Default for AudioFilter {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Bypass: false,
            FilterType: enums::AudioFilterType::Peak,
            Frequency: 2000f32,
            Gain: 0f32,
            Q: 0.707f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioFlanger {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Bypass: bool,
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
impl_inherits!(AudioFlanger, Instance);
impl Default for AudioFlanger {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Bypass: false,
            Depth: 0.45f32,
            Mix: 0.85f32,
            Rate: 5f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AudioFocusService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AudioFocusService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioGate {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Attack: f32,
    pub Bypass: bool,
    pub Release: f32,
    pub Threshold: NumberRange,
}
impl_inherits!(AudioGate, Instance);
impl Default for AudioGate {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Attack: 0.01f32,
            Bypass: false,
            Release: 0.1f32,
            Threshold: NumberRange::new(-36f32, -24f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioLimiter {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Bypass: bool,
    pub MaxLevel: f32,
    pub Release: f32,
}
impl_inherits!(AudioLimiter, Instance);
impl Default for AudioLimiter {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Bypass: false,
            MaxLevel: 0f32,
            Release: 0.01f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioListener {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AcousticSimulationEnabled: bool,
    pub AngleAttenuation: BinaryString,
    pub AudioInteractionGroup: String,
    pub DistanceAttenuation: BinaryString,
    pub PositionOverride: Ref,
    pub SimulationFidelity: enums::AudioSimulationFidelity,
}
impl_inherits!(AudioListener, Instance);
impl Default for AudioListener {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AcousticSimulationEnabled: true,
            AngleAttenuation: b"\0".as_slice().into(),
            AudioInteractionGroup: "".to_owned(),
            DistanceAttenuation: b"\0".as_slice().into(),
            PositionOverride: Ref::none(),
            SimulationFidelity: enums::AudioSimulationFidelity::Automatic,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AudioPages {
    #[doc(hidden)]
    pub superclass: Pages,
}
impl_inherits!(AudioPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioPitchShifter {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Bypass: bool,
    pub Pitch: f32,
    pub WindowSize: enums::AudioWindowSize,
}
impl_inherits!(AudioPitchShifter, Instance);
impl Default for AudioPitchShifter {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Bypass: false,
            Pitch: 1.25f32,
            WindowSize: enums::AudioWindowSize::Medium,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioPlayer {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AudioContent: Content,
    pub AutoLoad: bool,
    pub AutoPlay: bool,
    pub IsMutedForCapture: bool,
    pub LoopRegion: NumberRange,
    pub Looping: bool,
    pub PlaybackRegion: NumberRange,
    pub PlaybackSpeed: f64,
    pub TimePosition: f64,
    pub Volume: f32,
}
impl_inherits!(AudioPlayer, Instance);
impl Default for AudioPlayer {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AudioContent: Content::none(),
            AutoLoad: true,
            AutoPlay: false,
            IsMutedForCapture: false,
            LoopRegion: NumberRange::new(0f32, 60000f32),
            Looping: false,
            PlaybackRegion: NumberRange::new(0f32, 60000f32),
            PlaybackSpeed: 1f64,
            TimePosition: 0f64,
            Volume: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioRecorder {
    #[doc(hidden)]
    pub superclass: Instance,
    pub IsRecording: bool,
}
impl_inherits!(AudioRecorder, Instance);
impl Default for AudioRecorder {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            IsRecording: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioReverb {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Bypass: bool,
    pub DecayRatio: f32,
    pub DecayTime: f32,
    pub Density: f32,
    pub Diffusion: f32,
    pub DryLevel: f32,
    pub EarlyDelayTime: f32,
    pub HighCutFrequency: f32,
    pub LateDelayTime: f32,
    pub LowShelfFrequency: f32,
    pub LowShelfGain: f32,
    pub ReferenceFrequency: f32,
    pub WetLevel: f32,
}
impl_inherits!(AudioReverb, Instance);
impl Default for AudioReverb {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Bypass: false,
            DecayRatio: 0.5f32,
            DecayTime: 1.5f32,
            Density: 1f32,
            Diffusion: 1f32,
            DryLevel: 0f32,
            EarlyDelayTime: 0.02f32,
            HighCutFrequency: 20000f32,
            LateDelayTime: 0.04f32,
            LowShelfFrequency: 250f32,
            LowShelfGain: 0f32,
            ReferenceFrequency: 5000f32,
            WetLevel: -6f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioSearchParams {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Album: String,
    pub Artist: String,
    pub AudioSubType: enums::AudioSubType,
    pub MaxDuration: i32,
    pub MinDuration: i32,
    pub SearchKeyword: String,
    pub Tag: String,
    pub Title: String,
}
impl_inherits!(AudioSearchParams, Instance);
impl Default for AudioSearchParams {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Album: "".to_owned(),
            Artist: "".to_owned(),
            AudioSubType: enums::AudioSubType::Music,
            MaxDuration: 0i32,
            MinDuration: 0i32,
            SearchKeyword: "".to_owned(),
            Tag: "".to_owned(),
            Title: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioSpeechToText {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Enabled: bool,
    pub Text: String,
}
impl_inherits!(AudioSpeechToText, Instance);
impl Default for AudioSpeechToText {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Enabled: false,
            Text: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioTextToSpeech {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Looping: bool,
    pub Pitch: f32,
    pub PlaybackSpeed: f32,
    pub Speed: f32,
    pub Text: String,
    pub TimePosition: f64,
    pub VoiceId: String,
    pub Volume: f32,
}
impl_inherits!(AudioTextToSpeech, Instance);
impl Default for AudioTextToSpeech {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Looping: false,
            Pitch: 0f32,
            PlaybackSpeed: 1f32,
            Speed: 1f32,
            Text: "".to_owned(),
            TimePosition: 0f64,
            VoiceId: "".to_owned(),
            Volume: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioTremolo {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Bypass: bool,
    pub Depth: f32,
    pub Duty: f32,
    pub Frequency: f32,
    pub Shape: f32,
    pub Skew: f32,
    pub Square: f32,
}
impl_inherits!(AudioTremolo, Instance);
impl Default for AudioTremolo {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Bypass: false,
            Depth: 1f32,
            Duty: 0.5f32,
            Frequency: 5f32,
            Shape: 0f32,
            Skew: 0f32,
            Square: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AuroraScript {
    #[doc(hidden)]
    pub superclass: LuaSourceContainer,
    pub AuroraScriptBindingsSerialize: BinaryString,
    pub EnableCulling: bool,
    pub EnableLod: bool,
    pub LodCriticality: i32,
    pub Priority: i32,
    pub Source: String,
}
impl_inherits!(AuroraScript, LuaSourceContainer);
impl Default for AuroraScript {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = LuaSourceContainer {
            superclass,
            ScriptGuid: "".to_owned(),
        };
        Self {
            superclass,
            AuroraScriptBindingsSerialize: b"".as_slice().into(),
            EnableCulling: false,
            EnableLod: false,
            LodCriticality: 0i32,
            Priority: 0i32,
            Source: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AuroraScriptObject {
    #[doc(hidden)]
    pub superclass: Instance,
    pub BehaviorWeak: Ref,
    pub BoundInstanceWeak: Ref,
    pub FrameId: i32,
    pub LodLevel: i32,
    pub PriorFrameInvoked: i32,
}
impl_inherits!(AuroraScriptObject, Instance);
impl Default for AuroraScriptObject {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            BehaviorWeak: Ref::none(),
            BoundInstanceWeak: Ref::none(),
            FrameId: 0i32,
            LodLevel: 0i32,
            PriorFrameInvoked: 0i32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AuroraScriptService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AuroraScriptService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AuroraService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub HashRoundingPoint: f64,
    pub IgnoreRotation: bool,
    pub LockStepIdOffset: bool,
    pub RollbackOffset: i32,
}
impl_inherits!(AuroraService, Instance);
impl Default for AuroraService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            HashRoundingPoint: 0f64,
            IgnoreRotation: false,
            LockStepIdOffset: false,
            RollbackOffset: 0i32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarAccessoryRules {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AccessoryMode: enums::AvatarSettingsAccessoryMode,
    pub CustomAccessoryMode: enums::AvatarSettingsCustomAccessoryMode,
    pub CustomBackAccessoryEnabled: bool,
    pub CustomBackAccessoryId: i64,
    pub CustomFaceAccessoryEnabled: bool,
    pub CustomFaceAccessoryId: i64,
    pub CustomFrontAccessoryEnabled: bool,
    pub CustomFrontAccessoryId: i64,
    pub CustomHairAccessoryEnabled: bool,
    pub CustomHairAccessoryId: i64,
    pub CustomHeadAccessoryEnabled: bool,
    pub CustomHeadAccessoryId: i64,
    pub CustomNeckAccessoryEnabled: bool,
    pub CustomNeckAccessoryId: i64,
    pub CustomShoulderAccessoryEnabled: bool,
    pub CustomShoulderAccessoryId: i64,
    pub CustomWaistAccessoryEnabled: bool,
    pub CustomWaistAccessoryId: i64,
    pub EnableSound: bool,
    pub EnableVfx: bool,
    pub LimitBounds: Vector3,
    pub LimitMethod: enums::AvatarSettingsAccessoryLimitMethod,
}
impl_inherits!(AvatarAccessoryRules, Instance);
impl Default for AvatarAccessoryRules {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AccessoryMode: enums::AvatarSettingsAccessoryMode::PlayerChoice,
            CustomAccessoryMode: enums::AvatarSettingsCustomAccessoryMode::PlayerChoice,
            CustomBackAccessoryEnabled: false,
            CustomBackAccessoryId: 0i64,
            CustomFaceAccessoryEnabled: false,
            CustomFaceAccessoryId: 0i64,
            CustomFrontAccessoryEnabled: false,
            CustomFrontAccessoryId: 0i64,
            CustomHairAccessoryEnabled: false,
            CustomHairAccessoryId: 0i64,
            CustomHeadAccessoryEnabled: false,
            CustomHeadAccessoryId: 0i64,
            CustomNeckAccessoryEnabled: false,
            CustomNeckAccessoryId: 0i64,
            CustomShoulderAccessoryEnabled: false,
            CustomShoulderAccessoryId: 0i64,
            CustomWaistAccessoryEnabled: false,
            CustomWaistAccessoryId: 0i64,
            EnableSound: true,
            EnableVfx: true,
            LimitBounds: Vector3::new(0f32, 0f32, 0f32),
            LimitMethod: enums::AvatarSettingsAccessoryLimitMethod::Remove,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarAnimationRules {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AnimationClipsMode: enums::AvatarSettingsAnimationClipsMode,
    pub AnimationPacksMode: enums::AvatarSettingsAnimationPacksMode,
    pub CustomClimbAnimationEnabled: bool,
    pub CustomClimbAnimationId: i64,
    pub CustomFallAnimationEnabled: bool,
    pub CustomFallAnimationId: i64,
    pub CustomIdleAlt1AnimationEnabled: bool,
    pub CustomIdleAlt1AnimationId: i64,
    pub CustomIdleAlt2AnimationEnabled: bool,
    pub CustomIdleAlt2AnimationId: i64,
    pub CustomIdleAnimationEnabled: bool,
    pub CustomIdleAnimationId: i64,
    pub CustomJumpAnimationEnabled: bool,
    pub CustomJumpAnimationId: i64,
    pub CustomRunAnimationEnabled: bool,
    pub CustomRunAnimationId: i64,
    pub CustomSwimAnimationEnabled: bool,
    pub CustomSwimAnimationId: i64,
    pub CustomSwimIdleAnimationEnabled: bool,
    pub CustomSwimIdleAnimationId: i64,
    pub CustomWalkAnimationEnabled: bool,
    pub CustomWalkAnimationId: i64,
}
impl_inherits!(AvatarAnimationRules, Instance);
impl Default for AvatarAnimationRules {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AnimationClipsMode: enums::AvatarSettingsAnimationClipsMode::PlayerChoice,
            AnimationPacksMode: enums::AvatarSettingsAnimationPacksMode::PlayerChoice,
            CustomClimbAnimationEnabled: false,
            CustomClimbAnimationId: 0i64,
            CustomFallAnimationEnabled: false,
            CustomFallAnimationId: 0i64,
            CustomIdleAlt1AnimationEnabled: false,
            CustomIdleAlt1AnimationId: 0i64,
            CustomIdleAlt2AnimationEnabled: false,
            CustomIdleAlt2AnimationId: 0i64,
            CustomIdleAnimationEnabled: false,
            CustomIdleAnimationId: 0i64,
            CustomJumpAnimationEnabled: false,
            CustomJumpAnimationId: 0i64,
            CustomRunAnimationEnabled: false,
            CustomRunAnimationId: 0i64,
            CustomSwimAnimationEnabled: false,
            CustomSwimAnimationId: 0i64,
            CustomSwimIdleAnimationEnabled: false,
            CustomSwimIdleAnimationId: 0i64,
            CustomWalkAnimationEnabled: false,
            CustomWalkAnimationId: 0i64,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarBodyRules {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AppearanceMode: enums::AvatarSettingsAppearanceMode,
    pub BuildMode: enums::AvatarSettingsBuildMode,
    pub CustomBodyBundleId: i64,
    pub CustomBodyType: enums::AvatarSettingsCustomBodyType,
    pub CustomBodyTypeScale: NumberRange,
    pub CustomEyebrowEnabled: bool,
    pub CustomEyebrowId: i64,
    pub CustomEyelashEnabled: bool,
    pub CustomEyelashId: i64,
    pub CustomFaceEnabled: bool,
    pub CustomFaceId: i64,
    pub CustomHeadEnabled: bool,
    pub CustomHeadId: i64,
    pub CustomHeadScale: NumberRange,
    pub CustomHeight: NumberRange,
    pub CustomHeightScale: NumberRange,
    pub CustomLeftArmEnabled: bool,
    pub CustomLeftArmId: i64,
    pub CustomLeftLegEnabled: bool,
    pub CustomLeftLegId: i64,
    pub CustomMoodEnabled: bool,
    pub CustomMoodId: i64,
    pub CustomProportionsScale: NumberRange,
    pub CustomRightArmEnabled: bool,
    pub CustomRightArmId: i64,
    pub CustomRightLegEnabled: bool,
    pub CustomRightLegId: i64,
    pub CustomTorsoEnabled: bool,
    pub CustomTorsoId: i64,
    pub CustomWidthScale: NumberRange,
    pub KeepPlayerHead: bool,
    pub ScaleMode: enums::AvatarSettingsScaleMode,
}
impl_inherits!(AvatarBodyRules, Instance);
impl Default for AvatarBodyRules {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AppearanceMode: enums::AvatarSettingsAppearanceMode::PlayerChoice,
            BuildMode: enums::AvatarSettingsBuildMode::PlayerChoice,
            CustomBodyBundleId: 0i64,
            CustomBodyType: enums::AvatarSettingsCustomBodyType::AvatarReference,
            CustomBodyTypeScale: NumberRange::new(0f32, 1f32),
            CustomEyebrowEnabled: false,
            CustomEyebrowId: 0i64,
            CustomEyelashEnabled: false,
            CustomEyelashId: 0i64,
            CustomFaceEnabled: false,
            CustomFaceId: 0i64,
            CustomHeadEnabled: false,
            CustomHeadId: 0i64,
            CustomHeadScale: NumberRange::new(0.95f32, 1f32),
            CustomHeight: NumberRange::new(5.5f32, 5.5f32),
            CustomHeightScale: NumberRange::new(0.9f32, 1.05f32),
            CustomLeftArmEnabled: false,
            CustomLeftArmId: 0i64,
            CustomLeftLegEnabled: false,
            CustomLeftLegId: 0i64,
            CustomMoodEnabled: false,
            CustomMoodId: 0i64,
            CustomProportionsScale: NumberRange::new(0f32, 1f32),
            CustomRightArmEnabled: false,
            CustomRightArmId: 0i64,
            CustomRightLegEnabled: false,
            CustomRightLegId: 0i64,
            CustomTorsoEnabled: false,
            CustomTorsoId: 0i64,
            CustomWidthScale: NumberRange::new(0.7f32, 1f32),
            KeepPlayerHead: true,
            ScaleMode: enums::AvatarSettingsScaleMode::PlayerChoice,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AvatarChatService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AvatarChatService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarClothingRules {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ClothingMode: enums::AvatarSettingsClothingMode,
    pub CustomClassicPantsAccessoryEnabled: bool,
    pub CustomClassicPantsAccessoryId: i64,
    pub CustomClassicShirtsAccessoryEnabled: bool,
    pub CustomClassicShirtsAccessoryId: i64,
    pub CustomClassicTShirtsAccessoryEnabled: bool,
    pub CustomClassicTShirtsAccessoryId: i64,
    pub CustomClothingMode: enums::AvatarSettingsCustomClothingMode,
    pub CustomDressSkirtAccessoryEnabled: bool,
    pub CustomDressSkirtAccessoryId: i64,
    pub CustomJacketAccessoryEnabled: bool,
    pub CustomJacketAccessoryId: i64,
    pub CustomLeftShoesAccessoryEnabled: bool,
    pub CustomLeftShoesAccessoryId: i64,
    pub CustomPantsAccessoryEnabled: bool,
    pub CustomPantsAccessoryId: i64,
    pub CustomRightShoesAccessoryEnabled: bool,
    pub CustomRightShoesAccessoryId: i64,
    pub CustomShirtAccessoryEnabled: bool,
    pub CustomShirtAccessoryId: i64,
    pub CustomShortsAccessoryEnabled: bool,
    pub CustomShortsAccessoryId: i64,
    pub CustomSweaterAccessoryEnabled: bool,
    pub CustomSweaterAccessoryId: i64,
    pub CustomTShirtAccessoryEnabled: bool,
    pub CustomTShirtAccessoryId: i64,
    pub LimitBounds: Vector3,
}
impl_inherits!(AvatarClothingRules, Instance);
impl Default for AvatarClothingRules {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ClothingMode: enums::AvatarSettingsClothingMode::PlayerChoice,
            CustomClassicPantsAccessoryEnabled: false,
            CustomClassicPantsAccessoryId: 0i64,
            CustomClassicShirtsAccessoryEnabled: false,
            CustomClassicShirtsAccessoryId: 0i64,
            CustomClassicTShirtsAccessoryEnabled: false,
            CustomClassicTShirtsAccessoryId: 0i64,
            CustomClothingMode: enums::AvatarSettingsCustomClothingMode::PlayerChoice,
            CustomDressSkirtAccessoryEnabled: false,
            CustomDressSkirtAccessoryId: 0i64,
            CustomJacketAccessoryEnabled: false,
            CustomJacketAccessoryId: 0i64,
            CustomLeftShoesAccessoryEnabled: false,
            CustomLeftShoesAccessoryId: 0i64,
            CustomPantsAccessoryEnabled: false,
            CustomPantsAccessoryId: 0i64,
            CustomRightShoesAccessoryEnabled: false,
            CustomRightShoesAccessoryId: 0i64,
            CustomShirtAccessoryEnabled: false,
            CustomShirtAccessoryId: 0i64,
            CustomShortsAccessoryEnabled: false,
            CustomShortsAccessoryId: 0i64,
            CustomSweaterAccessoryEnabled: false,
            CustomSweaterAccessoryId: 0i64,
            CustomTShirtAccessoryEnabled: false,
            CustomTShirtAccessoryId: 0i64,
            LimitBounds: Vector3::new(0f32, 0f32, 0f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarCollisionRules {
    #[doc(hidden)]
    pub superclass: Instance,
    pub CollisionMode: enums::AvatarSettingsCollisionMode,
    pub HitAndTouchDetectionMode: enums::AvatarSettingsHitAndTouchDetectionMode,
    pub LegacyCollisionMode: enums::AvatarSettingsLegacyCollisionMode,
    pub SingleColliderSize: Vector3,
}
impl_inherits!(AvatarCollisionRules, Instance);
impl Default for AvatarCollisionRules {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            CollisionMode: enums::AvatarSettingsCollisionMode::Default,
            HitAndTouchDetectionMode: enums::AvatarSettingsHitAndTouchDetectionMode::UseParts,
            LegacyCollisionMode: enums::AvatarSettingsLegacyCollisionMode::InnerBoxColliders,
            SingleColliderSize: Vector3::new(2f32, 3f32, 1f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AvatarCreationService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AvatarCreationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AvatarEditorService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AvatarEditorService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AvatarImportService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AvatarImportService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarRules {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AvatarType: enums::GameAvatarType,
}
impl_inherits!(AvatarRules, Instance);
impl Default for AvatarRules {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AvatarType: enums::GameAvatarType::R15,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AvatarSettings {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(AvatarSettings, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Backpack {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(Backpack, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BackpackItem {
    #[doc(hidden)]
    pub superclass: Model,
    pub TextureContent: Content,
}
impl_inherits!(BackpackItem, Model);
impl Default for BackpackItem {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PVInstance { superclass };
        let superclass = Model {
            superclass,
            LevelOfDetail: enums::ModelLevelOfDetail::Automatic,
            ModelMeshCFrame: CFrame::identity(),
            ModelMeshData: SharedString::new(b"".to_vec()),
            ModelMeshSize: Vector3::new(0f32, 0f32, 0f32),
            ModelStreamingMode: enums::ModelStreamingMode::Default,
            NeedsPivotMigration: false,
            PrimaryPart: Ref::none(),
            SlimHash: SharedString::new(b"".to_vec()),
            WorldPivotData: None,
        };
        Self {
            superclass,
            TextureContent: Content::none(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BadgeService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(BadgeService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BallSocketConstraint {
    #[doc(hidden)]
    pub superclass: Constraint,
    pub LimitsEnabled: bool,
    pub MaxFrictionTorqueXml: f32,
    pub Radius: f32,
    pub Restitution: f32,
    pub TwistLimitsEnabled: bool,
    pub TwistLowerAngle: f32,
    pub TwistUpperAngle: f32,
    pub UpperAngle: f32,
}
impl_inherits!(BallSocketConstraint, Constraint);
impl Default for BallSocketConstraint {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Constraint {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            Color: BrickColor::from_number(1009u16).unwrap(),
            Enabled: true,
            Visible: false,
        };
        Self {
            superclass,
            LimitsEnabled: false,
            MaxFrictionTorqueXml: 0f32,
            Radius: 0.15f32,
            Restitution: 0f32,
            TwistLimitsEnabled: false,
            TwistLowerAngle: -45f32,
            TwistUpperAngle: 45f32,
            UpperAngle: 45f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BanHistoryPages {
    #[doc(hidden)]
    pub superclass: Pages,
}
impl_inherits!(BanHistoryPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BaseImportData {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ImportName: String,
    pub ShouldImport: bool,
}
impl_inherits!(BaseImportData, Instance);
impl Default for BaseImportData {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ImportName: "".to_owned(),
            ShouldImport: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BasePart {
    #[doc(hidden)]
    pub superclass: PVInstance,
    pub Anchored: bool,
    pub AudioCanCollide: bool,
    pub BackParamA: f32,
    pub BackParamB: f32,
    pub BackSurface: enums::SurfaceType,
    pub BackSurfaceInput: enums::InputType,
    pub BottomParamA: f32,
    pub BottomParamB: f32,
    pub BottomSurface: enums::SurfaceType,
    pub BottomSurfaceInput: enums::InputType,
    pub CFrame: CFrame,
    pub CanCollide: bool,
    pub CanQuery: bool,
    pub CanTouch: bool,
    pub CastShadow: bool,
    pub CollisionGroup: String,
    pub CollisionGroupId: i32,
    pub CustomPhysicalProperties: PhysicalProperties,
    pub EnableFluidForces: bool,
    pub FrontParamA: f32,
    pub FrontParamB: f32,
    pub FrontSurface: enums::SurfaceType,
    pub FrontSurfaceInput: enums::InputType,
    pub LeftParamA: f32,
    pub LeftParamB: f32,
    pub LeftSurface: enums::SurfaceType,
    pub LeftSurfaceInput: enums::InputType,
    pub Locked: bool,
    pub Massless: bool,
    pub Material: enums::Material,
    pub PivotOffset: CFrame,
    pub Reflectance: f32,
    pub RightParamA: f32,
    pub RightParamB: f32,
    pub RightSurface: enums::SurfaceType,
    pub RightSurfaceInput: enums::InputType,
    pub RootPriority: i32,
    pub RotVelocity: Vector3,
    pub TopParamA: f32,
    pub TopParamB: f32,
    pub TopSurface: enums::SurfaceType,
    pub TopSurfaceInput: enums::InputType,
    pub Transparency: f32,
    pub Velocity: Vector3,
}
impl_inherits!(BasePart, PVInstance);
impl Default for BasePart {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PVInstance { superclass };
        Self {
            superclass,
            Anchored: false,
            AudioCanCollide: false,
            BackParamA: 0f32,
            BackParamB: 0f32,
            BackSurface: enums::SurfaceType::Smooth,
            BackSurfaceInput: enums::InputType::NoInput,
            BottomParamA: 0f32,
            BottomParamB: 0f32,
            BottomSurface: enums::SurfaceType::Smooth,
            BottomSurfaceInput: enums::InputType::NoInput,
            CFrame: CFrame::identity(),
            CanCollide: false,
            CanQuery: false,
            CanTouch: false,
            CastShadow: false,
            CollisionGroup: "".to_owned(),
            CollisionGroupId: 0i32,
            CustomPhysicalProperties: PhysicalProperties::Default,
            EnableFluidForces: false,
            FrontParamA: 0f32,
            FrontParamB: 0f32,
            FrontSurface: enums::SurfaceType::Smooth,
            FrontSurfaceInput: enums::InputType::NoInput,
            LeftParamA: 0f32,
            LeftParamB: 0f32,
            LeftSurface: enums::SurfaceType::Smooth,
            LeftSurfaceInput: enums::InputType::NoInput,
            Locked: false,
            Massless: false,
            Material: enums::Material::Plastic,
            PivotOffset: CFrame::identity(),
            Reflectance: 0f32,
            RightParamA: 0f32,
            RightParamB: 0f32,
            RightSurface: enums::SurfaceType::Smooth,
            RightSurfaceInput: enums::InputType::NoInput,
            RootPriority: 0i32,
            RotVelocity: Vector3::new(0f32, 0f32, 0f32),
            TopParamA: 0f32,
            TopParamB: 0f32,
            TopSurface: enums::SurfaceType::Smooth,
            TopSurfaceInput: enums::InputType::NoInput,
            Transparency: 0f32,
            Velocity: Vector3::new(0f32, 0f32, 0f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BasePlayerGui {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(BasePlayerGui, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BaseRemoteEvent {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(BaseRemoteEvent, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BaseScript {
    #[doc(hidden)]
    pub superclass: LuaSourceContainer,
    pub Disabled: bool,
    pub LinkedSource: ContentId,
    pub RunContext: enums::RunContext,
}
impl_inherits!(BaseScript, LuaSourceContainer);
impl Default for BaseScript {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = LuaSourceContainer {
            superclass,
            ScriptGuid: "".to_owned(),
        };
        Self {
            superclass,
            Disabled: false,
            LinkedSource: "".into(),
            RunContext: enums::RunContext::Legacy,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BaseWrap {
    #[doc(hidden)]
    pub superclass: Instance,
    pub CageMeshContent: Content,
    pub CageOrigin: CFrame,
    pub HsrAssetId: ContentId,
    pub HsrData: SharedString,
    pub HsrMeshIdData: SharedString,
    pub ImportOrigin: CFrame,
    pub TemporaryCageMeshId: ContentId,
}
impl_inherits!(BaseWrap, Instance);
impl Default for BaseWrap {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            CageMeshContent: Content::none(),
            CageOrigin: CFrame::identity(),
            HsrAssetId: "".into(),
            HsrData: SharedString::new(b"".to_vec()),
            HsrMeshIdData: SharedString::new(b"".to_vec()),
            ImportOrigin: CFrame::identity(),
            TemporaryCageMeshId: "".into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Beam {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Attachment0: Ref,
    pub Attachment1: Ref,
    pub Brightness: f32,
    pub Color: ColorSequence,
    pub CurveSize0: f32,
    pub CurveSize1: f32,
    pub Enabled: bool,
    pub FaceCamera: bool,
    pub LightEmission: f32,
    pub LightInfluence: f32,
    pub Segments: i32,
    pub Texture: ContentId,
    pub TextureLength: f32,
    pub TextureMode: enums::TextureMode,
    pub TextureSpeed: f32,
    pub Transparency: NumberSequence,
    pub Width0: f32,
    pub Width1: f32,
    pub ZOffset: f32,
}
impl_inherits!(Beam, Instance);
impl Default for Beam {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            Brightness: 1f32,
            Color: ColorSequence {
                keypoints: vec![
                    ColorSequenceKeypoint::new(0f32, Color3::new(1f32, 1f32, 1f32)),
                    ColorSequenceKeypoint::new(1f32, Color3::new(1f32, 1f32, 1f32)),
                ],
            },
            CurveSize0: 0f32,
            CurveSize1: 0f32,
            Enabled: true,
            FaceCamera: false,
            LightEmission: 0f32,
            LightInfluence: 0f32,
            Segments: 10i32,
            Texture: "".into(),
            TextureLength: 1f32,
            TextureMode: enums::TextureMode::Stretch,
            TextureSpeed: 1f32,
            Transparency: NumberSequence {
                keypoints: vec![
                    NumberSequenceKeypoint::new(0f32, 0.5f32, 0f32),
                    NumberSequenceKeypoint::new(1f32, 0.5f32, 0f32),
                ],
            },
            Width0: 1f32,
            Width1: 1f32,
            ZOffset: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BevelMesh {
    #[doc(hidden)]
    pub superclass: DataModelMesh,
    pub Bevel: f32,
    pub BevelRoundness: f32,
    pub Bulge: f32,
}
impl_inherits!(BevelMesh, DataModelMesh);
impl Default for BevelMesh {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = DataModelMesh {
            superclass,
            Offset: Vector3::new(0f32, 0f32, 0f32),
            Scale: Vector3::new(0f32, 0f32, 0f32),
            VertexColor: Vector3::new(0f32, 0f32, 0f32),
        };
        Self {
            superclass,
            Bevel: 0f32,
            BevelRoundness: 0f32,
            Bulge: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BillboardGui {
    #[doc(hidden)]
    pub superclass: LayerCollector,
    pub Active: bool,
    pub Adornee: Ref,
    pub AlwaysOnTop: bool,
    pub Brightness: f32,
    pub ClipsDescendants: bool,
    pub DistanceLowerLimit: f32,
    pub DistanceUpperLimit: f32,
    pub ExtentsOffset: Vector3,
    pub ExtentsOffsetWorldSpace: Vector3,
    pub LightInfluence: f32,
    pub MaxDistance: f32,
    pub PlayerToHideFrom: Ref,
    pub Size: UDim2,
    pub SizeOffset: Vector2,
    pub StudsOffset: Vector3,
    pub StudsOffsetWorldSpace: Vector3,
}
impl_inherits!(BillboardGui, LayerCollector);
impl Default for BillboardGui {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase2d {
            superclass,
            AutoLocalize: true,
            RootLocalizationTable: Ref::none(),
            SelectionBehaviorDown: enums::SelectionBehavior::Escape,
            SelectionBehaviorLeft: enums::SelectionBehavior::Escape,
            SelectionBehaviorRight: enums::SelectionBehavior::Escape,
            SelectionBehaviorUp: enums::SelectionBehavior::Escape,
            SelectionGroup: false,
        };
        let superclass = LayerCollector {
            superclass,
            Enabled: true,
            ResetOnSpawn: true,
            ZIndexBehavior: enums::ZIndexBehavior::Global,
        };
        Self {
            superclass,
            Active: false,
            Adornee: Ref::none(),
            AlwaysOnTop: false,
            Brightness: 1f32,
            ClipsDescendants: false,
            DistanceLowerLimit: 0f32,
            DistanceUpperLimit: -1f32,
            ExtentsOffset: Vector3::new(0f32, 0f32, 0f32),
            ExtentsOffsetWorldSpace: Vector3::new(0f32, 0f32, 0f32),
            LightInfluence: 0f32,
            MaxDistance: f32::INFINITY,
            PlayerToHideFrom: Ref::none(),
            Size: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            SizeOffset: Vector2::new(0f32, 0f32),
            StudsOffset: Vector3::new(0f32, 0f32, 0f32),
            StudsOffsetWorldSpace: Vector3::new(0f32, 0f32, 0f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BinaryStringValue {
    #[doc(hidden)]
    pub superclass: ValueBase,
    pub Value: BinaryString,
}
impl_inherits!(BinaryStringValue, ValueBase);
impl Default for BinaryStringValue {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        Self {
            superclass,
            Value: b"".as_slice().into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BindableEvent {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(BindableEvent, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BindableFunction {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(BindableFunction, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BlockMesh {
    #[doc(hidden)]
    pub superclass: BevelMesh,
}
impl_inherits!(BlockMesh, BevelMesh);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BloomEffect {
    #[doc(hidden)]
    pub superclass: PostEffect,
    pub Intensity: f32,
    pub Size: f32,
    pub Threshold: f32,
}
impl_inherits!(BloomEffect, PostEffect);
impl Default for BloomEffect {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PostEffect {
            superclass,
            Enabled: true,
        };
        Self {
            superclass,
            Intensity: 0.4f32,
            Size: 24f32,
            Threshold: 0.95f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BlurEffect {
    #[doc(hidden)]
    pub superclass: PostEffect,
    pub Size: f32,
}
impl_inherits!(BlurEffect, PostEffect);
impl Default for BlurEffect {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PostEffect {
            superclass,
            Enabled: true,
        };
        Self {
            superclass,
            Size: 24f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyAngularVelocity {
    #[doc(hidden)]
    pub superclass: BodyMover,
    pub AngularVelocity: Vector3,
    pub MaxTorque: Vector3,
    pub P: f32,
}
impl_inherits!(BodyAngularVelocity, BodyMover);
impl Default for BodyAngularVelocity {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BodyMover { superclass };
        Self {
            superclass,
            AngularVelocity: Vector3::new(0f32, 2f32, 0f32),
            MaxTorque: Vector3::new(4000f32, 4000f32, 4000f32),
            P: 1250f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyColors {
    #[doc(hidden)]
    pub superclass: CharacterAppearance,
    pub HeadColor3: Color3,
    pub LeftArmColor3: Color3,
    pub LeftLegColor3: Color3,
    pub RightArmColor3: Color3,
    pub RightLegColor3: Color3,
    pub TorsoColor3: Color3,
}
impl_inherits!(BodyColors, CharacterAppearance);
impl Default for BodyColors {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = CharacterAppearance { superclass };
        Self {
            superclass,
            HeadColor3: Color3::new(0.9921569f32, 0.9176471f32, 0.5529412f32),
            LeftArmColor3: Color3::new(0.9921569f32, 0.9176471f32, 0.5529412f32),
            LeftLegColor3: Color3::new(0.050980397f32, 0.41176474f32, 0.6745098f32),
            RightArmColor3: Color3::new(0.9921569f32, 0.9176471f32, 0.5529412f32),
            RightLegColor3: Color3::new(0.050980397f32, 0.41176474f32, 0.6745098f32),
            TorsoColor3: Color3::new(0.15686275f32, 0.49803925f32, 0.2784314f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyForce {
    #[doc(hidden)]
    pub superclass: BodyMover,
    pub Force: Vector3,
}
impl_inherits!(BodyForce, BodyMover);
impl Default for BodyForce {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BodyMover { superclass };
        Self {
            superclass,
            Force: Vector3::new(0f32, 1f32, 0f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyGyro {
    #[doc(hidden)]
    pub superclass: BodyMover,
    pub CFrame: CFrame,
    pub D: f32,
    pub MaxTorque: Vector3,
    pub P: f32,
}
impl_inherits!(BodyGyro, BodyMover);
impl Default for BodyGyro {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BodyMover { superclass };
        Self {
            superclass,
            CFrame: CFrame::identity(),
            D: 500f32,
            MaxTorque: Vector3::new(400000f32, 0f32, 400000f32),
            P: 3000f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BodyMover {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(BodyMover, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyPartDescription {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AssetId: i64,
    pub BodyPart: enums::BodyPart,
    pub Color: Color3,
    pub HeadShape: String,
    pub Instance: Ref,
}
impl_inherits!(BodyPartDescription, Instance);
impl Default for BodyPartDescription {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AssetId: 0i64,
            BodyPart: enums::BodyPart::Head,
            Color: Color3::new(0f32, 0f32, 0f32),
            HeadShape: "".to_owned(),
            Instance: Ref::none(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyPosition {
    #[doc(hidden)]
    pub superclass: BodyMover,
    pub D: f32,
    pub MaxForce: Vector3,
    pub P: f32,
    pub Position: Vector3,
}
impl_inherits!(BodyPosition, BodyMover);
impl Default for BodyPosition {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BodyMover { superclass };
        Self {
            superclass,
            D: 1250f32,
            MaxForce: Vector3::new(4000f32, 4000f32, 4000f32),
            P: 10000f32,
            Position: Vector3::new(0f32, 50f32, 0f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyThrust {
    #[doc(hidden)]
    pub superclass: BodyMover,
    pub Force: Vector3,
    pub Location: Vector3,
}
impl_inherits!(BodyThrust, BodyMover);
impl Default for BodyThrust {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BodyMover { superclass };
        Self {
            superclass,
            Force: Vector3::new(0f32, 1f32, 0f32),
            Location: Vector3::new(0f32, 0f32, 0f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyVelocity {
    #[doc(hidden)]
    pub superclass: BodyMover,
    pub MaxForce: Vector3,
    pub P: f32,
    pub Velocity: Vector3,
}
impl_inherits!(BodyVelocity, BodyMover);
impl Default for BodyVelocity {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BodyMover { superclass };
        Self {
            superclass,
            MaxForce: Vector3::new(4000f32, 4000f32, 4000f32),
            P: 1250f32,
            Velocity: Vector3::new(0f32, 2f32, 0f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Bone {
    #[doc(hidden)]
    pub superclass: Attachment,
}
impl_inherits!(Bone, Attachment);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BoolValue {
    #[doc(hidden)]
    pub superclass: ValueBase,
    pub Value: bool,
}
impl_inherits!(BoolValue, ValueBase);
impl Default for BoolValue {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        Self {
            superclass,
            Value: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BoxHandleAdornment {
    #[doc(hidden)]
    pub superclass: HandleAdornment,
    pub Shading: enums::AdornShading,
    pub Size: Vector3,
}
impl_inherits!(BoxHandleAdornment, HandleAdornment);
impl Default for BoxHandleAdornment {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase3d {
            superclass,
            Color3: Color3::new(0.050980393f32, 0.4117647f32, 0.6745098f32),
            Transparency: 0f32,
            Visible: true,
        };
        let superclass = PVAdornment {
            superclass,
            Adornee: Ref::none(),
        };
        let superclass = HandleAdornment {
            superclass,
            AdornCullingMode: enums::AdornCullingMode::Automatic,
            AlwaysOnTop: false,
            CFrame: CFrame::identity(),
            SizeRelativeOffset: Vector3::new(0f32, 0f32, 0f32),
            ZIndex: -1i32,
        };
        Self {
            superclass,
            Shading: enums::AdornShading::Default,
            Size: Vector3::new(1f32, 1f32, 1f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Breakpoint {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(Breakpoint, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BrickColorValue {
    #[doc(hidden)]
    pub superclass: ValueBase,
    pub Value: BrickColor,
}
impl_inherits!(BrickColorValue, ValueBase);
impl Default for BrickColorValue {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        Self {
            superclass,
            Value: BrickColor::from_number(194u16).unwrap(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BrowserService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(BrowserService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BubbleChatConfiguration {
    #[doc(hidden)]
    pub superclass: TextChatConfigurations,
    pub AdorneeName: String,
    pub BackgroundColor3: Color3,
    pub BackgroundTransparency: f64,
    pub BubbleDuration: f32,
    pub BubblesSpacing: f32,
    pub Enabled: bool,
    pub Font: enums::Font,
    pub FontFace: Font,
    pub LocalPlayerStudsOffset: Vector3,
    pub MaxBubbles: f32,
    pub MaxDistance: f32,
    pub MinimizeDistance: f32,
    pub TailVisible: bool,
    pub TextColor3: Color3,
    pub TextSize: i64,
    pub VerticalStudsOffset: f32,
}
impl_inherits!(BubbleChatConfiguration, TextChatConfigurations);
impl Default for BubbleChatConfiguration {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = TextChatConfigurations { superclass };
        Self {
            superclass,
            AdorneeName: "HumanoidRootPart".to_owned(),
            BackgroundColor3: Color3::new(0.98039216f32, 0.98039216f32, 0.98039216f32),
            BackgroundTransparency: 0.1f64,
            BubbleDuration: 15f32,
            BubblesSpacing: 6f32,
            Enabled: true,
            Font: enums::Font::GothamMedium,
            FontFace: Font {
                family: "rbxasset://fonts/families/GothamSSm.json".to_owned(),
                weight: FontWeight::Medium,
                style: FontStyle::Normal,
                cached_face_id: Some("rbxasset://fonts/Montserrat-Medium.ttf".to_owned()),
            },
            LocalPlayerStudsOffset: Vector3::new(0f32, 0f32, 0f32),
            MaxBubbles: 3f32,
            MaxDistance: 100f32,
            MinimizeDistance: 40f32,
            TailVisible: true,
            TextColor3: Color3::new(0.22352941f32, 0.23137255f32, 0.23921569f32),
            TextSize: 16i64,
            VerticalStudsOffset: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BubbleChatMessageProperties {
    #[doc(hidden)]
    pub superclass: TextChatMessageProperties,
}
impl_inherits!(BubbleChatMessageProperties, TextChatMessageProperties);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BugReporterService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(BugReporterService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BulkImportService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(BulkImportService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BuoyancySensor {
    #[doc(hidden)]
    pub superclass: SensorBase,
    pub FullySubmerged: bool,
    pub TouchingSurface: bool,
}
impl_inherits!(BuoyancySensor, SensorBase);
impl Default for BuoyancySensor {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SensorBase {
            superclass,
            UpdateType: enums::SensorUpdateType::OnRead,
        };
        Self {
            superclass,
            FullySubmerged: false,
            TouchingSurface: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CFrameValue {
    #[doc(hidden)]
    pub superclass: ValueBase,
    pub Value: CFrame,
}
impl_inherits!(CFrameValue, ValueBase);
impl Default for CFrameValue {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        Self {
            superclass,
            Value: CFrame::identity(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CSGDictionaryService {
    #[doc(hidden)]
    pub superclass: FlyweightService,
}
impl_inherits!(CSGDictionaryService, FlyweightService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CacheableContentProvider {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(CacheableContentProvider, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CalloutService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(CalloutService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Camera {
    #[doc(hidden)]
    pub superclass: PVInstance,
    pub CFrame: CFrame,
    pub CameraSubject: Ref,
    pub CameraType: enums::CameraType,
    pub FieldOfView: f32,
    pub FieldOfViewMode: enums::FieldOfViewMode,
    pub Focus: CFrame,
    pub HeadLocked: bool,
    pub HeadScale: f32,
    pub VrTiltAndRollEnabled: bool,
}
impl_inherits!(Camera, PVInstance);
impl Default for Camera {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PVInstance { superclass };
        Self {
            superclass,
            CFrame: CFrame::new(
                Vector3::new(0f32, 20f32, 20f32),
                Matrix3::new(
                    Vector3::new(1f32, 0f32, -0f32),
                    Vector3::new(-0f32, 0.7071068f32, 0.7071068f32),
                    Vector3::new(0f32, -0.7071068f32, 0.7071068f32),
                ),
            ),
            CameraSubject: Ref::none(),
            CameraType: enums::CameraType::Fixed,
            FieldOfView: 70f32,
            FieldOfViewMode: enums::FieldOfViewMode::Vertical,
            Focus: CFrame::new(
                Vector3::new(0f32, 0f32, -5f32),
                Matrix3::new(
                    Vector3::new(1f32, 0f32, 0f32),
                    Vector3::new(0f32, 1f32, 0f32),
                    Vector3::new(0f32, 0f32, 1f32),
                ),
            ),
            HeadLocked: true,
            HeadScale: 1f32,
            VrTiltAndRollEnabled: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CanvasGroup {
    #[doc(hidden)]
    pub superclass: GuiObject,
    pub GroupColor3: Color3,
    pub GroupTransparency: f32,
}
impl_inherits!(CanvasGroup, GuiObject);
impl Default for CanvasGroup {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase2d {
            superclass,
            AutoLocalize: true,
            RootLocalizationTable: Ref::none(),
            SelectionBehaviorDown: enums::SelectionBehavior::Escape,
            SelectionBehaviorLeft: enums::SelectionBehavior::Escape,
            SelectionBehaviorRight: enums::SelectionBehavior::Escape,
            SelectionBehaviorUp: enums::SelectionBehavior::Escape,
            SelectionGroup: false,
        };
        let superclass = GuiObject {
            superclass,
            Active: false,
            AnchorPoint: Vector2::new(0f32, 0f32),
            AutomaticSize: enums::AutomaticSize::None,
            BackgroundColor3: Color3::new(0.6392157f32, 0.63529414f32, 0.64705884f32),
            BackgroundTransparency: 0f32,
            BorderColor3: Color3::new(0.105882354f32, 0.16470589f32, 0.20784314f32),
            BorderMode: enums::BorderMode::Outline,
            BorderSizePixel: 1i32,
            ClipsDescendants: true,
            Draggable: false,
            Interactable: true,
            LayoutOrder: 0i32,
            NextSelectionDown: Ref::none(),
            NextSelectionLeft: Ref::none(),
            NextSelectionRight: Ref::none(),
            NextSelectionUp: Ref::none(),
            Position: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            Rotation: 0f32,
            Selectable: false,
            SelectionImageObject: Ref::none(),
            SelectionOrder: 0i32,
            Size: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            SizeConstraint: enums::SizeConstraint::RelativeXY,
            Visible: true,
            ZIndex: 1i32,
        };
        Self {
            superclass,
            GroupColor3: Color3::new(1f32, 1f32, 1f32),
            GroupTransparency: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Capture {
    #[doc(hidden)]
    pub superclass: Object,
}
impl_inherits!(Capture, Object);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CaptureService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(CaptureService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CapturesPages {
    #[doc(hidden)]
    pub superclass: Pages,
}
impl_inherits!(CapturesPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CatalogPages {
    #[doc(hidden)]
    pub superclass: Pages,
}
impl_inherits!(CatalogPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ChangeHistoryService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ChangeHistoryService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ChangeHistoryStreamingService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ChangeHistoryStreamingService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChannelSelectorSoundEffect {
    #[doc(hidden)]
    pub superclass: CustomSoundEffect,
    pub Channel: i32,
}
impl_inherits!(ChannelSelectorSoundEffect, CustomSoundEffect);
impl Default for ChannelSelectorSoundEffect {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundEffect {
            superclass,
            Enabled: true,
            Priority: 0i32,
        };
        let superclass = CustomSoundEffect { superclass };
        Self {
            superclass,
            Channel: 1i32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChannelTabsConfiguration {
    #[doc(hidden)]
    pub superclass: TextChatConfigurations,
    pub BackgroundColor3: Color3,
    pub BackgroundTransparency: f64,
    pub Enabled: bool,
    pub FontFace: Font,
    pub HoverBackgroundColor3: Color3,
    pub SelectedTabTextColor3: Color3,
    pub TextColor3: Color3,
    pub TextSize: i64,
    pub TextStrokeColor3: Color3,
    pub TextStrokeTransparency: f64,
}
impl_inherits!(ChannelTabsConfiguration, TextChatConfigurations);
impl Default for ChannelTabsConfiguration {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = TextChatConfigurations { superclass };
        Self {
            superclass,
            BackgroundColor3: Color3::new(0.09803922f32, 0.105882354f32, 0.11372549f32),
            BackgroundTransparency: 0f64,
            Enabled: false,
            FontFace: Font {
                family: "rbxasset://fonts/families/BuilderSans.json".to_owned(),
                weight: FontWeight::Bold,
                style: FontStyle::Normal,
                cached_face_id: Some("rbxasset://fonts/BuilderSans-Bold.otf".to_owned()),
            },
            HoverBackgroundColor3: Color3::new(0.49019608f32, 0.49019608f32, 0.49019608f32),
            SelectedTabTextColor3: Color3::new(1f32, 1f32, 1f32),
            TextColor3: Color3::new(0.6862745f32, 0.6862745f32, 0.6862745f32),
            TextSize: 18i64,
            TextStrokeColor3: Color3::new(0f32, 0f32, 0f32),
            TextStrokeTransparency: 1f64,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CharacterAppearance {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(CharacterAppearance, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CharacterMesh {
    #[doc(hidden)]
    pub superclass: CharacterAppearance,
    pub BaseTextureId: i64,
    pub BodyPart: enums::BodyPart,
    pub MeshId: i64,
    pub OverlayTextureId: i64,
}
impl_inherits!(CharacterMesh, CharacterAppearance);
impl Default for CharacterMesh {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = CharacterAppearance { superclass };
        Self {
            superclass,
            BaseTextureId: 0i64,
            BodyPart: enums::BodyPart::Head,
            MeshId: 0i64,
            OverlayTextureId: 0i64,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Chat {
    #[doc(hidden)]
    pub superclass: Instance,
    pub BubbleChatEnabled: bool,
    pub IsAutoMigrated: bool,
    pub LoadDefaultChat: bool,
}
impl_inherits!(Chat, Instance);
impl Default for Chat {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            BubbleChatEnabled: false,
            IsAutoMigrated: false,
            LoadDefaultChat: true,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChatInputBarConfiguration {
    #[doc(hidden)]
    pub superclass: TextChatConfigurations,
    pub AutocompleteEnabled: bool,
    pub BackgroundColor3: Color3,
    pub BackgroundTransparency: f64,
    pub Enabled: bool,
    pub FontFace: Font,
    pub KeyboardKeyCode: enums::KeyCode,
    pub PlaceholderColor3: Color3,
    pub TargetTextChannel: Ref,
    pub TextColor3: Color3,
    pub TextSize: i64,
    pub TextStrokeColor3: Color3,
    pub TextStrokeTransparency: f64,
}
impl_inherits!(ChatInputBarConfiguration, TextChatConfigurations);
impl Default for ChatInputBarConfiguration {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = TextChatConfigurations { superclass };
        Self {
            superclass,
            AutocompleteEnabled: true,
            BackgroundColor3: Color3::new(0.09803922f32, 0.105882354f32, 0.11372549f32),
            BackgroundTransparency: 0.2f64,
            Enabled: true,
            FontFace: Font {
                family: "rbxasset://fonts/families/GothamSSm.json".to_owned(),
                weight: FontWeight::Medium,
                style: FontStyle::Normal,
                cached_face_id: Some("rbxasset://fonts/Montserrat-Medium.ttf".to_owned()),
            },
            KeyboardKeyCode: enums::KeyCode::Slash,
            PlaceholderColor3: Color3::new(0.69803923f32, 0.69803923f32, 0.69803923f32),
            TargetTextChannel: Ref::none(),
            TextColor3: Color3::new(1f32, 1f32, 1f32),
            TextSize: 14i64,
            TextStrokeColor3: Color3::new(0f32, 0f32, 0f32),
            TextStrokeTransparency: 0.5f64,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChatWindowConfiguration {
    #[doc(hidden)]
    pub superclass: TextChatConfigurations,
    pub BackgroundColor3: Color3,
    pub BackgroundTransparency: f64,
    pub Enabled: bool,
    pub FontFace: Font,
    pub HeightScale: f32,
    pub HorizontalAlignment: enums::HorizontalAlignment,
    pub TextColor3: Color3,
    pub TextSize: i64,
    pub TextStrokeColor3: Color3,
    pub TextStrokeTransparency: f64,
    pub VerticalAlignment: enums::VerticalAlignment,
    pub WidthScale: f32,
}
impl_inherits!(ChatWindowConfiguration, TextChatConfigurations);
impl Default for ChatWindowConfiguration {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = TextChatConfigurations { superclass };
        Self {
            superclass,
            BackgroundColor3: Color3::new(0.09803922f32, 0.105882354f32, 0.11372549f32),
            BackgroundTransparency: 0.3f64,
            Enabled: true,
            FontFace: Font {
                family: "rbxasset://fonts/families/GothamSSm.json".to_owned(),
                weight: FontWeight::Medium,
                style: FontStyle::Normal,
                cached_face_id: Some("rbxasset://fonts/Montserrat-Medium.ttf".to_owned()),
            },
            HeightScale: 1f32,
            HorizontalAlignment: enums::HorizontalAlignment::Left,
            TextColor3: Color3::new(1f32, 1f32, 1f32),
            TextSize: 14i64,
            TextStrokeColor3: Color3::new(0f32, 0f32, 0f32),
            TextStrokeTransparency: 0.5f64,
            VerticalAlignment: enums::VerticalAlignment::Top,
            WidthScale: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ChatWindowMessageProperties {
    #[doc(hidden)]
    pub superclass: TextChatMessageProperties,
}
impl_inherits!(ChatWindowMessageProperties, TextChatMessageProperties);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ChatbotUIService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ChatbotUIService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChorusSoundEffect {
    #[doc(hidden)]
    pub superclass: SoundEffect,
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
impl_inherits!(ChorusSoundEffect, SoundEffect);
impl Default for ChorusSoundEffect {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundEffect {
            superclass,
            Enabled: true,
            Priority: 0i32,
        };
        Self {
            superclass,
            Depth: 0.15f32,
            Mix: 0.5f32,
            Rate: 0.5f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ClickDetector {
    #[doc(hidden)]
    pub superclass: Instance,
    pub CursorIconContent: Content,
    pub MaxActivationDistance: f32,
}
impl_inherits!(ClickDetector, Instance);
impl Default for ClickDetector {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            CursorIconContent: Content::none(),
            MaxActivationDistance: 32f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ClientReplicator {
    #[doc(hidden)]
    pub superclass: NetworkReplicator,
}
impl_inherits!(ClientReplicator, NetworkReplicator);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ClimbController {
    #[doc(hidden)]
    pub superclass: ControllerBase,
    pub AccelerationTime: f32,
    pub BalanceMaxTorque: f32,
    pub BalanceSpeed: f32,
    pub MoveMaxForce: f32,
}
impl_inherits!(ClimbController, ControllerBase);
impl Default for ClimbController {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ControllerBase {
            superclass,
            BalanceRigidityEnabled: false,
            MoveSpeedFactor: 1f32,
        };
        Self {
            superclass,
            AccelerationTime: 0f32,
            BalanceMaxTorque: 10000f32,
            BalanceSpeed: 100f32,
            MoveMaxForce: 10000f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Clothing {
    #[doc(hidden)]
    pub superclass: CharacterAppearance,
    pub Color3: Color3,
}
impl_inherits!(Clothing, CharacterAppearance);
impl Default for Clothing {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = CharacterAppearance { superclass };
        Self {
            superclass,
            Color3: Color3::new(0f32, 0f32, 0f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CloudCRUDService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(CloudCRUDService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CloudLocalizationTable {
    #[doc(hidden)]
    pub superclass: LocalizationTable,
}
impl_inherits!(CloudLocalizationTable, LocalizationTable);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Clouds {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Color: Color3,
    pub Cover: f32,
    pub Density: f32,
    pub Enabled: bool,
}
impl_inherits!(Clouds, Instance);
impl Default for Clouds {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Color: Color3::new(1f32, 1f32, 1f32),
            Cover: 0.5f32,
            Density: 0.7f32,
            Enabled: true,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ClusterPacketCache {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ClusterPacketCache, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Collaborator {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(Collaborator, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CollaboratorsService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(CollaboratorsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CollectionService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(CollectionService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Color3Value {
    #[doc(hidden)]
    pub superclass: ValueBase,
    pub Value: Color3,
}
impl_inherits!(Color3Value, ValueBase);
impl Default for Color3Value {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        Self {
            superclass,
            Value: Color3::new(0f32, 0f32, 0f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ColorCorrectionEffect {
    #[doc(hidden)]
    pub superclass: PostEffect,
    pub Brightness: f32,
    pub Contrast: f32,
    pub Saturation: f32,
    pub TintColor: Color3,
}
impl_inherits!(ColorCorrectionEffect, PostEffect);
impl Default for ColorCorrectionEffect {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PostEffect {
            superclass,
            Enabled: true,
        };
        Self {
            superclass,
            Brightness: 0f32,
            Contrast: 0f32,
            Saturation: 0f32,
            TintColor: Color3::new(1f32, 1f32, 1f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ColorGradingEffect {
    #[doc(hidden)]
    pub superclass: PostEffect,
    pub TonemapperPreset: enums::TonemapperPreset,
}
impl_inherits!(ColorGradingEffect, PostEffect);
impl Default for ColorGradingEffect {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PostEffect {
            superclass,
            Enabled: true,
        };
        Self {
            superclass,
            TonemapperPreset: enums::TonemapperPreset::Default,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CommerceService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(CommerceService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CompressorSoundEffect {
    #[doc(hidden)]
    pub superclass: SoundEffect,
    pub Attack: f32,
    pub GainMakeup: f32,
    pub Ratio: f32,
    pub Release: f32,
    pub SideChain: Ref,
    pub Threshold: f32,
}
impl_inherits!(CompressorSoundEffect, SoundEffect);
impl Default for CompressorSoundEffect {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundEffect {
            superclass,
            Enabled: true,
            Priority: 0i32,
        };
        Self {
            superclass,
            Attack: 0.1f32,
            GainMakeup: 0f32,
            Ratio: 40f32,
            Release: 0.1f32,
            SideChain: Ref::none(),
            Threshold: -40f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ConeHandleAdornment {
    #[doc(hidden)]
    pub superclass: HandleAdornment,
    pub Height: f32,
    pub Hollow: bool,
    pub Radius: f32,
    pub Shading: enums::AdornShading,
}
impl_inherits!(ConeHandleAdornment, HandleAdornment);
impl Default for ConeHandleAdornment {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase3d {
            superclass,
            Color3: Color3::new(0.050980393f32, 0.4117647f32, 0.6745098f32),
            Transparency: 0f32,
            Visible: true,
        };
        let superclass = PVAdornment {
            superclass,
            Adornee: Ref::none(),
        };
        let superclass = HandleAdornment {
            superclass,
            AdornCullingMode: enums::AdornCullingMode::Automatic,
            AlwaysOnTop: false,
            CFrame: CFrame::identity(),
            SizeRelativeOffset: Vector3::new(0f32, 0f32, 0f32),
            ZIndex: -1i32,
        };
        Self {
            superclass,
            Height: 2f32,
            Hollow: false,
            Radius: 0.5f32,
            Shading: enums::AdornShading::Default,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ConfigService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ConfigService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ConfigSnapshot {
    #[doc(hidden)]
    pub superclass: Object,
}
impl_inherits!(ConfigSnapshot, Object);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Configuration {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(Configuration, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ConfigureServerService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ConfigureServerService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ConnectivityService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ConnectivityService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Constraint {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Attachment0: Ref,
    pub Attachment1: Ref,
    pub Color: BrickColor,
    pub Enabled: bool,
    pub Visible: bool,
}
impl_inherits!(Constraint, Instance);
impl Default for Constraint {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            Color: BrickColor::from_number(194u16).unwrap(),
            Enabled: false,
            Visible: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ContentProvider {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ContentProvider, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ContextActionService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ContextActionService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Controller {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(Controller, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ControllerBase {
    #[doc(hidden)]
    pub superclass: Instance,
    pub BalanceRigidityEnabled: bool,
    pub MoveSpeedFactor: f32,
}
impl_inherits!(ControllerBase, Instance);
impl Default for ControllerBase {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            BalanceRigidityEnabled: false,
            MoveSpeedFactor: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ControllerManager {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ActiveController: Ref,
    pub BaseMoveSpeed: f32,
    pub BaseTurnSpeed: f32,
    pub ClimbSensor: Ref,
    pub FacingDirection: Vector3,
    pub GroundSensor: Ref,
    pub MovingDirection: Vector3,
    pub RootPart: Ref,
    pub UpDirection: Vector3,
}
impl_inherits!(ControllerManager, Instance);
impl Default for ControllerManager {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ActiveController: Ref::none(),
            BaseMoveSpeed: 16f32,
            BaseTurnSpeed: 8f32,
            ClimbSensor: Ref::none(),
            FacingDirection: Vector3::new(0f32, 0f32, 1f32),
            GroundSensor: Ref::none(),
            MovingDirection: Vector3::new(0f32, 0f32, 0f32),
            RootPart: Ref::none(),
            UpDirection: Vector3::new(0f32, 1f32, 0f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ControllerPartSensor {
    #[doc(hidden)]
    pub superclass: ControllerSensor,
    pub HitFrame: CFrame,
    pub HitNormal: Vector3,
    pub SearchDistance: f32,
    pub SensedPart: Ref,
    pub SensorMode: enums::SensorMode,
}
impl_inherits!(ControllerPartSensor, ControllerSensor);
impl Default for ControllerPartSensor {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SensorBase {
            superclass,
            UpdateType: enums::SensorUpdateType::OnRead,
        };
        let superclass = ControllerSensor { superclass };
        Self {
            superclass,
            HitFrame: CFrame::identity(),
            HitNormal: Vector3::new(0f32, 0f32, 0f32),
            SearchDistance: 0f32,
            SensedPart: Ref::none(),
            SensorMode: enums::SensorMode::Floor,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ControllerSensor {
    #[doc(hidden)]
    pub superclass: SensorBase,
}
impl_inherits!(ControllerSensor, SensorBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ControllerService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ControllerService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ConversationalAIAcceptanceService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ConversationalAIAcceptanceService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CookiesService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(CookiesService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CoreGui {
    #[doc(hidden)]
    pub superclass: BasePlayerGui,
    pub SelectionImageObject: Ref,
}
impl_inherits!(CoreGui, BasePlayerGui);
impl Default for CoreGui {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BasePlayerGui { superclass };
        Self {
            superclass,
            SelectionImageObject: Ref::none(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CorePackages {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(CorePackages, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CoreScript {
    #[doc(hidden)]
    pub superclass: BaseScript,
}
impl_inherits!(CoreScript, BaseScript);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CoreScriptDebuggingManagerHelper {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(CoreScriptDebuggingManagerHelper, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CoreScriptSyncService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(CoreScriptSyncService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CornerWedgePart {
    #[doc(hidden)]
    pub superclass: BasePart,
}
impl_inherits!(CornerWedgePart, BasePart);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CreationDBService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(CreationDBService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CreatorStoreService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(CreatorStoreService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CrossDMScriptChangeListener {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(CrossDMScriptChangeListener, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CurveAnimation {
    #[doc(hidden)]
    pub superclass: AnimationClip,
}
impl_inherits!(CurveAnimation, AnimationClip);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CustomEvent {
    #[doc(hidden)]
    pub superclass: Instance,
    pub PersistedCurrentValue: f32,
}
impl_inherits!(CustomEvent, Instance);
impl Default for CustomEvent {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            PersistedCurrentValue: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CustomEventReceiver {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Source: Ref,
}
impl_inherits!(CustomEventReceiver, Instance);
impl Default for CustomEventReceiver {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Source: Ref::none(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CustomLog {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(CustomLog, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CustomSoundEffect {
    #[doc(hidden)]
    pub superclass: SoundEffect,
}
impl_inherits!(CustomSoundEffect, SoundEffect);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CylinderHandleAdornment {
    #[doc(hidden)]
    pub superclass: HandleAdornment,
    pub Angle: f32,
    pub Height: f32,
    pub InnerRadius: f32,
    pub Radius: f32,
    pub Shading: enums::AdornShading,
}
impl_inherits!(CylinderHandleAdornment, HandleAdornment);
impl Default for CylinderHandleAdornment {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase3d {
            superclass,
            Color3: Color3::new(0.050980393f32, 0.4117647f32, 0.6745098f32),
            Transparency: 0f32,
            Visible: true,
        };
        let superclass = PVAdornment {
            superclass,
            Adornee: Ref::none(),
        };
        let superclass = HandleAdornment {
            superclass,
            AdornCullingMode: enums::AdornCullingMode::Automatic,
            AlwaysOnTop: false,
            CFrame: CFrame::identity(),
            SizeRelativeOffset: Vector3::new(0f32, 0f32, 0f32),
            ZIndex: -1i32,
        };
        Self {
            superclass,
            Angle: 360f32,
            Height: 1f32,
            InnerRadius: 0f32,
            Radius: 1f32,
            Shading: enums::AdornShading::Default,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CylinderMesh {
    #[doc(hidden)]
    pub superclass: BevelMesh,
}
impl_inherits!(CylinderMesh, BevelMesh);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CylindricalConstraint {
    #[doc(hidden)]
    pub superclass: SlidingBallConstraint,
    pub AngularActuatorType: enums::ActuatorType,
    pub AngularLimitsEnabled: bool,
    pub AngularResponsiveness: f32,
    pub AngularRestitution: f32,
    pub AngularSpeed: f32,
    pub AngularVelocity: f32,
    pub InclinationAngle: f32,
    pub LowerAngle: f32,
    pub MotorMaxAngularAcceleration: f32,
    pub MotorMaxTorque: f32,
    pub RotationAxisVisible: bool,
    pub ServoMaxTorque: f32,
    pub SoftlockAngularServoUponReachingTarget: bool,
    pub TargetAngle: f32,
    pub UpperAngle: f32,
}
impl_inherits!(CylindricalConstraint, SlidingBallConstraint);
impl Default for CylindricalConstraint {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Constraint {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            Color: BrickColor::from_number(1009u16).unwrap(),
            Enabled: true,
            Visible: false,
        };
        let superclass = SlidingBallConstraint {
            superclass,
            ActuatorType: enums::ActuatorType::None,
            LimitsEnabled: false,
            LinearResponsiveness: 45f32,
            LowerLimit: 0f32,
            MotorMaxAcceleration: f32::INFINITY,
            MotorMaxForce: 0f32,
            Restitution: 0f32,
            ServoMaxForce: 0f32,
            Size: 0.15f32,
            SoftlockServoUponReachingTarget: false,
            Speed: 0f32,
            TargetPosition: 0f32,
            UpperLimit: 5f32,
            Velocity: 0f32,
        };
        Self {
            superclass,
            AngularActuatorType: enums::ActuatorType::None,
            AngularLimitsEnabled: false,
            AngularResponsiveness: 45f32,
            AngularRestitution: 0f32,
            AngularSpeed: 0f32,
            AngularVelocity: 0f32,
            InclinationAngle: 0f32,
            LowerAngle: -45f32,
            MotorMaxAngularAcceleration: 500000f32,
            MotorMaxTorque: 0f32,
            RotationAxisVisible: false,
            ServoMaxTorque: 0f32,
            SoftlockAngularServoUponReachingTarget: false,
            TargetAngle: 0f32,
            UpperAngle: 45f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataModel {
    #[doc(hidden)]
    pub superclass: ServiceProvider,
}
impl_inherits!(DataModel, ServiceProvider);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataModelMesh {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Offset: Vector3,
    pub Scale: Vector3,
    pub VertexColor: Vector3,
}
impl_inherits!(DataModelMesh, Instance);
impl Default for DataModelMesh {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Offset: Vector3::new(0f32, 0f32, 0f32),
            Scale: Vector3::new(0f32, 0f32, 0f32),
            VertexColor: Vector3::new(0f32, 0f32, 0f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataModelPatchService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(DataModelPatchService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataModelSession {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(DataModelSession, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStore {
    #[doc(hidden)]
    pub superclass: GlobalDataStore,
}
impl_inherits!(DataStore, GlobalDataStore);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreGetOptions {
    #[doc(hidden)]
    pub superclass: Instance,
    pub UseCache: bool,
}
impl_inherits!(DataStoreGetOptions, Instance);
impl Default for DataStoreGetOptions {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            UseCache: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreIncrementOptions {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(DataStoreIncrementOptions, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreInfo {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(DataStoreInfo, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreKey {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(DataStoreKey, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreKeyInfo {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(DataStoreKeyInfo, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreKeyPages {
    #[doc(hidden)]
    pub superclass: Pages,
}
impl_inherits!(DataStoreKeyPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreListingPages {
    #[doc(hidden)]
    pub superclass: Pages,
}
impl_inherits!(DataStoreListingPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreObjectVersionInfo {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(DataStoreObjectVersionInfo, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreOptions {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AllScopes: bool,
}
impl_inherits!(DataStoreOptions, Instance);
impl Default for DataStoreOptions {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AllScopes: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStorePages {
    #[doc(hidden)]
    pub superclass: Pages,
}
impl_inherits!(DataStorePages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AutomaticRetry: bool,
    pub LegacyNamingScheme: bool,
}
impl_inherits!(DataStoreService, Instance);
impl Default for DataStoreService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AutomaticRetry: true,
            LegacyNamingScheme: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreSetOptions {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(DataStoreSetOptions, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreVersionPages {
    #[doc(hidden)]
    pub superclass: Pages,
}
impl_inherits!(DataStoreVersionPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Debris {
    #[doc(hidden)]
    pub superclass: Instance,
    pub MaxItems: i32,
}
impl_inherits!(Debris, Instance);
impl Default for Debris {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            MaxItems: 1000i32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebugSettings {
    #[doc(hidden)]
    pub superclass: Instance,
    pub IsScriptStackTracingEnabled: bool,
    pub ReportSoundWarnings: bool,
    pub TickCountPreciseOverride: enums::TickCountSampleMethod,
}
impl_inherits!(DebugSettings, Instance);
impl Default for DebugSettings {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            IsScriptStackTracingEnabled: false,
            ReportSoundWarnings: false,
            TickCountPreciseOverride: enums::TickCountSampleMethod::Fast,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DebuggablePluginWatcher {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(DebuggablePluginWatcher, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerBreakpoint {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Condition: String,
    pub ContinueExecution: bool,
    pub IsContextDependentBreakpoint: bool,
    pub IsEnabled: bool,
    pub Line: i32,
    pub LogExpression: String,
}
impl_inherits!(DebuggerBreakpoint, Instance);
impl Default for DebuggerBreakpoint {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Condition: "".to_owned(),
            ContinueExecution: false,
            IsContextDependentBreakpoint: false,
            IsEnabled: false,
            Line: 0i32,
            LogExpression: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DebuggerConnection {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(DebuggerConnection, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerConnectionManager {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Timeout: f64,
}
impl_inherits!(DebuggerConnectionManager, Instance);
impl Default for DebuggerConnectionManager {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Timeout: 0f64,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DebuggerLuaResponse {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(DebuggerLuaResponse, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DebuggerManager {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(DebuggerManager, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DebuggerUIService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(DebuggerUIService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DebuggerVariable {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(DebuggerVariable, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerWatch {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Expression: String,
}
impl_inherits!(DebuggerWatch, Instance);
impl Default for DebuggerWatch {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Expression: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Decal {
    #[doc(hidden)]
    pub superclass: FaceInstance,
    pub Color3: Color3,
    pub MetalnessMapContent: Content,
    pub NormalMapContent: Content,
    pub RoughnessMapContent: Content,
    pub TextureContent: Content,
    pub TexturePack: ContentId,
    pub TexturePackMetadata: String,
    pub Transparency: f32,
    pub UvOffset: Vector2,
    pub UvScale: Vector2,
    pub ZIndex: i32,
}
impl_inherits!(Decal, FaceInstance);
impl Default for Decal {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = FaceInstance {
            superclass,
            Face: enums::NormalId::Front,
        };
        Self {
            superclass,
            Color3: Color3::new(1f32, 1f32, 1f32),
            MetalnessMapContent: Content::none(),
            NormalMapContent: Content::none(),
            RoughnessMapContent: Content::none(),
            TextureContent: Content::none(),
            TexturePack: "".into(),
            TexturePackMetadata: "".to_owned(),
            Transparency: 0f32,
            UvOffset: Vector2::new(0f32, 0f32),
            UvScale: Vector2::new(1f32, 1f32),
            ZIndex: 1i32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DepthOfFieldEffect {
    #[doc(hidden)]
    pub superclass: PostEffect,
    pub FarIntensity: f32,
    pub FocusDistance: f32,
    pub InFocusRadius: f32,
    pub NearIntensity: f32,
}
impl_inherits!(DepthOfFieldEffect, PostEffect);
impl Default for DepthOfFieldEffect {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PostEffect {
            superclass,
            Enabled: true,
        };
        Self {
            superclass,
            FarIntensity: 0.75f32,
            FocusDistance: 0.05f32,
            InFocusRadius: 10f32,
            NearIntensity: 0.75f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DeviceIdService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(DeviceIdService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Dialog {
    #[doc(hidden)]
    pub superclass: Instance,
    pub BehaviorType: enums::DialogBehaviorType,
    pub ConversationDistance: f32,
    pub GoodbyeChoiceActive: bool,
    pub GoodbyeDialog: String,
    pub InitialPrompt: String,
    pub Purpose: enums::DialogPurpose,
    pub Tone: enums::DialogTone,
    pub TriggerDistance: f32,
    pub TriggerOffset: Vector3,
}
impl_inherits!(Dialog, Instance);
impl Default for Dialog {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            BehaviorType: enums::DialogBehaviorType::SinglePlayer,
            ConversationDistance: 25f32,
            GoodbyeChoiceActive: true,
            GoodbyeDialog: "".to_owned(),
            InitialPrompt: "".to_owned(),
            Purpose: enums::DialogPurpose::Help,
            Tone: enums::DialogTone::Neutral,
            TriggerDistance: 0f32,
            TriggerOffset: Vector3::new(0f32, 0f32, 0f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DialogChoice {
    #[doc(hidden)]
    pub superclass: Instance,
    pub GoodbyeChoiceActive: bool,
    pub GoodbyeDialog: String,
    pub ResponseDialog: String,
    pub UserDialog: String,
}
impl_inherits!(DialogChoice, Instance);
impl Default for DialogChoice {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            GoodbyeChoiceActive: true,
            GoodbyeDialog: "".to_owned(),
            ResponseDialog: "".to_owned(),
            UserDialog: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DistortionSoundEffect {
    #[doc(hidden)]
    pub superclass: SoundEffect,
    pub Level: f32,
}
impl_inherits!(DistortionSoundEffect, SoundEffect);
impl Default for DistortionSoundEffect {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundEffect {
            superclass,
            Enabled: true,
            Priority: 0i32,
        };
        Self {
            superclass,
            Level: 0.75f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DockWidgetPluginGui {
    #[doc(hidden)]
    pub superclass: PluginGui,
}
impl_inherits!(DockWidgetPluginGui, PluginGui);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DoubleConstrainedValue {
    #[doc(hidden)]
    pub superclass: ValueBase,
    pub MaxValue: f64,
    pub MinValue: f64,
    pub Value: f64,
}
impl_inherits!(DoubleConstrainedValue, ValueBase);
impl Default for DoubleConstrainedValue {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        Self {
            superclass,
            MaxValue: 1f64,
            MinValue: 0f64,
            Value: 0f64,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DraftsService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(DraftsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DragDetector {
    #[doc(hidden)]
    pub superclass: ClickDetector,
    pub ActivatedCursorIconContent: Content,
    pub ApplyAtCenterOfMass: bool,
    pub DragFrame: CFrame,
    pub DragStyle: enums::DragDetectorDragStyle,
    pub Enabled: bool,
    pub GamepadModeSwitchKeyCode: enums::KeyCode,
    pub KeyboardModeSwitchKeyCode: enums::KeyCode,
    pub MaxDragAngle: f32,
    pub MaxDragTranslation: Vector3,
    pub MaxForce: f32,
    pub MaxTorque: f32,
    pub MinDragAngle: f32,
    pub MinDragTranslation: Vector3,
    pub Orientation: Vector3,
    pub PermissionPolicy: enums::DragDetectorPermissionPolicy,
    pub ReferenceInstance: Ref,
    pub ResponseStyle: enums::DragDetectorResponseStyle,
    pub Responsiveness: f32,
    pub RunLocally: bool,
    pub TrackballRadialPullFactor: f32,
    pub TrackballRollFactor: f32,
    pub VrSwitchKeyCode: enums::KeyCode,
}
impl_inherits!(DragDetector, ClickDetector);
impl Default for DragDetector {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ClickDetector {
            superclass,
            CursorIconContent: Content::none(),
            MaxActivationDistance: 32f32,
        };
        Self {
            superclass,
            ActivatedCursorIconContent: Content::none(),
            ApplyAtCenterOfMass: false,
            DragFrame: CFrame::identity(),
            DragStyle: enums::DragDetectorDragStyle::TranslatePlane,
            Enabled: true,
            GamepadModeSwitchKeyCode: enums::KeyCode::ButtonR1,
            KeyboardModeSwitchKeyCode: enums::KeyCode::LeftControl,
            MaxDragAngle: 0f32,
            MaxDragTranslation: Vector3::new(0f32, 0f32, 0f32),
            MaxForce: 10000000f32,
            MaxTorque: 10000f32,
            MinDragAngle: 0f32,
            MinDragTranslation: Vector3::new(0f32, 0f32, 0f32),
            Orientation: Vector3::new(-0f32, 179.99998f32, 90f32),
            PermissionPolicy: enums::DragDetectorPermissionPolicy::Everybody,
            ReferenceInstance: Ref::none(),
            ResponseStyle: enums::DragDetectorResponseStyle::Physical,
            Responsiveness: 10f32,
            RunLocally: false,
            TrackballRadialPullFactor: 1f32,
            TrackballRollFactor: 1f32,
            VrSwitchKeyCode: enums::KeyCode::ButtonL2,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Dragger {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(Dragger, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DraggerService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(DraggerService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DynamicRotate {
    #[doc(hidden)]
    pub superclass: JointInstance,
    pub BaseAngle: f32,
}
impl_inherits!(DynamicRotate, JointInstance);
impl Default for DynamicRotate {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = JointInstance {
            superclass,
            C0: CFrame::identity(),
            C1: CFrame::identity(),
            Enabled: false,
            Part0: Ref::none(),
            Part1: Ref::none(),
        };
        Self {
            superclass,
            BaseAngle: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EchoSoundEffect {
    #[doc(hidden)]
    pub superclass: SoundEffect,
    pub Delay: f32,
    pub DryLevel: f32,
    pub Feedback: f32,
    pub WetLevel: f32,
}
impl_inherits!(EchoSoundEffect, SoundEffect);
impl Default for EchoSoundEffect {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundEffect {
            superclass,
            Enabled: true,
            Priority: 0i32,
        };
        Self {
            superclass,
            Delay: 1f32,
            DryLevel: 0f32,
            Feedback: 0.5f32,
            WetLevel: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EditableImage {
    #[doc(hidden)]
    pub superclass: Object,
    pub ImageData: BinaryString,
}
impl_inherits!(EditableImage, Object);
impl Default for EditableImage {
    fn default() -> Self {
        let superclass = Object::default();
        Self {
            superclass,
            ImageData: b"".as_slice().into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EditableMesh {
    #[doc(hidden)]
    pub superclass: Object,
    pub MeshData: SharedString,
    pub SkinningEnabled: bool,
}
impl_inherits!(EditableMesh, Object);
impl Default for EditableMesh {
    fn default() -> Self {
        let superclass = Object::default();
        Self {
            superclass,
            MeshData: SharedString::new(b"".to_vec()),
            SkinningEnabled: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct EditableService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(EditableService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct EmotesPages {
    #[doc(hidden)]
    pub superclass: InventoryPages,
}
impl_inherits!(EmotesPages, InventoryPages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct EncodingService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(EncodingService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EqualizerSoundEffect {
    #[doc(hidden)]
    pub superclass: SoundEffect,
    pub HighGain: f32,
    pub LowGain: f32,
    pub MidGain: f32,
}
impl_inherits!(EqualizerSoundEffect, SoundEffect);
impl Default for EqualizerSoundEffect {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundEffect {
            superclass,
            Enabled: true,
            Priority: 0i32,
        };
        Self {
            superclass,
            HighGain: 0f32,
            LowGain: -20f32,
            MidGain: -10f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EulerRotationCurve {
    #[doc(hidden)]
    pub superclass: Instance,
    pub RotationOrder: enums::RotationOrder,
}
impl_inherits!(EulerRotationCurve, Instance);
impl Default for EulerRotationCurve {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            RotationOrder: enums::RotationOrder::XYZ,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct EventIngestService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(EventIngestService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExampleV2Service {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ExampleV2Service, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExecutedRemoteCommand {
    #[doc(hidden)]
    pub superclass: Object,
}
impl_inherits!(ExecutedRemoteCommand, Object);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExperienceAuthService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ExperienceAuthService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ExperienceInviteOptions {
    #[doc(hidden)]
    pub superclass: Instance,
    pub InviteMessageId: String,
    pub InviteUser: i64,
    pub LaunchData: String,
    pub PromptMessage: String,
}
impl_inherits!(ExperienceInviteOptions, Instance);
impl Default for ExperienceInviteOptions {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            InviteMessageId: "".to_owned(),
            InviteUser: 0i64,
            LaunchData: "".to_owned(),
            PromptMessage: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExperienceNotificationService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ExperienceNotificationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExperienceService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ExperienceService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExperienceStateCaptureService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ExperienceStateCaptureService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExperienceStateRecordingService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ExperienceStateRecordingService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExplorerFilter {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ExplorerFilter, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExplorerFilterAutocompleter {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ExplorerFilterAutocompleter, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExplorerServiceVisibilityService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ExplorerServiceVisibilityService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Explosion {
    #[doc(hidden)]
    pub superclass: Instance,
    pub BlastPressure: f32,
    pub BlastRadius: f32,
    pub DestroyJointRadiusPercent: f32,
    pub ExplosionType: enums::ExplosionType,
    pub Position: Vector3,
    pub TimeScale: f32,
    pub Visible: bool,
}
impl_inherits!(Explosion, Instance);
impl Default for Explosion {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            BlastPressure: 500000f32,
            BlastRadius: 4f32,
            DestroyJointRadiusPercent: 1f32,
            ExplosionType: enums::ExplosionType::Craters,
            Position: Vector3::new(0f32, 0f32, 0f32),
            TimeScale: 1f32,
            Visible: true,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FaceAnimatorService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(FaceAnimatorService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FaceControls {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(FaceControls, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FaceInstance {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Face: enums::NormalId,
}
impl_inherits!(FaceInstance, Instance);
impl Default for FaceInstance {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Face: enums::NormalId::Right,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FacialAgeEstimationService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(FacialAgeEstimationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FacialAnimationRecordingService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(FacialAnimationRecordingService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FacialAnimationStreamingServiceStats {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(FacialAnimationStreamingServiceStats, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FacialAnimationStreamingServiceV2 {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ServiceState: i32,
}
impl_inherits!(FacialAnimationStreamingServiceV2, Instance);
impl Default for FacialAnimationStreamingServiceV2 {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ServiceState: 0i32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FacialAnimationStreamingSubsessionStats {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(FacialAnimationStreamingSubsessionStats, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FacsImportData {
    #[doc(hidden)]
    pub superclass: BaseImportData,
}
impl_inherits!(FacsImportData, BaseImportData);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Feature {
    #[doc(hidden)]
    pub superclass: Instance,
    pub FaceId: enums::NormalId,
    pub InOut: enums::InOut,
    pub LeftRight: enums::LeftRight,
    pub TopBottom: enums::TopBottom,
}
impl_inherits!(Feature, Instance);
impl Default for Feature {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            FaceId: enums::NormalId::Right,
            InOut: enums::InOut::Edge,
            LeftRight: enums::LeftRight::Left,
            TopBottom: enums::TopBottom::Top,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FeatureRestrictionManager {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(FeatureRestrictionManager, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct File {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(File, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FileMesh {
    #[doc(hidden)]
    pub superclass: DataModelMesh,
    pub MeshId: ContentId,
    pub TextureId: ContentId,
}
impl_inherits!(FileMesh, DataModelMesh);
impl Default for FileMesh {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = DataModelMesh {
            superclass,
            Offset: Vector3::new(0f32, 0f32, 0f32),
            Scale: Vector3::new(1f32, 1f32, 1f32),
            VertexColor: Vector3::new(1f32, 1f32, 1f32),
        };
        Self {
            superclass,
            MeshId: "".into(),
            TextureId: "".into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Fire {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Color: Color3,
    pub Enabled: bool,
    pub SecondaryColor: Color3,
    pub TimeScale: f32,
}
impl_inherits!(Fire, Instance);
impl Default for Fire {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Color: Color3::new(0.92549026f32, 0.54509807f32, 0.27450982f32),
            Enabled: true,
            SecondaryColor: Color3::new(0.54509807f32, 0.3137255f32, 0.21568629f32),
            TimeScale: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Flag {
    #[doc(hidden)]
    pub superclass: Tool,
    pub TeamColor: BrickColor,
}
impl_inherits!(Flag, Tool);
impl Default for Flag {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PVInstance { superclass };
        let superclass = Model {
            superclass,
            LevelOfDetail: enums::ModelLevelOfDetail::Automatic,
            ModelMeshCFrame: CFrame::identity(),
            ModelMeshData: SharedString::new(b"".to_vec()),
            ModelMeshSize: Vector3::new(0f32, 0f32, 0f32),
            ModelStreamingMode: enums::ModelStreamingMode::Default,
            NeedsPivotMigration: false,
            PrimaryPart: Ref::none(),
            SlimHash: SharedString::new(b"".to_vec()),
            WorldPivotData: None,
        };
        let superclass = BackpackItem {
            superclass,
            TextureContent: Content::none(),
        };
        let superclass = Tool {
            superclass,
            CanBeDropped: true,
            Enabled: true,
            Grip: CFrame::identity(),
            ManualActivationOnly: false,
            RequiresHandle: true,
            ToolTip: "".to_owned(),
        };
        Self {
            superclass,
            TeamColor: BrickColor::from_number(194u16).unwrap(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FlagStand {
    #[doc(hidden)]
    pub superclass: Part,
    pub TeamColor: BrickColor,
}
impl_inherits!(FlagStand, Part);
impl Default for FlagStand {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PVInstance { superclass };
        let superclass = BasePart {
            superclass,
            Anchored: false,
            AudioCanCollide: true,
            BackParamA: -0.5f32,
            BackParamB: 0.5f32,
            BackSurface: enums::SurfaceType::Smooth,
            BackSurfaceInput: enums::InputType::NoInput,
            BottomParamA: -0.5f32,
            BottomParamB: 0.5f32,
            BottomSurface: enums::SurfaceType::Inlet,
            BottomSurfaceInput: enums::InputType::NoInput,
            CFrame: CFrame::identity(),
            CanCollide: true,
            CanQuery: true,
            CanTouch: true,
            CastShadow: true,
            CollisionGroup: "Default".to_owned(),
            CollisionGroupId: 0i32,
            CustomPhysicalProperties: PhysicalProperties::Default,
            EnableFluidForces: true,
            FrontParamA: -0.5f32,
            FrontParamB: 0.5f32,
            FrontSurface: enums::SurfaceType::Smooth,
            FrontSurfaceInput: enums::InputType::NoInput,
            LeftParamA: -0.5f32,
            LeftParamB: 0.5f32,
            LeftSurface: enums::SurfaceType::Smooth,
            LeftSurfaceInput: enums::InputType::NoInput,
            Locked: false,
            Massless: false,
            Material: enums::Material::Plastic,
            PivotOffset: CFrame::identity(),
            Reflectance: 0f32,
            RightParamA: -0.5f32,
            RightParamB: 0.5f32,
            RightSurface: enums::SurfaceType::Smooth,
            RightSurfaceInput: enums::InputType::NoInput,
            RootPriority: 0i32,
            RotVelocity: Vector3::new(0f32, 0f32, 0f32),
            TopParamA: -0.5f32,
            TopParamB: 0.5f32,
            TopSurface: enums::SurfaceType::Studs,
            TopSurfaceInput: enums::InputType::NoInput,
            Transparency: 0f32,
            Velocity: Vector3::new(0f32, 0f32, 0f32),
        };
        let superclass = FormFactorPart { superclass };
        let superclass = Part { superclass };
        Self {
            superclass,
            TeamColor: BrickColor::from_number(194u16).unwrap(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FlagStandService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(FlagStandService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FlangeSoundEffect {
    #[doc(hidden)]
    pub superclass: SoundEffect,
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
impl_inherits!(FlangeSoundEffect, SoundEffect);
impl Default for FlangeSoundEffect {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundEffect {
            superclass,
            Enabled: true,
            Priority: 0i32,
        };
        Self {
            superclass,
            Depth: 0.45f32,
            Mix: 0.85f32,
            Rate: 5f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FloatCurve {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ValuesAndTimes: BinaryString,
}
impl_inherits!(FloatCurve, Instance);
impl Default for FloatCurve {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ValuesAndTimes: b"\x02\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0".as_slice().into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FloorWire {
    #[doc(hidden)]
    pub superclass: GuiBase3d,
    pub CycleOffset: f32,
    pub From: Ref,
    pub StudsBetweenTextures: f32,
    pub Texture: ContentId,
    pub TextureSize: Vector2,
    pub To: Ref,
    pub Velocity: f32,
    pub WireRadius: f32,
}
impl_inherits!(FloorWire, GuiBase3d);
impl Default for FloorWire {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase3d {
            superclass,
            Color3: Color3::new(0.050980393f32, 0.4117647f32, 0.6745098f32),
            Transparency: 0f32,
            Visible: true,
        };
        Self {
            superclass,
            CycleOffset: 0f32,
            From: Ref::none(),
            StudsBetweenTextures: 4f32,
            Texture: "".into(),
            TextureSize: Vector2::new(1f32, 1f32),
            To: Ref::none(),
            Velocity: 2f32,
            WireRadius: 0.0625f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FluidForceSensor {
    #[doc(hidden)]
    pub superclass: SensorBase,
}
impl_inherits!(FluidForceSensor, SensorBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FlyweightService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(FlyweightService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Folder {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(Folder, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ForceField {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Visible: bool,
}
impl_inherits!(ForceField, Instance);
impl Default for ForceField {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Visible: true,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FormFactorPart {
    #[doc(hidden)]
    pub superclass: BasePart,
}
impl_inherits!(FormFactorPart, BasePart);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Frame {
    #[doc(hidden)]
    pub superclass: GuiObject,
    pub Style: enums::FrameStyle,
}
impl_inherits!(Frame, GuiObject);
impl Default for Frame {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase2d {
            superclass,
            AutoLocalize: true,
            RootLocalizationTable: Ref::none(),
            SelectionBehaviorDown: enums::SelectionBehavior::Escape,
            SelectionBehaviorLeft: enums::SelectionBehavior::Escape,
            SelectionBehaviorRight: enums::SelectionBehavior::Escape,
            SelectionBehaviorUp: enums::SelectionBehavior::Escape,
            SelectionGroup: false,
        };
        let superclass = GuiObject {
            superclass,
            Active: false,
            AnchorPoint: Vector2::new(0f32, 0f32),
            AutomaticSize: enums::AutomaticSize::None,
            BackgroundColor3: Color3::new(0.6392157f32, 0.63529414f32, 0.64705884f32),
            BackgroundTransparency: 0f32,
            BorderColor3: Color3::new(0.105882354f32, 0.16470589f32, 0.20784314f32),
            BorderMode: enums::BorderMode::Outline,
            BorderSizePixel: 1i32,
            ClipsDescendants: false,
            Draggable: false,
            Interactable: true,
            LayoutOrder: 0i32,
            NextSelectionDown: Ref::none(),
            NextSelectionLeft: Ref::none(),
            NextSelectionRight: Ref::none(),
            NextSelectionUp: Ref::none(),
            Position: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            Rotation: 0f32,
            Selectable: false,
            SelectionImageObject: Ref::none(),
            SelectionOrder: 0i32,
            Size: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            SizeConstraint: enums::SizeConstraint::RelativeXY,
            Visible: true,
            ZIndex: 1i32,
        };
        Self {
            superclass,
            Style: enums::FrameStyle::Custom,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FriendPages {
    #[doc(hidden)]
    pub superclass: Pages,
}
impl_inherits!(FriendPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FriendService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(FriendService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FunctionalTest {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Description: String,
    pub HasMigratedSettingsToTestService: bool,
}
impl_inherits!(FunctionalTest, Instance);
impl Default for FunctionalTest {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Description: "?".to_owned(),
            HasMigratedSettingsToTestService: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GamePassService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(GamePassService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GameSettings {
    #[doc(hidden)]
    pub superclass: Instance,
    pub VideoCaptureEnabled: bool,
}
impl_inherits!(GameSettings, Instance);
impl Default for GameSettings {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            VideoCaptureEnabled: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GamepadService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub GamepadCursorEnabled: bool,
}
impl_inherits!(GamepadService, Instance);
impl Default for GamepadService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            GamepadCursorEnabled: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GenerationService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(GenerationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GenericChallengeService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(GenericChallengeService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GenericSettings {
    #[doc(hidden)]
    pub superclass: ServiceProvider,
}
impl_inherits!(GenericSettings, ServiceProvider);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Geometry {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(Geometry, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GeometryService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(GeometryService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GetTextBoundsParams {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Font: Font,
    pub RichText: bool,
    pub Size: f32,
    pub Text: String,
    pub Width: f32,
}
impl_inherits!(GetTextBoundsParams, Instance);
impl Default for GetTextBoundsParams {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Font: Font::new(
                "rbxasset://fonts/families/SourceSansPro.json",
                FontWeight::Regular,
                FontStyle::Normal,
            ),
            RichText: false,
            Size: 0f32,
            Text: "".to_owned(),
            Width: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GlobalDataStore {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(GlobalDataStore, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GlobalSettings {
    #[doc(hidden)]
    pub superclass: GenericSettings,
}
impl_inherits!(GlobalSettings, GenericSettings);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Glue {
    #[doc(hidden)]
    pub superclass: JointInstance,
    pub F0: Vector3,
    pub F1: Vector3,
    pub F2: Vector3,
    pub F3: Vector3,
}
impl_inherits!(Glue, JointInstance);
impl Default for Glue {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = JointInstance {
            superclass,
            C0: CFrame::identity(),
            C1: CFrame::identity(),
            Enabled: true,
            Part0: Ref::none(),
            Part1: Ref::none(),
        };
        Self {
            superclass,
            F0: Vector3::new(0f32, 0f32, 0f32),
            F1: Vector3::new(0f32, 0f32, 0f32),
            F2: Vector3::new(0f32, 0f32, 0f32),
            F3: Vector3::new(0f32, 0f32, 0f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GroundController {
    #[doc(hidden)]
    pub superclass: ControllerBase,
    pub AccelerationLean: f32,
    pub AccelerationTime: f32,
    pub BalanceMaxTorque: f32,
    pub BalanceSpeed: f32,
    pub DecelerationTime: f32,
    pub Friction: f32,
    pub FrictionWeight: f32,
    pub GroundOffset: f32,
    pub StandForce: f32,
    pub StandSpeed: f32,
    pub TurnSpeedFactor: f32,
}
impl_inherits!(GroundController, ControllerBase);
impl Default for GroundController {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ControllerBase {
            superclass,
            BalanceRigidityEnabled: false,
            MoveSpeedFactor: 1f32,
        };
        Self {
            superclass,
            AccelerationLean: 1f32,
            AccelerationTime: 0f32,
            BalanceMaxTorque: 10000f32,
            BalanceSpeed: 100f32,
            DecelerationTime: 0f32,
            Friction: 2f32,
            FrictionWeight: 1f32,
            GroundOffset: 1f32,
            StandForce: 10000f32,
            StandSpeed: 100f32,
            TurnSpeedFactor: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GroupImportData {
    #[doc(hidden)]
    pub superclass: BaseImportData,
    pub Anchored: bool,
    pub ImportAsModelAsset: bool,
    pub InsertInWorkspace: bool,
}
impl_inherits!(GroupImportData, BaseImportData);
impl Default for GroupImportData {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BaseImportData {
            superclass,
            ImportName: "".to_owned(),
            ShouldImport: false,
        };
        Self {
            superclass,
            Anchored: false,
            ImportAsModelAsset: false,
            InsertInWorkspace: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GroupService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(GroupService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GuiBase {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(GuiBase, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiBase2d {
    #[doc(hidden)]
    pub superclass: GuiBase,
    pub AutoLocalize: bool,
    pub RootLocalizationTable: Ref,
    pub SelectionBehaviorDown: enums::SelectionBehavior,
    pub SelectionBehaviorLeft: enums::SelectionBehavior,
    pub SelectionBehaviorRight: enums::SelectionBehavior,
    pub SelectionBehaviorUp: enums::SelectionBehavior,
    pub SelectionGroup: bool,
}
impl_inherits!(GuiBase2d, GuiBase);
impl Default for GuiBase2d {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        Self {
            superclass,
            AutoLocalize: false,
            RootLocalizationTable: Ref::none(),
            SelectionBehaviorDown: enums::SelectionBehavior::Escape,
            SelectionBehaviorLeft: enums::SelectionBehavior::Escape,
            SelectionBehaviorRight: enums::SelectionBehavior::Escape,
            SelectionBehaviorUp: enums::SelectionBehavior::Escape,
            SelectionGroup: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiBase3d {
    #[doc(hidden)]
    pub superclass: GuiBase,
    pub Color3: Color3,
    pub Transparency: f32,
    pub Visible: bool,
}
impl_inherits!(GuiBase3d, GuiBase);
impl Default for GuiBase3d {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        Self {
            superclass,
            Color3: Color3::new(0f32, 0f32, 0f32),
            Transparency: 0f32,
            Visible: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiButton {
    #[doc(hidden)]
    pub superclass: GuiObject,
    pub AutoButtonColor: bool,
    pub HoverHapticEffect: Ref,
    pub Modal: bool,
    pub PressHapticEffect: Ref,
    pub Selected: bool,
    pub Style: enums::ButtonStyle,
}
impl_inherits!(GuiButton, GuiObject);
impl Default for GuiButton {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase2d {
            superclass,
            AutoLocalize: false,
            RootLocalizationTable: Ref::none(),
            SelectionBehaviorDown: enums::SelectionBehavior::Escape,
            SelectionBehaviorLeft: enums::SelectionBehavior::Escape,
            SelectionBehaviorRight: enums::SelectionBehavior::Escape,
            SelectionBehaviorUp: enums::SelectionBehavior::Escape,
            SelectionGroup: false,
        };
        let superclass = GuiObject {
            superclass,
            Active: false,
            AnchorPoint: Vector2::new(0f32, 0f32),
            AutomaticSize: enums::AutomaticSize::None,
            BackgroundColor3: Color3::new(0f32, 0f32, 0f32),
            BackgroundTransparency: 0f32,
            BorderColor3: Color3::new(0f32, 0f32, 0f32),
            BorderMode: enums::BorderMode::Outline,
            BorderSizePixel: 0i32,
            ClipsDescendants: false,
            Draggable: false,
            Interactable: false,
            LayoutOrder: 0i32,
            NextSelectionDown: Ref::none(),
            NextSelectionLeft: Ref::none(),
            NextSelectionRight: Ref::none(),
            NextSelectionUp: Ref::none(),
            Position: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            Rotation: 0f32,
            Selectable: false,
            SelectionImageObject: Ref::none(),
            SelectionOrder: 0i32,
            Size: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            SizeConstraint: enums::SizeConstraint::RelativeXY,
            Visible: false,
            ZIndex: 0i32,
        };
        Self {
            superclass,
            AutoButtonColor: false,
            HoverHapticEffect: Ref::none(),
            Modal: false,
            PressHapticEffect: Ref::none(),
            Selected: false,
            Style: enums::ButtonStyle::Custom,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GuiLabel {
    #[doc(hidden)]
    pub superclass: GuiObject,
}
impl_inherits!(GuiLabel, GuiObject);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GuiMain {
    #[doc(hidden)]
    pub superclass: ScreenGui,
}
impl_inherits!(GuiMain, ScreenGui);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiObject {
    #[doc(hidden)]
    pub superclass: GuiBase2d,
    pub Active: bool,
    pub AnchorPoint: Vector2,
    pub AutomaticSize: enums::AutomaticSize,
    pub BackgroundColor3: Color3,
    pub BackgroundTransparency: f32,
    pub BorderColor3: Color3,
    pub BorderMode: enums::BorderMode,
    pub BorderSizePixel: i32,
    pub ClipsDescendants: bool,
    pub Draggable: bool,
    pub Interactable: bool,
    pub LayoutOrder: i32,
    pub NextSelectionDown: Ref,
    pub NextSelectionLeft: Ref,
    pub NextSelectionRight: Ref,
    pub NextSelectionUp: Ref,
    pub Position: UDim2,
    pub Rotation: f32,
    pub Selectable: bool,
    pub SelectionImageObject: Ref,
    pub SelectionOrder: i32,
    pub Size: UDim2,
    pub SizeConstraint: enums::SizeConstraint,
    pub Visible: bool,
    pub ZIndex: i32,
}
impl_inherits!(GuiObject, GuiBase2d);
impl Default for GuiObject {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase2d {
            superclass,
            AutoLocalize: false,
            RootLocalizationTable: Ref::none(),
            SelectionBehaviorDown: enums::SelectionBehavior::Escape,
            SelectionBehaviorLeft: enums::SelectionBehavior::Escape,
            SelectionBehaviorRight: enums::SelectionBehavior::Escape,
            SelectionBehaviorUp: enums::SelectionBehavior::Escape,
            SelectionGroup: false,
        };
        Self {
            superclass,
            Active: false,
            AnchorPoint: Vector2::new(0f32, 0f32),
            AutomaticSize: enums::AutomaticSize::None,
            BackgroundColor3: Color3::new(0f32, 0f32, 0f32),
            BackgroundTransparency: 0f32,
            BorderColor3: Color3::new(0f32, 0f32, 0f32),
            BorderMode: enums::BorderMode::Outline,
            BorderSizePixel: 0i32,
            ClipsDescendants: false,
            Draggable: false,
            Interactable: false,
            LayoutOrder: 0i32,
            NextSelectionDown: Ref::none(),
            NextSelectionLeft: Ref::none(),
            NextSelectionRight: Ref::none(),
            NextSelectionUp: Ref::none(),
            Position: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            Rotation: 0f32,
            Selectable: false,
            SelectionImageObject: Ref::none(),
            SelectionOrder: 0i32,
            Size: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            SizeConstraint: enums::SizeConstraint::RelativeXY,
            Visible: false,
            ZIndex: 0i32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AutoSelectGuiEnabled: bool,
    pub GuiNavigationEnabled: bool,
    pub SelectedObject: Ref,
}
impl_inherits!(GuiService, Instance);
impl Default for GuiService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AutoSelectGuiEnabled: false,
            GuiNavigationEnabled: false,
            SelectedObject: Ref::none(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GuidRegistryService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(GuidRegistryService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HSRDataContentProvider {
    #[doc(hidden)]
    pub superclass: CacheableContentProvider,
}
impl_inherits!(HSRDataContentProvider, CacheableContentProvider);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HandRigDescription {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Index1: Ref,
    pub Index1TposeAdjustment: CFrame,
    pub Index2: Ref,
    pub Index2TposeAdjustment: CFrame,
    pub Index3: Ref,
    pub Index3TposeAdjustment: CFrame,
    pub IndexRange: Vector3,
    pub IndexSize: f32,
    pub Middle1: Ref,
    pub Middle1TposeAdjustment: CFrame,
    pub Middle2: Ref,
    pub Middle2TposeAdjustment: CFrame,
    pub Middle3: Ref,
    pub Middle3TposeAdjustment: CFrame,
    pub MiddleRange: Vector3,
    pub MiddleSize: f32,
    pub Pinky1: Ref,
    pub Pinky1TposeAdjustment: CFrame,
    pub Pinky2: Ref,
    pub Pinky2TposeAdjustment: CFrame,
    pub Pinky3: Ref,
    pub Pinky3TposeAdjustment: CFrame,
    pub PinkyRange: Vector3,
    pub PinkySize: f32,
    pub Ring1: Ref,
    pub Ring1TposeAdjustment: CFrame,
    pub Ring2: Ref,
    pub Ring2TposeAdjustment: CFrame,
    pub Ring3: Ref,
    pub Ring3TposeAdjustment: CFrame,
    pub RingRange: Vector3,
    pub RingSize: f32,
    pub Side: enums::HandRigDescriptionSide,
    pub Thumb1: Ref,
    pub Thumb1TposeAdjustment: CFrame,
    pub Thumb2: Ref,
    pub Thumb2TposeAdjustment: CFrame,
    pub Thumb3: Ref,
    pub Thumb3TposeAdjustment: CFrame,
    pub ThumbRange: Vector3,
    pub ThumbSize: f32,
}
impl_inherits!(HandRigDescription, Instance);
impl Default for HandRigDescription {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Index1: Ref::none(),
            Index1TposeAdjustment: CFrame::identity(),
            Index2: Ref::none(),
            Index2TposeAdjustment: CFrame::identity(),
            Index3: Ref::none(),
            Index3TposeAdjustment: CFrame::identity(),
            IndexRange: Vector3::new(0f32, 0f32, 0f32),
            IndexSize: 0f32,
            Middle1: Ref::none(),
            Middle1TposeAdjustment: CFrame::identity(),
            Middle2: Ref::none(),
            Middle2TposeAdjustment: CFrame::identity(),
            Middle3: Ref::none(),
            Middle3TposeAdjustment: CFrame::identity(),
            MiddleRange: Vector3::new(0f32, 0f32, 0f32),
            MiddleSize: 0f32,
            Pinky1: Ref::none(),
            Pinky1TposeAdjustment: CFrame::identity(),
            Pinky2: Ref::none(),
            Pinky2TposeAdjustment: CFrame::identity(),
            Pinky3: Ref::none(),
            Pinky3TposeAdjustment: CFrame::identity(),
            PinkyRange: Vector3::new(0f32, 0f32, 0f32),
            PinkySize: 0f32,
            Ring1: Ref::none(),
            Ring1TposeAdjustment: CFrame::identity(),
            Ring2: Ref::none(),
            Ring2TposeAdjustment: CFrame::identity(),
            Ring3: Ref::none(),
            Ring3TposeAdjustment: CFrame::identity(),
            RingRange: Vector3::new(0f32, 0f32, 0f32),
            RingSize: 0f32,
            Side: enums::HandRigDescriptionSide::None,
            Thumb1: Ref::none(),
            Thumb1TposeAdjustment: CFrame::identity(),
            Thumb2: Ref::none(),
            Thumb2TposeAdjustment: CFrame::identity(),
            Thumb3: Ref::none(),
            Thumb3TposeAdjustment: CFrame::identity(),
            ThumbRange: Vector3::new(0f32, 0f32, 0f32),
            ThumbSize: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HandleAdornment {
    #[doc(hidden)]
    pub superclass: PVAdornment,
    pub AdornCullingMode: enums::AdornCullingMode,
    pub AlwaysOnTop: bool,
    pub CFrame: CFrame,
    pub SizeRelativeOffset: Vector3,
    pub ZIndex: i32,
}
impl_inherits!(HandleAdornment, PVAdornment);
impl Default for HandleAdornment {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase3d {
            superclass,
            Color3: Color3::new(0f32, 0f32, 0f32),
            Transparency: 0f32,
            Visible: false,
        };
        let superclass = PVAdornment {
            superclass,
            Adornee: Ref::none(),
        };
        Self {
            superclass,
            AdornCullingMode: enums::AdornCullingMode::Automatic,
            AlwaysOnTop: false,
            CFrame: CFrame::identity(),
            SizeRelativeOffset: Vector3::new(0f32, 0f32, 0f32),
            ZIndex: 0i32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Handles {
    #[doc(hidden)]
    pub superclass: HandlesBase,
    pub Faces: Faces,
    pub Style: enums::HandlesStyle,
}
impl_inherits!(Handles, HandlesBase);
impl Default for Handles {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase3d {
            superclass,
            Color3: Color3::new(0.050980393f32, 0.4117647f32, 0.6745098f32),
            Transparency: 0f32,
            Visible: true,
        };
        let superclass = PartAdornment {
            superclass,
            Adornee: Ref::none(),
        };
        let superclass = HandlesBase { superclass };
        Self {
            superclass,
            Faces: Faces::all(),
            Style: enums::HandlesStyle::Resize,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HandlesBase {
    #[doc(hidden)]
    pub superclass: PartAdornment,
}
impl_inherits!(HandlesBase, PartAdornment);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HapticEffect {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Looped: bool,
    pub Position: Vector3,
    pub Radius: f32,
    pub Type: enums::HapticEffectType,
    pub Waveform: Ref,
    pub WaveformData: BinaryString,
}
impl_inherits!(HapticEffect, Instance);
impl Default for HapticEffect {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Looped: false,
            Position: Vector3::new(0f32, 0f32, 0f32),
            Radius: 3f32,
            Type: enums::HapticEffectType::UIClick,
            Waveform: Ref::none(),
            WaveformData: b"".as_slice().into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HapticService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(HapticService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HarmonyService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(HarmonyService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Hat {
    #[doc(hidden)]
    pub superclass: Accoutrement,
}
impl_inherits!(Hat, Accoutrement);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HeapProfilerService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(HeapProfilerService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HeatmapService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(HeatmapService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HeightmapImporterService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(HeightmapImporterService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HiddenSurfaceRemovalAsset {
    #[doc(hidden)]
    pub superclass: Instance,
    pub HsrData: BinaryString,
    pub HsrMeshIdData: BinaryString,
}
impl_inherits!(HiddenSurfaceRemovalAsset, Instance);
impl Default for HiddenSurfaceRemovalAsset {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            HsrData: b"".as_slice().into(),
            HsrMeshIdData: b"".as_slice().into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Highlight {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Adornee: Ref,
    pub DepthMode: enums::HighlightDepthMode,
    pub Enabled: bool,
    pub FillColor: Color3,
    pub FillTransparency: f32,
    pub OutlineColor: Color3,
    pub OutlineTransparency: f32,
}
impl_inherits!(Highlight, Instance);
impl Default for Highlight {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Adornee: Ref::none(),
            DepthMode: enums::HighlightDepthMode::AlwaysOnTop,
            Enabled: true,
            FillColor: Color3::new(1f32, 0f32, 0f32),
            FillTransparency: 0.5f32,
            OutlineColor: Color3::new(1f32, 1f32, 1f32),
            OutlineTransparency: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HingeConstraint {
    #[doc(hidden)]
    pub superclass: Constraint,
    pub ActuatorType: enums::ActuatorType,
    pub AngularResponsiveness: f32,
    pub AngularSpeed: f32,
    pub AngularVelocity: f32,
    pub LimitsEnabled: bool,
    pub LowerAngle: f32,
    pub MotorMaxAcceleration: f32,
    pub MotorMaxTorque: f32,
    pub Radius: f32,
    pub Restitution: f32,
    pub ServoMaxTorque: f32,
    pub SoftlockServoUponReachingTarget: bool,
    pub TargetAngle: f32,
    pub UpperAngle: f32,
}
impl_inherits!(HingeConstraint, Constraint);
impl Default for HingeConstraint {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Constraint {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            Color: BrickColor::from_number(1009u16).unwrap(),
            Enabled: true,
            Visible: false,
        };
        Self {
            superclass,
            ActuatorType: enums::ActuatorType::None,
            AngularResponsiveness: 45f32,
            AngularSpeed: 0f32,
            AngularVelocity: 0f32,
            LimitsEnabled: false,
            LowerAngle: -45f32,
            MotorMaxAcceleration: 500000f32,
            MotorMaxTorque: 0f32,
            Radius: 0.15f32,
            Restitution: 0f32,
            ServoMaxTorque: 0f32,
            SoftlockServoUponReachingTarget: false,
            TargetAngle: 0f32,
            UpperAngle: 45f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Hint {
    #[doc(hidden)]
    pub superclass: Message,
}
impl_inherits!(Hint, Message);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Hole {
    #[doc(hidden)]
    pub superclass: Feature,
}
impl_inherits!(Hole, Feature);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Hopper {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(Hopper, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HopperBin {
    #[doc(hidden)]
    pub superclass: BackpackItem,
    pub Active: bool,
    pub BinType: enums::BinType,
}
impl_inherits!(HopperBin, BackpackItem);
impl Default for HopperBin {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PVInstance { superclass };
        let superclass = Model {
            superclass,
            LevelOfDetail: enums::ModelLevelOfDetail::Automatic,
            ModelMeshCFrame: CFrame::identity(),
            ModelMeshData: SharedString::new(b"".to_vec()),
            ModelMeshSize: Vector3::new(0f32, 0f32, 0f32),
            ModelStreamingMode: enums::ModelStreamingMode::Default,
            NeedsPivotMigration: false,
            PrimaryPart: Ref::none(),
            SlimHash: SharedString::new(b"".to_vec()),
            WorldPivotData: None,
        };
        let superclass = BackpackItem {
            superclass,
            TextureContent: Content::none(),
        };
        Self {
            superclass,
            Active: false,
            BinType: enums::BinType::Script,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HttpRbxApiService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(HttpRbxApiService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HttpRequest {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(HttpRequest, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HttpService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub HttpEnabled: bool,
}
impl_inherits!(HttpService, Instance);
impl Default for HttpService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            HttpEnabled: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Humanoid {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AutoJumpEnabled: bool,
    pub AutoRotate: bool,
    pub AutomaticScalingEnabled: bool,
    pub BreakJointsOnDeath: bool,
    pub CollisionType: enums::HumanoidCollisionType,
    pub DisplayDistanceType: enums::HumanoidDisplayDistanceType,
    pub DisplayName: String,
    pub EvaluateStateMachine: bool,
    pub HealthDisplayDistance: f32,
    pub HealthDisplayType: enums::HumanoidHealthDisplayType,
    pub HealthXml: f32,
    pub HipHeight: f32,
    pub InternalBodyScale: Vector3,
    pub InternalHeadScale: f32,
    pub JumpHeight: f32,
    pub JumpPower: f32,
    pub MaxHealth: f32,
    pub MaxSlopeAngle: f32,
    pub NameDisplayDistance: f32,
    pub NameOcclusion: enums::NameOcclusion,
    pub RequiresNeck: bool,
    pub RigType: enums::HumanoidRigType,
    pub UseJumpPower: bool,
    pub WalkSpeed: f32,
}
impl_inherits!(Humanoid, Instance);
impl Default for Humanoid {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AutoJumpEnabled: true,
            AutoRotate: true,
            AutomaticScalingEnabled: true,
            BreakJointsOnDeath: true,
            CollisionType: enums::HumanoidCollisionType::OuterBox,
            DisplayDistanceType: enums::HumanoidDisplayDistanceType::Viewer,
            DisplayName: "".to_owned(),
            EvaluateStateMachine: true,
            HealthDisplayDistance: 100f32,
            HealthDisplayType: enums::HumanoidHealthDisplayType::DisplayWhenDamaged,
            HealthXml: 100f32,
            HipHeight: 0f32,
            InternalBodyScale: Vector3::new(1f32, 1f32, 1f32),
            InternalHeadScale: 1f32,
            JumpHeight: 7.2f32,
            JumpPower: 50f32,
            MaxHealth: 100f32,
            MaxSlopeAngle: 89f32,
            NameDisplayDistance: 100f32,
            NameOcclusion: enums::NameOcclusion::OccludeAll,
            RequiresNeck: true,
            RigType: enums::HumanoidRigType::R6,
            UseJumpPower: true,
            WalkSpeed: 16f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HumanoidController {
    #[doc(hidden)]
    pub superclass: Controller,
}
impl_inherits!(HumanoidController, Controller);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HumanoidDescription {
    #[doc(hidden)]
    pub superclass: Instance,
    pub BodyTypeScale: f32,
    pub ClimbAnimation: i64,
    pub DepthScale: f32,
    pub EmotesDataInternal: String,
    pub EquippedEmotesDataInternal: String,
    pub Face: i64,
    pub FallAnimation: i64,
    pub GraphicTShirt: i64,
    pub HeadScale: f32,
    pub HeightScale: f32,
    pub IdleAnimation: i64,
    pub JumpAnimation: i64,
    pub MoodAnimation: i64,
    pub Pants: i64,
    pub ProportionScale: f32,
    pub RunAnimation: i64,
    pub Shirt: i64,
    pub SwimAnimation: i64,
    pub WalkAnimation: i64,
    pub WidthScale: f32,
}
impl_inherits!(HumanoidDescription, Instance);
impl Default for HumanoidDescription {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            BodyTypeScale: 0.3f32,
            ClimbAnimation: 0i64,
            DepthScale: 1f32,
            EmotesDataInternal: "".to_owned(),
            EquippedEmotesDataInternal: "".to_owned(),
            Face: 0i64,
            FallAnimation: 0i64,
            GraphicTShirt: 0i64,
            HeadScale: 1f32,
            HeightScale: 1f32,
            IdleAnimation: 0i64,
            JumpAnimation: 0i64,
            MoodAnimation: 0i64,
            Pants: 0i64,
            ProportionScale: 1f32,
            RunAnimation: 0i64,
            Shirt: 0i64,
            SwimAnimation: 0i64,
            WalkAnimation: 0i64,
            WidthScale: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HumanoidRigDescription {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Chest: Ref,
    pub ChestRangeMax: Vector3,
    pub ChestRangeMin: Vector3,
    pub ChestSize: f32,
    pub ChestTposeAdjustment: CFrame,
    pub HeadBase: Ref,
    pub HeadBaseRangeMax: Vector3,
    pub HeadBaseRangeMin: Vector3,
    pub HeadBaseSize: f32,
    pub HeadBaseTposeAdjustment: CFrame,
    pub LeftAnkle: Ref,
    pub LeftAnkleRangeMax: Vector3,
    pub LeftAnkleRangeMin: Vector3,
    pub LeftAnkleSize: f32,
    pub LeftAnkleTposeAdjustment: CFrame,
    pub LeftClavicle: Ref,
    pub LeftClavicleRangeMax: Vector3,
    pub LeftClavicleRangeMin: Vector3,
    pub LeftClavicleSize: f32,
    pub LeftClavicleTposeAdjustment: CFrame,
    pub LeftElbow: Ref,
    pub LeftElbowRangeMax: Vector3,
    pub LeftElbowRangeMin: Vector3,
    pub LeftElbowSize: f32,
    pub LeftElbowTposeAdjustment: CFrame,
    pub LeftHip: Ref,
    pub LeftHipRangeMax: Vector3,
    pub LeftHipRangeMin: Vector3,
    pub LeftHipSize: f32,
    pub LeftHipTposeAdjustment: CFrame,
    pub LeftKnee: Ref,
    pub LeftKneeRangeMax: Vector3,
    pub LeftKneeRangeMin: Vector3,
    pub LeftKneeSize: f32,
    pub LeftKneeTposeAdjustment: CFrame,
    pub LeftShoulder: Ref,
    pub LeftShoulderRangeMax: Vector3,
    pub LeftShoulderRangeMin: Vector3,
    pub LeftShoulderSize: f32,
    pub LeftShoulderTposeAdjustment: CFrame,
    pub LeftToes: Ref,
    pub LeftToesRangeMax: Vector3,
    pub LeftToesRangeMin: Vector3,
    pub LeftToesSize: f32,
    pub LeftToesTposeAdjustment: CFrame,
    pub LeftWrist: Ref,
    pub LeftWristRangeMax: Vector3,
    pub LeftWristRangeMin: Vector3,
    pub LeftWristSize: f32,
    pub LeftWristTposeAdjustment: CFrame,
    pub Neck: Ref,
    pub NeckRangeMax: Vector3,
    pub NeckRangeMin: Vector3,
    pub NeckSize: f32,
    pub NeckTposeAdjustment: CFrame,
    pub OriginOffset: CFrame,
    pub Pelvis: Ref,
    pub PelvisRangeMax: Vector3,
    pub PelvisRangeMin: Vector3,
    pub PelvisSize: f32,
    pub PelvisTposeAdjustment: CFrame,
    pub RightAnkle: Ref,
    pub RightAnkleRangeMax: Vector3,
    pub RightAnkleRangeMin: Vector3,
    pub RightAnkleSize: f32,
    pub RightAnkleTposeAdjustment: CFrame,
    pub RightClavicle: Ref,
    pub RightClavicleRangeMax: Vector3,
    pub RightClavicleRangeMin: Vector3,
    pub RightClavicleSize: f32,
    pub RightClavicleTposeAdjustment: CFrame,
    pub RightElbow: Ref,
    pub RightElbowRangeMax: Vector3,
    pub RightElbowRangeMin: Vector3,
    pub RightElbowSize: f32,
    pub RightElbowTposeAdjustment: CFrame,
    pub RightHip: Ref,
    pub RightHipRangeMax: Vector3,
    pub RightHipRangeMin: Vector3,
    pub RightHipSize: f32,
    pub RightHipTposeAdjustment: CFrame,
    pub RightKnee: Ref,
    pub RightKneeRangeMax: Vector3,
    pub RightKneeRangeMin: Vector3,
    pub RightKneeSize: f32,
    pub RightKneeTposeAdjustment: CFrame,
    pub RightShoulder: Ref,
    pub RightShoulderRangeMax: Vector3,
    pub RightShoulderRangeMin: Vector3,
    pub RightShoulderSize: f32,
    pub RightShoulderTposeAdjustment: CFrame,
    pub RightToes: Ref,
    pub RightToesRangeMax: Vector3,
    pub RightToesRangeMin: Vector3,
    pub RightToesSize: f32,
    pub RightToesTposeAdjustment: CFrame,
    pub RightWrist: Ref,
    pub RightWristRangeMax: Vector3,
    pub RightWristRangeMin: Vector3,
    pub RightWristSize: f32,
    pub RightWristTposeAdjustment: CFrame,
    pub Root: Ref,
    pub RootRangeMax: Vector3,
    pub RootRangeMin: Vector3,
    pub RootSize: f32,
    pub RootTposeAdjustment: CFrame,
    pub Waist: Ref,
    pub WaistRangeMax: Vector3,
    pub WaistRangeMin: Vector3,
    pub WaistSize: f32,
    pub WaistTposeAdjustment: CFrame,
}
impl_inherits!(HumanoidRigDescription, Instance);
impl Default for HumanoidRigDescription {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Chest: Ref::none(),
            ChestRangeMax: Vector3::new(0f32, 0f32, 0f32),
            ChestRangeMin: Vector3::new(0f32, 0f32, 0f32),
            ChestSize: 0f32,
            ChestTposeAdjustment: CFrame::identity(),
            HeadBase: Ref::none(),
            HeadBaseRangeMax: Vector3::new(0f32, 0f32, 0f32),
            HeadBaseRangeMin: Vector3::new(0f32, 0f32, 0f32),
            HeadBaseSize: 0f32,
            HeadBaseTposeAdjustment: CFrame::identity(),
            LeftAnkle: Ref::none(),
            LeftAnkleRangeMax: Vector3::new(0f32, 0f32, 0f32),
            LeftAnkleRangeMin: Vector3::new(0f32, 0f32, 0f32),
            LeftAnkleSize: 0f32,
            LeftAnkleTposeAdjustment: CFrame::identity(),
            LeftClavicle: Ref::none(),
            LeftClavicleRangeMax: Vector3::new(0f32, 0f32, 0f32),
            LeftClavicleRangeMin: Vector3::new(0f32, 0f32, 0f32),
            LeftClavicleSize: 0f32,
            LeftClavicleTposeAdjustment: CFrame::identity(),
            LeftElbow: Ref::none(),
            LeftElbowRangeMax: Vector3::new(0f32, 0f32, 0f32),
            LeftElbowRangeMin: Vector3::new(0f32, 0f32, 0f32),
            LeftElbowSize: 0f32,
            LeftElbowTposeAdjustment: CFrame::identity(),
            LeftHip: Ref::none(),
            LeftHipRangeMax: Vector3::new(0f32, 0f32, 0f32),
            LeftHipRangeMin: Vector3::new(0f32, 0f32, 0f32),
            LeftHipSize: 0f32,
            LeftHipTposeAdjustment: CFrame::identity(),
            LeftKnee: Ref::none(),
            LeftKneeRangeMax: Vector3::new(0f32, 0f32, 0f32),
            LeftKneeRangeMin: Vector3::new(0f32, 0f32, 0f32),
            LeftKneeSize: 0f32,
            LeftKneeTposeAdjustment: CFrame::identity(),
            LeftShoulder: Ref::none(),
            LeftShoulderRangeMax: Vector3::new(0f32, 0f32, 0f32),
            LeftShoulderRangeMin: Vector3::new(0f32, 0f32, 0f32),
            LeftShoulderSize: 0f32,
            LeftShoulderTposeAdjustment: CFrame::identity(),
            LeftToes: Ref::none(),
            LeftToesRangeMax: Vector3::new(0f32, 0f32, 0f32),
            LeftToesRangeMin: Vector3::new(0f32, 0f32, 0f32),
            LeftToesSize: 0f32,
            LeftToesTposeAdjustment: CFrame::identity(),
            LeftWrist: Ref::none(),
            LeftWristRangeMax: Vector3::new(0f32, 0f32, 0f32),
            LeftWristRangeMin: Vector3::new(0f32, 0f32, 0f32),
            LeftWristSize: 0f32,
            LeftWristTposeAdjustment: CFrame::identity(),
            Neck: Ref::none(),
            NeckRangeMax: Vector3::new(0f32, 0f32, 0f32),
            NeckRangeMin: Vector3::new(0f32, 0f32, 0f32),
            NeckSize: 0f32,
            NeckTposeAdjustment: CFrame::identity(),
            OriginOffset: CFrame::identity(),
            Pelvis: Ref::none(),
            PelvisRangeMax: Vector3::new(0f32, 0f32, 0f32),
            PelvisRangeMin: Vector3::new(0f32, 0f32, 0f32),
            PelvisSize: 0f32,
            PelvisTposeAdjustment: CFrame::identity(),
            RightAnkle: Ref::none(),
            RightAnkleRangeMax: Vector3::new(0f32, 0f32, 0f32),
            RightAnkleRangeMin: Vector3::new(0f32, 0f32, 0f32),
            RightAnkleSize: 0f32,
            RightAnkleTposeAdjustment: CFrame::identity(),
            RightClavicle: Ref::none(),
            RightClavicleRangeMax: Vector3::new(0f32, 0f32, 0f32),
            RightClavicleRangeMin: Vector3::new(0f32, 0f32, 0f32),
            RightClavicleSize: 0f32,
            RightClavicleTposeAdjustment: CFrame::identity(),
            RightElbow: Ref::none(),
            RightElbowRangeMax: Vector3::new(0f32, 0f32, 0f32),
            RightElbowRangeMin: Vector3::new(0f32, 0f32, 0f32),
            RightElbowSize: 0f32,
            RightElbowTposeAdjustment: CFrame::identity(),
            RightHip: Ref::none(),
            RightHipRangeMax: Vector3::new(0f32, 0f32, 0f32),
            RightHipRangeMin: Vector3::new(0f32, 0f32, 0f32),
            RightHipSize: 0f32,
            RightHipTposeAdjustment: CFrame::identity(),
            RightKnee: Ref::none(),
            RightKneeRangeMax: Vector3::new(0f32, 0f32, 0f32),
            RightKneeRangeMin: Vector3::new(0f32, 0f32, 0f32),
            RightKneeSize: 0f32,
            RightKneeTposeAdjustment: CFrame::identity(),
            RightShoulder: Ref::none(),
            RightShoulderRangeMax: Vector3::new(0f32, 0f32, 0f32),
            RightShoulderRangeMin: Vector3::new(0f32, 0f32, 0f32),
            RightShoulderSize: 0f32,
            RightShoulderTposeAdjustment: CFrame::identity(),
            RightToes: Ref::none(),
            RightToesRangeMax: Vector3::new(0f32, 0f32, 0f32),
            RightToesRangeMin: Vector3::new(0f32, 0f32, 0f32),
            RightToesSize: 0f32,
            RightToesTposeAdjustment: CFrame::identity(),
            RightWrist: Ref::none(),
            RightWristRangeMax: Vector3::new(0f32, 0f32, 0f32),
            RightWristRangeMin: Vector3::new(0f32, 0f32, 0f32),
            RightWristSize: 0f32,
            RightWristTposeAdjustment: CFrame::identity(),
            Root: Ref::none(),
            RootRangeMax: Vector3::new(0f32, 0f32, 0f32),
            RootRangeMin: Vector3::new(0f32, 0f32, 0f32),
            RootSize: 0f32,
            RootTposeAdjustment: CFrame::identity(),
            Waist: Ref::none(),
            WaistRangeMax: Vector3::new(0f32, 0f32, 0f32),
            WaistRangeMin: Vector3::new(0f32, 0f32, 0f32),
            WaistSize: 0f32,
            WaistTposeAdjustment: CFrame::identity(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct IKControl {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ChainRoot: Ref,
    pub Enabled: bool,
    pub EndEffector: Ref,
    pub EndEffectorOffset: CFrame,
    pub Offset: CFrame,
    pub Pole: Ref,
    pub Priority: i32,
    pub SmoothTime: f32,
    pub Target: Ref,
    pub Type: enums::IKControlType,
    pub Weight: f32,
}
impl_inherits!(IKControl, Instance);
impl Default for IKControl {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ChainRoot: Ref::none(),
            Enabled: true,
            EndEffector: Ref::none(),
            EndEffectorOffset: CFrame::identity(),
            Offset: CFrame::identity(),
            Pole: Ref::none(),
            Priority: 0i32,
            SmoothTime: 0.05f32,
            Target: Ref::none(),
            Type: enums::IKControlType::Transform,
            Weight: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ILegacyStudioBridge {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ILegacyStudioBridge, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct IXPService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(IXPService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ImageButton {
    #[doc(hidden)]
    pub superclass: GuiButton,
    pub HoverImageContent: Content,
    pub ImageColor3: Color3,
    pub ImageContent: Content,
    pub ImageRectOffset: Vector2,
    pub ImageRectSize: Vector2,
    pub ImageTransparency: f32,
    pub PressedImageContent: Content,
    pub ResampleMode: enums::ResamplerMode,
    pub ScaleType: enums::ScaleType,
    pub SliceCenter: Rect,
    pub SliceScale: f32,
    pub TileSize: UDim2,
}
impl_inherits!(ImageButton, GuiButton);
impl Default for ImageButton {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase2d {
            superclass,
            AutoLocalize: true,
            RootLocalizationTable: Ref::none(),
            SelectionBehaviorDown: enums::SelectionBehavior::Escape,
            SelectionBehaviorLeft: enums::SelectionBehavior::Escape,
            SelectionBehaviorRight: enums::SelectionBehavior::Escape,
            SelectionBehaviorUp: enums::SelectionBehavior::Escape,
            SelectionGroup: false,
        };
        let superclass = GuiObject {
            superclass,
            Active: true,
            AnchorPoint: Vector2::new(0f32, 0f32),
            AutomaticSize: enums::AutomaticSize::None,
            BackgroundColor3: Color3::new(0.6392157f32, 0.63529414f32, 0.64705884f32),
            BackgroundTransparency: 0f32,
            BorderColor3: Color3::new(0.105882354f32, 0.16470589f32, 0.20784314f32),
            BorderMode: enums::BorderMode::Outline,
            BorderSizePixel: 1i32,
            ClipsDescendants: false,
            Draggable: false,
            Interactable: true,
            LayoutOrder: 0i32,
            NextSelectionDown: Ref::none(),
            NextSelectionLeft: Ref::none(),
            NextSelectionRight: Ref::none(),
            NextSelectionUp: Ref::none(),
            Position: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            Rotation: 0f32,
            Selectable: true,
            SelectionImageObject: Ref::none(),
            SelectionOrder: 0i32,
            Size: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            SizeConstraint: enums::SizeConstraint::RelativeXY,
            Visible: true,
            ZIndex: 1i32,
        };
        let superclass = GuiButton {
            superclass,
            AutoButtonColor: true,
            HoverHapticEffect: Ref::none(),
            Modal: false,
            PressHapticEffect: Ref::none(),
            Selected: false,
            Style: enums::ButtonStyle::Custom,
        };
        Self {
            superclass,
            HoverImageContent: Content::none(),
            ImageColor3: Color3::new(1f32, 1f32, 1f32),
            ImageContent: Content::none(),
            ImageRectOffset: Vector2::new(0f32, 0f32),
            ImageRectSize: Vector2::new(0f32, 0f32),
            ImageTransparency: 0f32,
            PressedImageContent: Content::none(),
            ResampleMode: enums::ResamplerMode::Default,
            ScaleType: enums::ScaleType::Stretch,
            SliceCenter: Rect::new(Vector2::new(0f32, 0f32), Vector2::new(0f32, 0f32)),
            SliceScale: 1f32,
            TileSize: UDim2::new(UDim::new(1f32, 0i32), UDim::new(1f32, 0i32)),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ImageHandleAdornment {
    #[doc(hidden)]
    pub superclass: HandleAdornment,
    pub Image: ContentId,
    pub Size: Vector2,
}
impl_inherits!(ImageHandleAdornment, HandleAdornment);
impl Default for ImageHandleAdornment {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase3d {
            superclass,
            Color3: Color3::new(0.9490197f32, 0.95294124f32, 0.95294124f32),
            Transparency: 0f32,
            Visible: true,
        };
        let superclass = PVAdornment {
            superclass,
            Adornee: Ref::none(),
        };
        let superclass = HandleAdornment {
            superclass,
            AdornCullingMode: enums::AdornCullingMode::Automatic,
            AlwaysOnTop: false,
            CFrame: CFrame::identity(),
            SizeRelativeOffset: Vector3::new(0f32, 0f32, 0f32),
            ZIndex: -1i32,
        };
        Self {
            superclass,
            Image: "rbxasset://textures/SurfacesDefault.png".into(),
            Size: Vector2::new(1f32, 1f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ImageLabel {
    #[doc(hidden)]
    pub superclass: GuiLabel,
    pub ImageColor3: Color3,
    pub ImageContent: Content,
    pub ImageRectOffset: Vector2,
    pub ImageRectSize: Vector2,
    pub ImageTransparency: f32,
    pub ResampleMode: enums::ResamplerMode,
    pub ScaleType: enums::ScaleType,
    pub SliceCenter: Rect,
    pub SliceScale: f32,
    pub TileSize: UDim2,
}
impl_inherits!(ImageLabel, GuiLabel);
impl Default for ImageLabel {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase2d {
            superclass,
            AutoLocalize: true,
            RootLocalizationTable: Ref::none(),
            SelectionBehaviorDown: enums::SelectionBehavior::Escape,
            SelectionBehaviorLeft: enums::SelectionBehavior::Escape,
            SelectionBehaviorRight: enums::SelectionBehavior::Escape,
            SelectionBehaviorUp: enums::SelectionBehavior::Escape,
            SelectionGroup: false,
        };
        let superclass = GuiObject {
            superclass,
            Active: false,
            AnchorPoint: Vector2::new(0f32, 0f32),
            AutomaticSize: enums::AutomaticSize::None,
            BackgroundColor3: Color3::new(0.6392157f32, 0.63529414f32, 0.64705884f32),
            BackgroundTransparency: 0f32,
            BorderColor3: Color3::new(0.105882354f32, 0.16470589f32, 0.20784314f32),
            BorderMode: enums::BorderMode::Outline,
            BorderSizePixel: 1i32,
            ClipsDescendants: false,
            Draggable: false,
            Interactable: true,
            LayoutOrder: 0i32,
            NextSelectionDown: Ref::none(),
            NextSelectionLeft: Ref::none(),
            NextSelectionRight: Ref::none(),
            NextSelectionUp: Ref::none(),
            Position: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            Rotation: 0f32,
            Selectable: false,
            SelectionImageObject: Ref::none(),
            SelectionOrder: 0i32,
            Size: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            SizeConstraint: enums::SizeConstraint::RelativeXY,
            Visible: true,
            ZIndex: 1i32,
        };
        let superclass = GuiLabel { superclass };
        Self {
            superclass,
            ImageColor3: Color3::new(1f32, 1f32, 1f32),
            ImageContent: Content::none(),
            ImageRectOffset: Vector2::new(0f32, 0f32),
            ImageRectSize: Vector2::new(0f32, 0f32),
            ImageTransparency: 0f32,
            ResampleMode: enums::ResamplerMode::Default,
            ScaleType: enums::ScaleType::Stretch,
            SliceCenter: Rect::new(Vector2::new(0f32, 0f32), Vector2::new(0f32, 0f32)),
            SliceScale: 1f32,
            TileSize: UDim2::new(UDim::new(1f32, 0i32), UDim::new(1f32, 0i32)),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ImportSession {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ImportSession, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct IncrementalPatchBuilder {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AddPathsToBundle: bool,
    pub BuildDebouncePeriod: f64,
    pub HighCompression: bool,
    pub SerializePatch: bool,
    pub UseFileLevelCompressionInsteadOfChunk: bool,
    pub ZstdCompression: bool,
}
impl_inherits!(IncrementalPatchBuilder, Instance);
impl Default for IncrementalPatchBuilder {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AddPathsToBundle: false,
            BuildDebouncePeriod: 0f64,
            HighCompression: false,
            SerializePatch: false,
            UseFileLevelCompressionInsteadOfChunk: false,
            ZstdCompression: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InputAction {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Enabled: bool,
    pub Type: enums::InputActionType,
}
impl_inherits!(InputAction, Instance);
impl Default for InputAction {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Enabled: true,
            Type: enums::InputActionType::Bool,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InputBinding {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Backward: enums::KeyCode,
    pub Down: enums::KeyCode,
    pub Forward: enums::KeyCode,
    pub KeyCode: enums::KeyCode,
    pub Left: enums::KeyCode,
    pub PressedThreshold: f32,
    pub ReleasedThreshold: f32,
    pub ResponseCurve: f32,
    pub Right: enums::KeyCode,
    pub Scale: f32,
    pub UiButton: Ref,
    pub Up: enums::KeyCode,
    pub Vector2Scale: Vector2,
}
impl_inherits!(InputBinding, Instance);
impl Default for InputBinding {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Backward: enums::KeyCode::Unknown,
            Down: enums::KeyCode::Unknown,
            Forward: enums::KeyCode::Unknown,
            KeyCode: enums::KeyCode::Unknown,
            Left: enums::KeyCode::Unknown,
            PressedThreshold: 0.5f32,
            ReleasedThreshold: 0.2f32,
            ResponseCurve: 1f32,
            Right: enums::KeyCode::Unknown,
            Scale: 1f32,
            UiButton: Ref::none(),
            Up: enums::KeyCode::Unknown,
            Vector2Scale: Vector2::new(1f32, 1f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InputContext {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Enabled: bool,
    pub Priority: i32,
    pub Sink: bool,
}
impl_inherits!(InputContext, Instance);
impl Default for InputContext {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Enabled: true,
            Priority: 1000i32,
            Sink: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct InputObject {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(InputObject, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InsertService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AllowClientInsertModels: bool,
    pub AllowInsertFreeModels: bool,
}
impl_inherits!(InsertService, Instance);
impl Default for InsertService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AllowClientInsertModels: false,
            AllowInsertFreeModels: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Instance {
    #[doc(hidden)]
    pub superclass: Object,
    pub Capabilities: SecurityCapabilities,
    pub HistoryId: UniqueId,
    pub Name: String,
    pub SourceAssetId: i64,
    pub Tags: Tags,
    pub UniqueId: UniqueId,
}
impl_inherits!(Instance, Object);
impl Default for Instance {
    fn default() -> Self {
        let superclass = Object::default();
        Self {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InstanceAdornment {
    #[doc(hidden)]
    pub superclass: GuiBase3d,
    pub Adornee: Ref,
}
impl_inherits!(InstanceAdornment, GuiBase3d);
impl Default for InstanceAdornment {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase3d {
            superclass,
            Color3: Color3::new(0f32, 0f32, 0f32),
            Transparency: 0f32,
            Visible: false,
        };
        Self {
            superclass,
            Adornee: Ref::none(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct InstanceExtensionsService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(InstanceExtensionsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct IntConstrainedValue {
    #[doc(hidden)]
    pub superclass: ValueBase,
    pub MaxValue: i64,
    pub MinValue: i64,
    pub Value: i64,
}
impl_inherits!(IntConstrainedValue, ValueBase);
impl Default for IntConstrainedValue {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        Self {
            superclass,
            MaxValue: 10i64,
            MinValue: 0i64,
            Value: 0i64,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct IntValue {
    #[doc(hidden)]
    pub superclass: ValueBase,
    pub Value: i64,
}
impl_inherits!(IntValue, ValueBase);
impl Default for IntValue {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        Self {
            superclass,
            Value: 0i64,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InternalSyncItem {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AutoSync: bool,
    pub Enabled: bool,
    pub Path: String,
}
impl_inherits!(InternalSyncItem, Instance);
impl Default for InternalSyncItem {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AutoSync: false,
            Enabled: false,
            Path: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct InternalSyncService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(InternalSyncService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct IntersectOperation {
    #[doc(hidden)]
    pub superclass: PartOperation,
}
impl_inherits!(IntersectOperation, PartOperation);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct InventoryPages {
    #[doc(hidden)]
    pub superclass: Pages,
}
impl_inherits!(InventoryPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct JointImportData {
    #[doc(hidden)]
    pub superclass: BaseImportData,
}
impl_inherits!(JointImportData, BaseImportData);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct JointInstance {
    #[doc(hidden)]
    pub superclass: Instance,
    pub C0: CFrame,
    pub C1: CFrame,
    pub Enabled: bool,
    pub Part0: Ref,
    pub Part1: Ref,
}
impl_inherits!(JointInstance, Instance);
impl Default for JointInstance {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            C0: CFrame::identity(),
            C1: CFrame::identity(),
            Enabled: false,
            Part0: Ref::none(),
            Part1: Ref::none(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct JointsService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(JointsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct KeyboardService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(KeyboardService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Keyframe {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Time: f32,
}
impl_inherits!(Keyframe, Instance);
impl Default for Keyframe {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Time: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct KeyframeMarker {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Value: String,
}
impl_inherits!(KeyframeMarker, Instance);
impl Default for KeyframeMarker {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Value: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct KeyframeSequence {
    #[doc(hidden)]
    pub superclass: AnimationClip,
    pub AuthoredHipHeight: f32,
}
impl_inherits!(KeyframeSequence, AnimationClip);
impl Default for KeyframeSequence {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AnimationClip {
            superclass,
            GuidBinaryString: b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0".as_slice().into(),
            Loop: true,
            Priority: enums::AnimationPriority::Action,
        };
        Self {
            superclass,
            AuthoredHipHeight: 2f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct KeyframeSequenceProvider {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(KeyframeSequenceProvider, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LSPFileSyncService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(LSPFileSyncService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LanguageService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(LanguageService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LayerCollector {
    #[doc(hidden)]
    pub superclass: GuiBase2d,
    pub Enabled: bool,
    pub ResetOnSpawn: bool,
    pub ZIndexBehavior: enums::ZIndexBehavior,
}
impl_inherits!(LayerCollector, GuiBase2d);
impl Default for LayerCollector {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase2d {
            superclass,
            AutoLocalize: false,
            RootLocalizationTable: Ref::none(),
            SelectionBehaviorDown: enums::SelectionBehavior::Escape,
            SelectionBehaviorLeft: enums::SelectionBehavior::Escape,
            SelectionBehaviorRight: enums::SelectionBehavior::Escape,
            SelectionBehaviorUp: enums::SelectionBehavior::Escape,
            SelectionGroup: false,
        };
        Self {
            superclass,
            Enabled: false,
            ResetOnSpawn: false,
            ZIndexBehavior: enums::ZIndexBehavior::Global,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LegacyStudioBridge {
    #[doc(hidden)]
    pub superclass: ILegacyStudioBridge,
}
impl_inherits!(LegacyStudioBridge, ILegacyStudioBridge);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Light {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Brightness: f32,
    pub Color: Color3,
    pub Enabled: bool,
    pub Shadows: bool,
}
impl_inherits!(Light, Instance);
impl Default for Light {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Brightness: 0f32,
            Color: Color3::new(0f32, 0f32, 0f32),
            Enabled: false,
            Shadows: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Lighting {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Ambient: Color3,
    pub Brightness: f32,
    pub ClockTime: f32,
    pub ColorShiftBottom: Color3,
    pub ColorShiftTop: Color3,
    pub EnvironmentDiffuseScale: f32,
    pub EnvironmentSpecularScale: f32,
    pub ExposureCompensation: f32,
    pub ExtendLightRangeTo120: enums::RolloutState,
    pub FogColor: Color3,
    pub FogEnd: f32,
    pub FogStart: f32,
    pub GeographicLatitude: f32,
    pub GlobalShadows: bool,
    pub LightingStyle: enums::LightingStyle,
    pub OutdoorAmbient: Color3,
    pub Outlines: bool,
    pub PrioritizeLightingQuality: bool,
    pub ShadowSoftness: f32,
    pub Technology: enums::Technology,
    pub TimeOfDay: String,
}
impl_inherits!(Lighting, Instance);
impl Default for Lighting {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Ambient: Color3::new(0.5f32, 0.5f32, 0.5f32),
            Brightness: 1.9812492f32,
            ClockTime: 0f32,
            ColorShiftBottom: Color3::new(0f32, 0f32, 0f32),
            ColorShiftTop: Color3::new(0f32, 0f32, 0f32),
            EnvironmentDiffuseScale: 0f32,
            EnvironmentSpecularScale: 0f32,
            ExposureCompensation: 0f32,
            ExtendLightRangeTo120: enums::RolloutState::Default,
            FogColor: Color3::new(0.75f32, 0.75f32, 0.75f32),
            FogEnd: 100000f32,
            FogStart: 0f32,
            GeographicLatitude: 41.7333f32,
            GlobalShadows: false,
            LightingStyle: enums::LightingStyle::Soft,
            OutdoorAmbient: Color3::new(0.5f32, 0.5f32, 0.5f32),
            Outlines: true,
            PrioritizeLightingQuality: false,
            ShadowSoftness: 0.5f32,
            Technology: enums::Technology::Voxel,
            TimeOfDay: "14:00:00".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LineForce {
    #[doc(hidden)]
    pub superclass: Constraint,
    pub ApplyAtCenterOfMass: bool,
    pub InverseSquareLaw: bool,
    pub Magnitude: f32,
    pub MaxForce: f32,
    pub ReactionForceEnabled: bool,
}
impl_inherits!(LineForce, Constraint);
impl Default for LineForce {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Constraint {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            Color: BrickColor::from_number(23u16).unwrap(),
            Enabled: true,
            Visible: false,
        };
        Self {
            superclass,
            ApplyAtCenterOfMass: false,
            InverseSquareLaw: false,
            Magnitude: 1000f32,
            MaxForce: f32::INFINITY,
            ReactionForceEnabled: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LineHandleAdornment {
    #[doc(hidden)]
    pub superclass: HandleAdornment,
    pub Length: f32,
    pub Thickness: f32,
}
impl_inherits!(LineHandleAdornment, HandleAdornment);
impl Default for LineHandleAdornment {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase3d {
            superclass,
            Color3: Color3::new(0.050980393f32, 0.4117647f32, 0.6745098f32),
            Transparency: 0f32,
            Visible: true,
        };
        let superclass = PVAdornment {
            superclass,
            Adornee: Ref::none(),
        };
        let superclass = HandleAdornment {
            superclass,
            AdornCullingMode: enums::AdornCullingMode::Automatic,
            AlwaysOnTop: false,
            CFrame: CFrame::identity(),
            SizeRelativeOffset: Vector3::new(0f32, 0f32, 0f32),
            ZIndex: -1i32,
        };
        Self {
            superclass,
            Length: 5f32,
            Thickness: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LinearVelocity {
    #[doc(hidden)]
    pub superclass: Constraint,
    pub ForceLimitMode: enums::ForceLimitMode,
    pub ForceLimitsEnabled: bool,
    pub LineDirection: Vector3,
    pub LineVelocity: f32,
    pub MaxAxesForce: Vector3,
    pub MaxForce: f32,
    pub MaxPlanarAxesForce: Vector2,
    pub PlaneVelocity: Vector2,
    pub PrimaryTangentAxis: Vector3,
    pub ReactionForceEnabled: bool,
    pub RelativeTo: enums::ActuatorRelativeTo,
    pub SecondaryTangentAxis: Vector3,
    pub VectorVelocity: Vector3,
    pub VelocityConstraintMode: enums::VelocityConstraintMode,
}
impl_inherits!(LinearVelocity, Constraint);
impl Default for LinearVelocity {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Constraint {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            Color: BrickColor::from_number(26u16).unwrap(),
            Enabled: true,
            Visible: false,
        };
        Self {
            superclass,
            ForceLimitMode: enums::ForceLimitMode::Magnitude,
            ForceLimitsEnabled: true,
            LineDirection: Vector3::new(1f32, 0f32, 0f32),
            LineVelocity: -0f32,
            MaxAxesForce: Vector3::new(1000f32, 1000f32, 1000f32),
            MaxForce: 1000f32,
            MaxPlanarAxesForce: Vector2::new(1000f32, 1000f32),
            PlaneVelocity: Vector2::new(0f32, 0f32),
            PrimaryTangentAxis: Vector3::new(1f32, 0f32, 0f32),
            ReactionForceEnabled: true,
            RelativeTo: enums::ActuatorRelativeTo::World,
            SecondaryTangentAxis: Vector3::new(0f32, 1f32, 0f32),
            VectorVelocity: Vector3::new(0f32, 0f32, 0f32),
            VelocityConstraintMode: enums::VelocityConstraintMode::Vector,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LinkingService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(LinkingService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LiveScriptingService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(LiveScriptingService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LiveSyncService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(LiveSyncService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LocalDebuggerConnection {
    #[doc(hidden)]
    pub superclass: DebuggerConnection,
}
impl_inherits!(LocalDebuggerConnection, DebuggerConnection);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LocalScript {
    #[doc(hidden)]
    pub superclass: Script,
}
impl_inherits!(LocalScript, Script);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LocalStorageService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(LocalStorageService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LocalizationService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(LocalizationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LocalizationTable {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Contents: String,
    pub SourceLocaleId: String,
}
impl_inherits!(LocalizationTable, Instance);
impl Default for LocalizationTable {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Contents: "[]".to_owned(),
            SourceLocaleId: "en-us".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LodDataEntity {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(LodDataEntity, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LodDataService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(LodDataService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LogReporterService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(LogReporterService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LogService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(LogService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LoginService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(LoginService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LuaSettings {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(LuaSettings, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LuaSourceContainer {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ScriptGuid: String,
}
impl_inherits!(LuaSourceContainer, Instance);
impl Default for LuaSourceContainer {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ScriptGuid: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LuaWebService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(LuaWebService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LuauScriptAnalyzerService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(LuauScriptAnalyzerService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MLModelDeliveryService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(MLModelDeliveryService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MLService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(MLService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MLSession {
    #[doc(hidden)]
    pub superclass: Object,
}
impl_inherits!(MLSession, Object);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ManualGlue {
    #[doc(hidden)]
    pub superclass: ManualSurfaceJointInstance,
}
impl_inherits!(ManualGlue, ManualSurfaceJointInstance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ManualSurfaceJointInstance {
    #[doc(hidden)]
    pub superclass: JointInstance,
}
impl_inherits!(ManualSurfaceJointInstance, JointInstance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ManualWeld {
    #[doc(hidden)]
    pub superclass: ManualSurfaceJointInstance,
}
impl_inherits!(ManualWeld, ManualSurfaceJointInstance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MarkerCurve {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ValuesAndTimes: BinaryString,
}
impl_inherits!(MarkerCurve, Instance);
impl Default for MarkerCurve {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ValuesAndTimes: b"\x02\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0".as_slice().into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MarketplaceService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(MarketplaceService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MatchmakingService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(MatchmakingService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MaterialGenerationService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(MaterialGenerationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MaterialImportData {
    #[doc(hidden)]
    pub superclass: BaseImportData,
    pub DiffuseFilePath: String,
    pub EmissiveFilePath: String,
    pub MetalnessFilePath: String,
    pub NormalFilePath: String,
    pub RoughnessFilePath: String,
}
impl_inherits!(MaterialImportData, BaseImportData);
impl Default for MaterialImportData {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BaseImportData {
            superclass,
            ImportName: "".to_owned(),
            ShouldImport: false,
        };
        Self {
            superclass,
            DiffuseFilePath: "".to_owned(),
            EmissiveFilePath: "".to_owned(),
            MetalnessFilePath: "".to_owned(),
            NormalFilePath: "".to_owned(),
            RoughnessFilePath: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MaterialService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AsphaltName: String,
    pub BasaltName: String,
    pub BrickName: String,
    pub CardboardName: String,
    pub CarpetName: String,
    pub CeramicTilesName: String,
    pub ClayRoofTilesName: String,
    pub CobblestoneName: String,
    pub ConcreteName: String,
    pub CorrodedMetalName: String,
    pub CrackedLavaName: String,
    pub DiamondPlateName: String,
    pub FabricName: String,
    pub FoilName: String,
    pub GlacierName: String,
    pub GraniteName: String,
    pub GrassName: String,
    pub GroundName: String,
    pub IceName: String,
    pub LeafyGrassName: String,
    pub LeatherName: String,
    pub LimestoneName: String,
    pub MarbleName: String,
    pub MetalName: String,
    pub MudName: String,
    pub PavementName: String,
    pub PebbleName: String,
    pub PlasterName: String,
    pub PlasticName: String,
    pub RockName: String,
    pub RoofShinglesName: String,
    pub RubberName: String,
    pub SaltName: String,
    pub SandName: String,
    pub SandstoneName: String,
    pub SlateName: String,
    pub SmoothPlasticName: String,
    pub SnowName: String,
    pub Use2022MaterialsXml: bool,
    pub WoodName: String,
    pub WoodPlanksName: String,
}
impl_inherits!(MaterialService, Instance);
impl Default for MaterialService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AsphaltName: "Asphalt".to_owned(),
            BasaltName: "Basalt".to_owned(),
            BrickName: "Brick".to_owned(),
            CardboardName: "Cardboard".to_owned(),
            CarpetName: "Carpet".to_owned(),
            CeramicTilesName: "CeramicTiles".to_owned(),
            ClayRoofTilesName: "ClayRoofTiles".to_owned(),
            CobblestoneName: "Cobblestone".to_owned(),
            ConcreteName: "Concrete".to_owned(),
            CorrodedMetalName: "CorrodedMetal".to_owned(),
            CrackedLavaName: "CrackedLava".to_owned(),
            DiamondPlateName: "DiamondPlate".to_owned(),
            FabricName: "Fabric".to_owned(),
            FoilName: "Foil".to_owned(),
            GlacierName: "Glacier".to_owned(),
            GraniteName: "Granite".to_owned(),
            GrassName: "Grass".to_owned(),
            GroundName: "Ground".to_owned(),
            IceName: "Ice".to_owned(),
            LeafyGrassName: "LeafyGrass".to_owned(),
            LeatherName: "Leather".to_owned(),
            LimestoneName: "Limestone".to_owned(),
            MarbleName: "Marble".to_owned(),
            MetalName: "Metal".to_owned(),
            MudName: "Mud".to_owned(),
            PavementName: "Pavement".to_owned(),
            PebbleName: "Pebble".to_owned(),
            PlasterName: "Plaster".to_owned(),
            PlasticName: "Plastic".to_owned(),
            RockName: "Rock".to_owned(),
            RoofShinglesName: "RoofShingles".to_owned(),
            RubberName: "Rubber".to_owned(),
            SaltName: "Salt".to_owned(),
            SandName: "Sand".to_owned(),
            SandstoneName: "Sandstone".to_owned(),
            SlateName: "Slate".to_owned(),
            SmoothPlasticName: "SmoothPlastic".to_owned(),
            SnowName: "Snow".to_owned(),
            Use2022MaterialsXml: false,
            WoodName: "Wood".to_owned(),
            WoodPlanksName: "WoodPlanks".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MaterialVariant {
    #[doc(hidden)]
    pub superclass: Instance,
    pub BaseMaterial: enums::Material,
    pub ColorMapContent: Content,
    pub CustomPhysicalProperties: PhysicalProperties,
    pub EmissiveMaskContent: Content,
    pub EmissiveStrength: f32,
    pub EmissiveTint: Color3,
    pub MaterialPattern: enums::MaterialPattern,
    pub MetalnessMapContent: Content,
    pub NormalMapContent: Content,
    pub RoughnessMapContent: Content,
    pub StudsPerTile: f32,
    pub TexturePack: ContentId,
}
impl_inherits!(MaterialVariant, Instance);
impl Default for MaterialVariant {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            BaseMaterial: enums::Material::Plastic,
            ColorMapContent: Content::none(),
            CustomPhysicalProperties: PhysicalProperties::Default,
            EmissiveMaskContent: Content::none(),
            EmissiveStrength: 1f32,
            EmissiveTint: Color3::new(1f32, 1f32, 1f32),
            MaterialPattern: enums::MaterialPattern::Regular,
            MetalnessMapContent: Content::none(),
            NormalMapContent: Content::none(),
            RoughnessMapContent: Content::none(),
            StudsPerTile: 10f32,
            TexturePack: "".into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MemStorageConnection {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(MemStorageConnection, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MemStorageService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(MemStorageService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MemoryStoreHashMap {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(MemoryStoreHashMap, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MemoryStoreHashMapPages {
    #[doc(hidden)]
    pub superclass: Pages,
}
impl_inherits!(MemoryStoreHashMapPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MemoryStoreQueue {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(MemoryStoreQueue, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MemoryStoreService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(MemoryStoreService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MemoryStoreSortedMap {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(MemoryStoreSortedMap, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MeshContentProvider {
    #[doc(hidden)]
    pub superclass: CacheableContentProvider,
}
impl_inherits!(MeshContentProvider, CacheableContentProvider);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MeshImportData {
    #[doc(hidden)]
    pub superclass: BaseImportData,
    pub Anchored: bool,
    pub CageMeshIntersectedPreview: bool,
    pub CageNonManifoldPreview: bool,
    pub CageOverlappingVerticesPreview: bool,
    pub CageUvMisMatchedPreview: bool,
    pub DoubleSided: bool,
    pub IgnoreVertexColors: bool,
    pub IrrelevantCageModifiedPreview: bool,
    pub MeshHoleDetectedPreview: bool,
    pub OuterCageFarExtendedFromMeshPreview: bool,
    pub UseImportedPivot: bool,
}
impl_inherits!(MeshImportData, BaseImportData);
impl Default for MeshImportData {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BaseImportData {
            superclass,
            ImportName: "".to_owned(),
            ShouldImport: false,
        };
        Self {
            superclass,
            Anchored: false,
            CageMeshIntersectedPreview: false,
            CageNonManifoldPreview: false,
            CageOverlappingVerticesPreview: false,
            CageUvMisMatchedPreview: false,
            DoubleSided: false,
            IgnoreVertexColors: false,
            IrrelevantCageModifiedPreview: false,
            MeshHoleDetectedPreview: false,
            OuterCageFarExtendedFromMeshPreview: false,
            UseImportedPivot: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MeshPart {
    #[doc(hidden)]
    pub superclass: TriangleMeshPart,
    pub DoubleSided: bool,
    pub HasJointOffset: bool,
    pub HasSkinnedMesh: bool,
    pub InitialSize: Vector3,
    pub JointOffset: Vector3,
    pub MeshContent: Content,
    pub PhysicsData: BinaryString,
    pub RenderFidelity: enums::RenderFidelity,
    pub TextureContent: Content,
    pub VertexCount: i32,
}
impl_inherits!(MeshPart, TriangleMeshPart);
impl Default for MeshPart {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PVInstance { superclass };
        let superclass = BasePart {
            superclass,
            Anchored: false,
            AudioCanCollide: true,
            BackParamA: -0.5f32,
            BackParamB: 0.5f32,
            BackSurface: enums::SurfaceType::Smooth,
            BackSurfaceInput: enums::InputType::NoInput,
            BottomParamA: -0.5f32,
            BottomParamB: 0.5f32,
            BottomSurface: enums::SurfaceType::Smooth,
            BottomSurfaceInput: enums::InputType::NoInput,
            CFrame: CFrame::identity(),
            CanCollide: true,
            CanQuery: true,
            CanTouch: true,
            CastShadow: true,
            CollisionGroup: "Default".to_owned(),
            CollisionGroupId: 0i32,
            CustomPhysicalProperties: PhysicalProperties::Default,
            EnableFluidForces: true,
            FrontParamA: -0.5f32,
            FrontParamB: 0.5f32,
            FrontSurface: enums::SurfaceType::Smooth,
            FrontSurfaceInput: enums::InputType::NoInput,
            LeftParamA: -0.5f32,
            LeftParamB: 0.5f32,
            LeftSurface: enums::SurfaceType::Smooth,
            LeftSurfaceInput: enums::InputType::NoInput,
            Locked: false,
            Massless: false,
            Material: enums::Material::Plastic,
            PivotOffset: CFrame::identity(),
            Reflectance: 0f32,
            RightParamA: -0.5f32,
            RightParamB: 0.5f32,
            RightSurface: enums::SurfaceType::Smooth,
            RightSurfaceInput: enums::InputType::NoInput,
            RootPriority: 0i32,
            RotVelocity: Vector3::new(0f32, 0f32, 0f32),
            TopParamA: -0.5f32,
            TopParamB: 0.5f32,
            TopSurface: enums::SurfaceType::Smooth,
            TopSurfaceInput: enums::InputType::NoInput,
            Transparency: 0f32,
            Velocity: Vector3::new(0f32, 0f32, 0f32),
        };
        let superclass = TriangleMeshPart {
            superclass,
            AeroMeshData: SharedString::new(b"".to_vec()),
            FluidFidelityInternal: enums::FluidFidelity::Automatic,
            InertiaMigrated: true,
            PhysicalConfigData: SharedString::new(b"".to_vec()),
            UnscaledCofm: Vector3::new(0f32, 0f32, 0f32),
            UnscaledVolInertiaDiags: Vector3::new(
                0.00000000000000000000000000000016666664f32,
                0.00000000000000000000000000000016666664f32,
                0.00000000000000000000000000000016666664f32,
            ),
            UnscaledVolInertiaOffDiags: Vector3::new(0f32, 0f32, 0f32),
            UnscaledVolume: 0.0000000000000000009999999f32,
        };
        Self {
            superclass,
            DoubleSided: false,
            HasJointOffset: false,
            HasSkinnedMesh: false,
            InitialSize: Vector3::new(0f32, 0f32, 0f32),
            JointOffset: Vector3::new(0f32, 0f32, 0f32),
            MeshContent: Content::none(),
            PhysicsData: b"".as_slice().into(),
            RenderFidelity: enums::RenderFidelity::Automatic,
            TextureContent: Content::none(),
            VertexCount: 0i32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Message {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Text: String,
}
impl_inherits!(Message, Instance);
impl Default for Message {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Text: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MessageBusConnection {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(MessageBusConnection, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MessageBusService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(MessageBusService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MessagingService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(MessagingService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MetaBreakpoint {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Condition: String,
    pub ContinueExecution: bool,
    pub Enabled: bool,
    pub Line: i32,
    pub LogMessage: String,
    pub RemoveOnHit: bool,
    pub Script: String,
}
impl_inherits!(MetaBreakpoint, Instance);
impl Default for MetaBreakpoint {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Condition: "".to_owned(),
            ContinueExecution: false,
            Enabled: true,
            Line: 0i32,
            LogMessage: "".to_owned(),
            RemoveOnHit: false,
            Script: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MetaBreakpointContext {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ContextDataInternal: String,
}
impl_inherits!(MetaBreakpointContext, Instance);
impl Default for MetaBreakpointContext {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ContextDataInternal: "0 1 2 ".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MetaBreakpointManager {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(MetaBreakpointManager, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MicroProfilerService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ContextLabel: String,
}
impl_inherits!(MicroProfilerService, Instance);
impl Default for MicroProfilerService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ContextLabel: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Model {
    #[doc(hidden)]
    pub superclass: PVInstance,
    pub LevelOfDetail: enums::ModelLevelOfDetail,
    pub ModelMeshCFrame: CFrame,
    pub ModelMeshData: SharedString,
    pub ModelMeshSize: Vector3,
    pub ModelStreamingMode: enums::ModelStreamingMode,
    pub NeedsPivotMigration: bool,
    pub PrimaryPart: Ref,
    pub SlimHash: SharedString,
    pub WorldPivotData: Option<CFrame>,
}
impl_inherits!(Model, PVInstance);
impl Default for Model {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PVInstance { superclass };
        Self {
            superclass,
            LevelOfDetail: enums::ModelLevelOfDetail::Automatic,
            ModelMeshCFrame: CFrame::identity(),
            ModelMeshData: SharedString::new(b"".to_vec()),
            ModelMeshSize: Vector3::new(0f32, 0f32, 0f32),
            ModelStreamingMode: enums::ModelStreamingMode::Default,
            NeedsPivotMigration: false,
            PrimaryPart: Ref::none(),
            SlimHash: SharedString::new(b"".to_vec()),
            WorldPivotData: None,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ModerationService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ModerationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ModuleScript {
    #[doc(hidden)]
    pub superclass: LuaSourceContainer,
    pub LinkedSource: ContentId,
    pub Source: String,
}
impl_inherits!(ModuleScript, LuaSourceContainer);
impl Default for ModuleScript {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = LuaSourceContainer {
            superclass,
            ScriptGuid: "".to_owned(),
        };
        Self {
            superclass,
            LinkedSource: "".into(),
            Source: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Motor {
    #[doc(hidden)]
    pub superclass: JointInstance,
    pub DesiredAngle: f32,
    pub MaxVelocity: f32,
}
impl_inherits!(Motor, JointInstance);
impl Default for Motor {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = JointInstance {
            superclass,
            C0: CFrame::identity(),
            C1: CFrame::identity(),
            Enabled: true,
            Part0: Ref::none(),
            Part1: Ref::none(),
        };
        Self {
            superclass,
            DesiredAngle: 0f32,
            MaxVelocity: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Motor6D {
    #[doc(hidden)]
    pub superclass: Motor,
}
impl_inherits!(Motor6D, Motor);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MotorFeature {
    #[doc(hidden)]
    pub superclass: Feature,
}
impl_inherits!(MotorFeature, Feature);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Mouse {
    #[doc(hidden)]
    pub superclass: Instance,
    pub IconContent: Content,
    pub TargetFilter: Ref,
}
impl_inherits!(Mouse, Instance);
impl Default for Mouse {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            IconContent: Content::none(),
            TargetFilter: Ref::none(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MouseService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(MouseService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MultipleDocumentInterfaceInstance {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(MultipleDocumentInterfaceInstance, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NegateOperation {
    #[doc(hidden)]
    pub superclass: PartOperation,
    pub PreviousOperation: enums::NegateOperationHiddenHistory,
}
impl_inherits!(NegateOperation, PartOperation);
impl Default for NegateOperation {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PVInstance { superclass };
        let superclass = BasePart {
            superclass,
            Anchored: true,
            AudioCanCollide: true,
            BackParamA: -0.5f32,
            BackParamB: 0.5f32,
            BackSurface: enums::SurfaceType::Smooth,
            BackSurfaceInput: enums::InputType::NoInput,
            BottomParamA: -0.5f32,
            BottomParamB: 0.5f32,
            BottomSurface: enums::SurfaceType::Smooth,
            BottomSurfaceInput: enums::InputType::NoInput,
            CFrame: CFrame::identity(),
            CanCollide: false,
            CanQuery: true,
            CanTouch: true,
            CastShadow: true,
            CollisionGroup: "Default".to_owned(),
            CollisionGroupId: 0i32,
            CustomPhysicalProperties: PhysicalProperties::Default,
            EnableFluidForces: true,
            FrontParamA: -0.5f32,
            FrontParamB: 0.5f32,
            FrontSurface: enums::SurfaceType::Smooth,
            FrontSurfaceInput: enums::InputType::NoInput,
            LeftParamA: -0.5f32,
            LeftParamB: 0.5f32,
            LeftSurface: enums::SurfaceType::Smooth,
            LeftSurfaceInput: enums::InputType::NoInput,
            Locked: false,
            Massless: false,
            Material: enums::Material::Plastic,
            PivotOffset: CFrame::identity(),
            Reflectance: 0f32,
            RightParamA: -0.5f32,
            RightParamB: 0.5f32,
            RightSurface: enums::SurfaceType::Smooth,
            RightSurfaceInput: enums::InputType::NoInput,
            RootPriority: 0i32,
            RotVelocity: Vector3::new(0f32, 0f32, 0f32),
            TopParamA: -0.5f32,
            TopParamB: 0.5f32,
            TopSurface: enums::SurfaceType::Smooth,
            TopSurfaceInput: enums::InputType::NoInput,
            Transparency: 0.1f32,
            Velocity: Vector3::new(0f32, 0f32, 0f32),
        };
        let superclass = TriangleMeshPart {
            superclass,
            AeroMeshData: SharedString::new(b"".to_vec()),
            FluidFidelityInternal: enums::FluidFidelity::Automatic,
            InertiaMigrated: true,
            PhysicalConfigData: SharedString::new(b"".to_vec()),
            UnscaledCofm: Vector3::new(0f32, 0f32, 0f32),
            UnscaledVolInertiaDiags: Vector3::new(0.16666666f32, 0.16666666f32, 0.16666666f32),
            UnscaledVolInertiaOffDiags: Vector3::new(0f32, 0f32, 0f32),
            UnscaledVolume: 1f32,
        };
        let superclass = PartOperation {
            superclass,
            AssetId: "".into(),
            ChildData: b"".as_slice().into(),
            ChildData2: SharedString::new(b"".to_vec()),
            ComponentIndex: -1i32,
            FormFactor: enums::FormFactor::Custom,
            InitialSize: Vector3::new(1f32, 1f32, 1f32),
            MeshData: b"".as_slice().into(),
            MeshData2: SharedString::new(b"".to_vec()),
            OffCentered: false,
            PhysicsData: b"".as_slice().into(),
            RenderFidelity: enums::RenderFidelity::Automatic,
            SmoothingAngle: 0f32,
            SolidMeshHolder: NetAssetRef::new(b"".to_vec()),
            UsePartColor: false,
        };
        Self {
            superclass,
            PreviousOperation: enums::NegateOperationHiddenHistory::None,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct NetworkClient {
    #[doc(hidden)]
    pub superclass: NetworkPeer,
}
impl_inherits!(NetworkClient, NetworkPeer);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct NetworkMarker {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(NetworkMarker, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct NetworkPeer {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(NetworkPeer, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct NetworkReplicator {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(NetworkReplicator, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct NetworkServer {
    #[doc(hidden)]
    pub superclass: NetworkPeer,
}
impl_inherits!(NetworkServer, NetworkPeer);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NetworkSettings {
    #[doc(hidden)]
    pub superclass: Instance,
    pub HttpProxyEnabled: bool,
    pub HttpProxyUrl: String,
    pub IncomingReplicationLag: f64,
    pub PrintJoinSizeBreakdown: bool,
    pub PrintPhysicsErrors: bool,
    pub PrintStreamInstanceQuota: bool,
    pub RandomizeJoinInstanceOrder: bool,
    pub RenderStreamedRegions: bool,
    pub ShowActiveAnimationAsset: bool,
}
impl_inherits!(NetworkSettings, Instance);
impl Default for NetworkSettings {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            HttpProxyEnabled: false,
            HttpProxyUrl: "".to_owned(),
            IncomingReplicationLag: 0f64,
            PrintJoinSizeBreakdown: false,
            PrintPhysicsErrors: false,
            PrintStreamInstanceQuota: false,
            RandomizeJoinInstanceOrder: false,
            RenderStreamedRegions: false,
            ShowActiveAnimationAsset: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NoCollisionConstraint {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Enabled: bool,
    pub Part0: Ref,
    pub Part1: Ref,
}
impl_inherits!(NoCollisionConstraint, Instance);
impl Default for NoCollisionConstraint {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Enabled: true,
            Part0: Ref::none(),
            Part1: Ref::none(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Noise {
    #[doc(hidden)]
    pub superclass: Instance,
    pub NoiseType: enums::NoiseType,
    pub Seed: i32,
}
impl_inherits!(Noise, Instance);
impl Default for Noise {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            NoiseType: enums::NoiseType::SimplexGabor,
            Seed: 0i32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct NonReplicatedCSGDictionaryService {
    #[doc(hidden)]
    pub superclass: FlyweightService,
}
impl_inherits!(NonReplicatedCSGDictionaryService, FlyweightService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct NotificationService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(NotificationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NumberPose {
    #[doc(hidden)]
    pub superclass: PoseBase,
    pub Value: f64,
}
impl_inherits!(NumberPose, PoseBase);
impl Default for NumberPose {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PoseBase {
            superclass,
            EasingDirection: enums::PoseEasingDirection::In,
            EasingStyle: enums::PoseEasingStyle::Linear,
            Weight: 1f32,
        };
        Self {
            superclass,
            Value: 0f64,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NumberValue {
    #[doc(hidden)]
    pub superclass: ValueBase,
    pub Value: f64,
}
impl_inherits!(NumberValue, ValueBase);
impl Default for NumberValue {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        Self {
            superclass,
            Value: 0f64,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Object {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ObjectValue {
    #[doc(hidden)]
    pub superclass: ValueBase,
    pub Value: Ref,
}
impl_inherits!(ObjectValue, ValueBase);
impl Default for ObjectValue {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        Self {
            superclass,
            Value: Ref::none(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct OmniRecommendationsService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(OmniRecommendationsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct OpenCloudApiV1 {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(OpenCloudApiV1, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct OpenCloudService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(OpenCloudService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct OperationGraph {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(OperationGraph, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct OrderedDataStore {
    #[doc(hidden)]
    pub superclass: GlobalDataStore,
}
impl_inherits!(OrderedDataStore, GlobalDataStore);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct OutfitPages {
    #[doc(hidden)]
    pub superclass: Pages,
}
impl_inherits!(OutfitPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PVAdornment {
    #[doc(hidden)]
    pub superclass: GuiBase3d,
    pub Adornee: Ref,
}
impl_inherits!(PVAdornment, GuiBase3d);
impl Default for PVAdornment {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase3d {
            superclass,
            Color3: Color3::new(0f32, 0f32, 0f32),
            Transparency: 0f32,
            Visible: false,
        };
        Self {
            superclass,
            Adornee: Ref::none(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PVInstance {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PVInstance, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PackageLink {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AutoUpdate: bool,
    pub DefaultName: String,
    pub ModifiedState: i32,
    pub SerializedDefaultAttributes: BinaryString,
    pub VersionIdSerialize: i64,
}
impl_inherits!(PackageLink, Instance);
impl Default for PackageLink {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AutoUpdate: false,
            DefaultName: "".to_owned(),
            ModifiedState: 0i32,
            SerializedDefaultAttributes: b"".as_slice().into(),
            VersionIdSerialize: 0i64,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PackageService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PackageService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PackageUIService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PackageUIService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Pages {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(Pages, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Pants {
    #[doc(hidden)]
    pub superclass: Clothing,
    pub PantsTemplate: ContentId,
}
impl_inherits!(Pants, Clothing);
impl Default for Pants {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = CharacterAppearance { superclass };
        let superclass = Clothing {
            superclass,
            Color3: Color3::new(1f32, 1f32, 1f32),
        };
        Self {
            superclass,
            PantsTemplate: "".into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ParabolaAdornment {
    #[doc(hidden)]
    pub superclass: PVAdornment,
}
impl_inherits!(ParabolaAdornment, PVAdornment);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Part {
    #[doc(hidden)]
    pub superclass: FormFactorPart,
}
impl_inherits!(Part, FormFactorPart);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PartAdornment {
    #[doc(hidden)]
    pub superclass: GuiBase3d,
    pub Adornee: Ref,
}
impl_inherits!(PartAdornment, GuiBase3d);
impl Default for PartAdornment {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase3d {
            superclass,
            Color3: Color3::new(0f32, 0f32, 0f32),
            Transparency: 0f32,
            Visible: false,
        };
        Self {
            superclass,
            Adornee: Ref::none(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PartOperation {
    #[doc(hidden)]
    pub superclass: TriangleMeshPart,
    pub AssetId: ContentId,
    pub ChildData: BinaryString,
    pub ChildData2: SharedString,
    pub ComponentIndex: i32,
    pub FormFactor: enums::FormFactor,
    pub InitialSize: Vector3,
    pub MeshData: BinaryString,
    pub MeshData2: SharedString,
    pub OffCentered: bool,
    pub PhysicsData: BinaryString,
    pub RenderFidelity: enums::RenderFidelity,
    pub SmoothingAngle: f32,
    pub SolidMeshHolder: NetAssetRef,
    pub UsePartColor: bool,
}
impl_inherits!(PartOperation, TriangleMeshPart);
impl Default for PartOperation {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PVInstance { superclass };
        let superclass = BasePart {
            superclass,
            Anchored: false,
            AudioCanCollide: true,
            BackParamA: -0.5f32,
            BackParamB: 0.5f32,
            BackSurface: enums::SurfaceType::Smooth,
            BackSurfaceInput: enums::InputType::NoInput,
            BottomParamA: -0.5f32,
            BottomParamB: 0.5f32,
            BottomSurface: enums::SurfaceType::Smooth,
            BottomSurfaceInput: enums::InputType::NoInput,
            CFrame: CFrame::identity(),
            CanCollide: true,
            CanQuery: true,
            CanTouch: true,
            CastShadow: true,
            CollisionGroup: "Default".to_owned(),
            CollisionGroupId: 0i32,
            CustomPhysicalProperties: PhysicalProperties::Default,
            EnableFluidForces: true,
            FrontParamA: -0.5f32,
            FrontParamB: 0.5f32,
            FrontSurface: enums::SurfaceType::Smooth,
            FrontSurfaceInput: enums::InputType::NoInput,
            LeftParamA: -0.5f32,
            LeftParamB: 0.5f32,
            LeftSurface: enums::SurfaceType::Smooth,
            LeftSurfaceInput: enums::InputType::NoInput,
            Locked: false,
            Massless: false,
            Material: enums::Material::Plastic,
            PivotOffset: CFrame::identity(),
            Reflectance: 0f32,
            RightParamA: -0.5f32,
            RightParamB: 0.5f32,
            RightSurface: enums::SurfaceType::Smooth,
            RightSurfaceInput: enums::InputType::NoInput,
            RootPriority: 0i32,
            RotVelocity: Vector3::new(0f32, 0f32, 0f32),
            TopParamA: -0.5f32,
            TopParamB: 0.5f32,
            TopSurface: enums::SurfaceType::Smooth,
            TopSurfaceInput: enums::InputType::NoInput,
            Transparency: 0f32,
            Velocity: Vector3::new(0f32, 0f32, 0f32),
        };
        let superclass = TriangleMeshPart {
            superclass,
            AeroMeshData: SharedString::new(b"".to_vec()),
            FluidFidelityInternal: enums::FluidFidelity::Automatic,
            InertiaMigrated: true,
            PhysicalConfigData: SharedString::new(b"".to_vec()),
            UnscaledCofm: Vector3::new(0f32, 0f32, 0f32),
            UnscaledVolInertiaDiags: Vector3::new(0.16666666f32, 0.16666666f32, 0.16666666f32),
            UnscaledVolInertiaOffDiags: Vector3::new(0f32, 0f32, 0f32),
            UnscaledVolume: 1f32,
        };
        Self {
            superclass,
            AssetId: "".into(),
            ChildData: b"".as_slice().into(),
            ChildData2: SharedString::new(b"".to_vec()),
            ComponentIndex: -1i32,
            FormFactor: enums::FormFactor::Custom,
            InitialSize: Vector3::new(1f32, 1f32, 1f32),
            MeshData: b"".as_slice().into(),
            MeshData2: SharedString::new(b"".to_vec()),
            OffCentered: false,
            PhysicsData: b"".as_slice().into(),
            RenderFidelity: enums::RenderFidelity::Automatic,
            SmoothingAngle: 0f32,
            SolidMeshHolder: NetAssetRef::new(b"".to_vec()),
            UsePartColor: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PartOperationAsset {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ChildData: BinaryString,
    pub MeshData: BinaryString,
}
impl_inherits!(PartOperationAsset, Instance);
impl Default for PartOperationAsset {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ChildData: b"".as_slice().into(),
            MeshData: b"".as_slice().into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ParticleEmitter {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Acceleration: Vector3,
    pub Brightness: f32,
    pub Color: ColorSequence,
    pub Drag: f32,
    pub EmissionDirection: enums::NormalId,
    pub Enabled: bool,
    pub FlipbookFramerate: NumberRange,
    pub FlipbookIncompatible: String,
    pub FlipbookLayout: enums::ParticleFlipbookLayout,
    pub FlipbookMode: enums::ParticleFlipbookMode,
    pub FlipbookSizeX: i32,
    pub FlipbookSizeY: i32,
    pub FlipbookStartRandom: bool,
    pub Lifetime: NumberRange,
    pub LightEmission: f32,
    pub LightInfluence: f32,
    pub LockedToPart: bool,
    pub Orientation: enums::ParticleOrientation,
    pub Rate: f32,
    pub RotSpeed: NumberRange,
    pub Rotation: NumberRange,
    pub Shape: enums::ParticleEmitterShape,
    pub ShapeInOut: enums::ParticleEmitterShapeInOut,
    pub ShapePartial: f32,
    pub ShapeStyle: enums::ParticleEmitterShapeStyle,
    pub Size: NumberSequence,
    pub Speed: NumberRange,
    pub SpreadAngle: Vector2,
    pub Squash: NumberSequence,
    pub Texture: ContentId,
    pub TimeScale: f32,
    pub Transparency: NumberSequence,
    pub VelocityInheritance: f32,
    pub WindAffectsDrag: bool,
    pub ZOffset: f32,
}
impl_inherits!(ParticleEmitter, Instance);
impl Default for ParticleEmitter {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Acceleration: Vector3::new(0f32, 0f32, 0f32),
            Brightness: 1f32,
            Color: ColorSequence {
                keypoints: vec![
                    ColorSequenceKeypoint::new(0f32, Color3::new(1f32, 1f32, 1f32)),
                    ColorSequenceKeypoint::new(1f32, Color3::new(1f32, 1f32, 1f32)),
                ],
            },
            Drag: 0f32,
            EmissionDirection: enums::NormalId::Top,
            Enabled: true,
            FlipbookFramerate: NumberRange::new(1f32, 1f32),
            FlipbookIncompatible: "Particle texture must be 1024 by 1024 to use flipbooks."
                .to_owned(),
            FlipbookLayout: enums::ParticleFlipbookLayout::None,
            FlipbookMode: enums::ParticleFlipbookMode::Loop,
            FlipbookSizeX: 1i32,
            FlipbookSizeY: 1i32,
            FlipbookStartRandom: false,
            Lifetime: NumberRange::new(5f32, 10f32),
            LightEmission: 0f32,
            LightInfluence: 0f32,
            LockedToPart: false,
            Orientation: enums::ParticleOrientation::FacingCamera,
            Rate: 20f32,
            RotSpeed: NumberRange::new(0f32, 0f32),
            Rotation: NumberRange::new(0f32, 0f32),
            Shape: enums::ParticleEmitterShape::Box,
            ShapeInOut: enums::ParticleEmitterShapeInOut::Outward,
            ShapePartial: 1f32,
            ShapeStyle: enums::ParticleEmitterShapeStyle::Volume,
            Size: NumberSequence {
                keypoints: vec![
                    NumberSequenceKeypoint::new(0f32, 1f32, 0f32),
                    NumberSequenceKeypoint::new(1f32, 1f32, 0f32),
                ],
            },
            Speed: NumberRange::new(5f32, 5f32),
            SpreadAngle: Vector2::new(0f32, 0f32),
            Squash: NumberSequence {
                keypoints: vec![
                    NumberSequenceKeypoint::new(0f32, 0f32, 0f32),
                    NumberSequenceKeypoint::new(1f32, 0f32, 0f32),
                ],
            },
            Texture: "rbxasset://textures/particles/sparkles_main.dds".into(),
            TimeScale: 1f32,
            Transparency: NumberSequence {
                keypoints: vec![
                    NumberSequenceKeypoint::new(0f32, 0f32, 0f32),
                    NumberSequenceKeypoint::new(1f32, 0f32, 0f32),
                ],
            },
            VelocityInheritance: 0f32,
            WindAffectsDrag: false,
            ZOffset: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PartyEmulatorService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PartyEmulatorService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PatchBundlerFileWatch {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PatchBundlerFileWatch, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PatchMapping {
    #[doc(hidden)]
    pub superclass: Instance,
    pub FlattenTree: bool,
    pub PatchId: String,
    pub TargetPath: String,
}
impl_inherits!(PatchMapping, Instance);
impl Default for PatchMapping {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            FlattenTree: false,
            PatchId: "".to_owned(),
            TargetPath: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Path {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(Path, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Path2D {
    #[doc(hidden)]
    pub superclass: GuiBase,
    pub Closed: bool,
    pub Color3: Color3,
    pub PropertiesSerialize: BinaryString,
    pub Thickness: f32,
    pub Transparency: f32,
    pub Visible: bool,
    pub ZIndex: i32,
}
impl_inherits!(Path2D, GuiBase);
impl Default for Path2D {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        Self {
            superclass,
            Closed: false,
            Color3: Color3::new(0f32, 0f32, 0f32),
            PropertiesSerialize: b"\0\0\0\0".as_slice().into(),
            Thickness: 1f32,
            Transparency: 0f32,
            Visible: true,
            ZIndex: 1i32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PathfindingLink {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Attachment0: Ref,
    pub Attachment1: Ref,
    pub IsBidirectional: bool,
    pub Label: String,
}
impl_inherits!(PathfindingLink, Instance);
impl Default for PathfindingLink {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            IsBidirectional: true,
            Label: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PathfindingModifier {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Label: String,
    pub PassThrough: bool,
}
impl_inherits!(PathfindingModifier, Instance);
impl Default for PathfindingModifier {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Label: "".to_owned(),
            PassThrough: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PathfindingService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PathfindingService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PausedState {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PausedState, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PausedStateBreakpoint {
    #[doc(hidden)]
    pub superclass: PausedState,
}
impl_inherits!(PausedStateBreakpoint, PausedState);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PausedStateException {
    #[doc(hidden)]
    pub superclass: PausedState,
}
impl_inherits!(PausedStateException, PausedState);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PerformanceControlService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PerformanceControlService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PermissionsService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PermissionsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PhysicsService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PhysicsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PhysicsSettings {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AllowSleep: bool,
    pub AreAnchorsShown: bool,
    pub AreAssembliesShown: bool,
    pub AreAssemblyCentersOfMassShown: bool,
    pub AreAwakePartsHighlighted: bool,
    pub AreBodyTypesShown: bool,
    pub AreCollisionCostsShown: bool,
    pub AreConstraintForcesShownForSelectedOrHoveredInstances: bool,
    pub AreConstraintTorquesShownForSelectedOrHoveredInstances: bool,
    pub AreContactForcesShownForSelectedOrHoveredAssemblies: bool,
    pub AreContactIslandsShown: bool,
    pub AreContactPointsShown: bool,
    pub AreGravityForcesShownForSelectedOrHoveredAssemblies: bool,
    pub AreJointCoordinatesShown: bool,
    pub AreMagnitudesShownForDrawnForcesAndTorques: bool,
    pub AreMechanismsShown: bool,
    pub AreModelCoordsShown: bool,
    pub AreNonAnchorsShown: bool,
    pub AreOwnersShown: bool,
    pub ArePartCoordsShown: bool,
    pub AreRegionsShown: bool,
    pub AreSolverIslandsShown: bool,
    pub AreTerrainReplicationRegionsShown: bool,
    pub AreTimestepsShown: bool,
    pub AreUnalignedPartsShown: bool,
    pub AreWorldCoordsShown: bool,
    pub DisableCsGv2: bool,
    pub DisableCsGv3ForPlugins: bool,
    pub DrawConstraintsNetForce: bool,
    pub DrawContactsNetForce: bool,
    pub DrawTotalNetForce: bool,
    pub EnableForceVisualizationSmoothing: bool,
    pub FluidForceDrawScale: f32,
    pub ForceCsGv2: bool,
    pub ForceDrawScale: f32,
    pub ForceVisualizationSmoothingSteps: i32,
    pub IsInterpolationThrottleShown: bool,
    pub IsReceiveAgeShown: bool,
    pub IsTreeShown: bool,
    pub PhysicsEnvironmentalThrottle: enums::EnviromentalPhysicsThrottle,
    pub ShowDecompositionGeometry: bool,
    pub ShowFluidForcesForSelectedOrHoveredMechanisms: bool,
    pub ShowInstanceNamesForDrawnForcesAndTorques: bool,
    pub SolverConvergenceMetricType: enums::SolverConvergenceMetricType,
    pub SolverConvergenceVisualizationMode: enums::SolverConvergenceVisualizationMode,
    pub ThrottleAdjustTime: f64,
    pub TorqueDrawScale: f32,
    pub UseCsGv2: bool,
}
impl_inherits!(PhysicsSettings, Instance);
impl Default for PhysicsSettings {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AllowSleep: false,
            AreAnchorsShown: false,
            AreAssembliesShown: false,
            AreAssemblyCentersOfMassShown: false,
            AreAwakePartsHighlighted: false,
            AreBodyTypesShown: false,
            AreCollisionCostsShown: false,
            AreConstraintForcesShownForSelectedOrHoveredInstances: false,
            AreConstraintTorquesShownForSelectedOrHoveredInstances: false,
            AreContactForcesShownForSelectedOrHoveredAssemblies: false,
            AreContactIslandsShown: false,
            AreContactPointsShown: false,
            AreGravityForcesShownForSelectedOrHoveredAssemblies: false,
            AreJointCoordinatesShown: false,
            AreMagnitudesShownForDrawnForcesAndTorques: false,
            AreMechanismsShown: false,
            AreModelCoordsShown: false,
            AreNonAnchorsShown: false,
            AreOwnersShown: false,
            ArePartCoordsShown: false,
            AreRegionsShown: false,
            AreSolverIslandsShown: false,
            AreTerrainReplicationRegionsShown: false,
            AreTimestepsShown: false,
            AreUnalignedPartsShown: false,
            AreWorldCoordsShown: false,
            DisableCsGv2: false,
            DisableCsGv3ForPlugins: false,
            DrawConstraintsNetForce: false,
            DrawContactsNetForce: false,
            DrawTotalNetForce: false,
            EnableForceVisualizationSmoothing: false,
            FluidForceDrawScale: 0f32,
            ForceCsGv2: false,
            ForceDrawScale: 0f32,
            ForceVisualizationSmoothingSteps: 0i32,
            IsInterpolationThrottleShown: false,
            IsReceiveAgeShown: false,
            IsTreeShown: false,
            PhysicsEnvironmentalThrottle: enums::EnviromentalPhysicsThrottle::DefaultAuto,
            ShowDecompositionGeometry: false,
            ShowFluidForcesForSelectedOrHoveredMechanisms: false,
            ShowInstanceNamesForDrawnForcesAndTorques: false,
            SolverConvergenceMetricType: enums::SolverConvergenceMetricType::IterationBased,
            SolverConvergenceVisualizationMode: enums::SolverConvergenceVisualizationMode::Disabled,
            ThrottleAdjustTime: 0f64,
            TorqueDrawScale: 0f32,
            UseCsGv2: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PitchShiftSoundEffect {
    #[doc(hidden)]
    pub superclass: SoundEffect,
    pub Octave: f32,
}
impl_inherits!(PitchShiftSoundEffect, SoundEffect);
impl Default for PitchShiftSoundEffect {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundEffect {
            superclass,
            Enabled: true,
            Priority: 0i32,
        };
        Self {
            superclass,
            Octave: 1.25f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlaceAssetIdsService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PlaceAssetIdsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlaceStatsService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PlaceStatsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlacesService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PlacesService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Plane {
    #[doc(hidden)]
    pub superclass: PlaneConstraint,
}
impl_inherits!(Plane, PlaneConstraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlaneConstraint {
    #[doc(hidden)]
    pub superclass: Constraint,
}
impl_inherits!(PlaneConstraint, Constraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Platform {
    #[doc(hidden)]
    pub superclass: Part,
}
impl_inherits!(Platform, Part);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlatformCloudStorageService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PlatformCloudStorageService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlatformFriendsService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PlatformFriendsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Player {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AutoJumpEnabled: bool,
    pub CameraMaxZoomDistance: f32,
    pub CameraMinZoomDistance: f32,
    pub CameraMode: enums::CameraMode,
    pub Character: Ref,
    pub CharacterAppearance: String,
    pub CharacterAppearanceId: i64,
    pub DevCameraOcclusionMode: enums::DevCameraOcclusionMode,
    pub DevComputerCameraMode: enums::DevComputerCameraMovementMode,
    pub DevComputerMovementMode: enums::DevComputerMovementMode,
    pub DevEnableMouseLock: bool,
    pub DevTouchCameraMode: enums::DevTouchCameraMovementMode,
    pub DevTouchMovementMode: enums::DevTouchMovementMode,
    pub HealthDisplayDistance: f32,
    pub NameDisplayDistance: f32,
    pub Neutral: bool,
    pub RawJoinData: BinaryString,
    pub ReplicationFocus: Ref,
    pub RespawnLocation: Ref,
    pub StepIdOffset: i32,
    pub Team: Ref,
    pub TeamColor: BrickColor,
    pub TeleportedIn: bool,
}
impl_inherits!(Player, Instance);
impl Default for Player {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AutoJumpEnabled: false,
            CameraMaxZoomDistance: 0f32,
            CameraMinZoomDistance: 0f32,
            CameraMode: enums::CameraMode::Classic,
            Character: Ref::none(),
            CharacterAppearance: "".to_owned(),
            CharacterAppearanceId: 0i64,
            DevCameraOcclusionMode: enums::DevCameraOcclusionMode::Zoom,
            DevComputerCameraMode: enums::DevComputerCameraMovementMode::UserChoice,
            DevComputerMovementMode: enums::DevComputerMovementMode::UserChoice,
            DevEnableMouseLock: false,
            DevTouchCameraMode: enums::DevTouchCameraMovementMode::UserChoice,
            DevTouchMovementMode: enums::DevTouchMovementMode::UserChoice,
            HealthDisplayDistance: 0f32,
            NameDisplayDistance: 0f32,
            Neutral: false,
            RawJoinData: b"".as_slice().into(),
            ReplicationFocus: Ref::none(),
            RespawnLocation: Ref::none(),
            StepIdOffset: 0i32,
            Team: Ref::none(),
            TeamColor: BrickColor::from_number(194u16).unwrap(),
            TeleportedIn: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlayerData {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PlayerData, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlayerDataRecord {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PlayerDataRecord, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlayerDataRecordConfig {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PlayerDataRecordConfig, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerDataService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub LoadFailureBehavior: enums::PlayerDataLoadFailureBehavior,
}
impl_inherits!(PlayerDataService, Instance);
impl Default for PlayerDataService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            LoadFailureBehavior: enums::PlayerDataLoadFailureBehavior::Failure,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerEmulatorService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub CustomPoliciesEnabled: bool,
    pub EmulatedCountryCode: String,
    pub EmulatedGameLocale: String,
    pub PlayerEmulationEnabled: bool,
    pub PseudolocalizationEnabled: bool,
    pub SerializedEmulatedPolicyInfo: BinaryString,
    pub TextElongationFactor: i32,
}
impl_inherits!(PlayerEmulatorService, Instance);
impl Default for PlayerEmulatorService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            CustomPoliciesEnabled: false,
            EmulatedCountryCode: "".to_owned(),
            EmulatedGameLocale: "".to_owned(),
            PlayerEmulationEnabled: false,
            PseudolocalizationEnabled: false,
            SerializedEmulatedPolicyInfo: b"".as_slice().into(),
            TextElongationFactor: 0i32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerGui {
    #[doc(hidden)]
    pub superclass: BasePlayerGui,
    pub ScreenOrientation: enums::ScreenOrientation,
    pub SelectionImageObject: Ref,
}
impl_inherits!(PlayerGui, BasePlayerGui);
impl Default for PlayerGui {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BasePlayerGui { superclass };
        Self {
            superclass,
            ScreenOrientation: enums::ScreenOrientation::LandscapeLeft,
            SelectionImageObject: Ref::none(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlayerHydrationService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PlayerHydrationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlayerMouse {
    #[doc(hidden)]
    pub superclass: Mouse,
}
impl_inherits!(PlayerMouse, Mouse);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlayerScripts {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PlayerScripts, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlayerViewService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PlayerViewService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Players {
    #[doc(hidden)]
    pub superclass: Instance,
    pub BanningEnabled: bool,
    pub CharacterAutoLoads: bool,
    pub RespawnTime: f32,
    pub UseStrafingAnimations: bool,
}
impl_inherits!(Players, Instance);
impl Default for Players {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            BanningEnabled: true,
            CharacterAutoLoads: true,
            RespawnTime: 5f32,
            UseStrafingAnimations: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Plugin {
    #[doc(hidden)]
    pub superclass: Instance,
    pub DisableUiDragDetectorDrags: bool,
    pub IsDebuggable: bool,
}
impl_inherits!(Plugin, Instance);
impl Default for Plugin {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            DisableUiDragDetectorDrags: false,
            IsDebuggable: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginAction {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PluginAction, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginCapabilities {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Manifest: String,
}
impl_inherits!(PluginCapabilities, Instance);
impl Default for PluginCapabilities {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self { superclass , Manifest : "{\"Metadata\":{\"TargetDataModels\": [\"Edit\", \"Server\", \"Client\"]},\"Permissions\":{}}" . to_owned () }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginDebugService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PluginDebugService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginDragEvent {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PluginDragEvent, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginGui {
    #[doc(hidden)]
    pub superclass: LayerCollector,
    pub Title: String,
}
impl_inherits!(PluginGui, LayerCollector);
impl Default for PluginGui {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase2d {
            superclass,
            AutoLocalize: false,
            RootLocalizationTable: Ref::none(),
            SelectionBehaviorDown: enums::SelectionBehavior::Escape,
            SelectionBehaviorLeft: enums::SelectionBehavior::Escape,
            SelectionBehaviorRight: enums::SelectionBehavior::Escape,
            SelectionBehaviorUp: enums::SelectionBehavior::Escape,
            SelectionGroup: false,
        };
        let superclass = LayerCollector {
            superclass,
            Enabled: false,
            ResetOnSpawn: false,
            ZIndexBehavior: enums::ZIndexBehavior::Global,
        };
        Self {
            superclass,
            Title: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginGuiService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PluginGuiService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginManagementService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PluginManagementService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginManager {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PluginManager, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginManagerInterface {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PluginManagerInterface, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginMenu {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PluginMenu, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginMouse {
    #[doc(hidden)]
    pub superclass: Mouse,
}
impl_inherits!(PluginMouse, Mouse);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginPolicyService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PluginPolicyService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginToolbar {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PluginToolbar, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginToolbarButton {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PluginToolbarButton, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PointLight {
    #[doc(hidden)]
    pub superclass: Light,
    pub Range: f32,
}
impl_inherits!(PointLight, Light);
impl Default for PointLight {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Light {
            superclass,
            Brightness: 1f32,
            Color: Color3::new(1f32, 1f32, 1f32),
            Enabled: true,
            Shadows: false,
        };
        Self {
            superclass,
            Range: 8f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PointsService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PointsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PolicyService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub IsLuobuServer: enums::TriStateBoolean,
    pub LuobuWhitelisted: enums::TriStateBoolean,
}
impl_inherits!(PolicyService, Instance);
impl Default for PolicyService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            IsLuobuServer: enums::TriStateBoolean::Unknown,
            LuobuWhitelisted: enums::TriStateBoolean::Unknown,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Pose {
    #[doc(hidden)]
    pub superclass: PoseBase,
    pub CFrame: CFrame,
}
impl_inherits!(Pose, PoseBase);
impl Default for Pose {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PoseBase {
            superclass,
            EasingDirection: enums::PoseEasingDirection::In,
            EasingStyle: enums::PoseEasingStyle::Linear,
            Weight: 1f32,
        };
        Self {
            superclass,
            CFrame: CFrame::identity(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PoseBase {
    #[doc(hidden)]
    pub superclass: Instance,
    pub EasingDirection: enums::PoseEasingDirection,
    pub EasingStyle: enums::PoseEasingStyle,
    pub Weight: f32,
}
impl_inherits!(PoseBase, Instance);
impl Default for PoseBase {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            EasingDirection: enums::PoseEasingDirection::In,
            EasingStyle: enums::PoseEasingStyle::Linear,
            Weight: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PostEffect {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Enabled: bool,
}
impl_inherits!(PostEffect, Instance);
impl Default for PostEffect {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Enabled: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PrismaticConstraint {
    #[doc(hidden)]
    pub superclass: SlidingBallConstraint,
}
impl_inherits!(PrismaticConstraint, SlidingBallConstraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ProcessInstancePhysicsService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ProcessInstancePhysicsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ProximityPrompt {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ActionText: String,
    pub AutoLocalize: bool,
    pub ClickablePrompt: bool,
    pub Enabled: bool,
    pub Exclusivity: enums::ProximityPromptExclusivity,
    pub GamepadKeyCode: enums::KeyCode,
    pub HoldDuration: f32,
    pub KeyboardKeyCode: enums::KeyCode,
    pub MaxActivationDistance: f32,
    pub MaxIndicatorDistance: f32,
    pub ObjectText: String,
    pub RequiresLineOfSight: bool,
    pub RootLocalizationTable: Ref,
    pub Style: enums::ProximityPromptStyle,
    pub UiOffset: Vector2,
}
impl_inherits!(ProximityPrompt, Instance);
impl Default for ProximityPrompt {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ActionText: "Interact".to_owned(),
            AutoLocalize: true,
            ClickablePrompt: true,
            Enabled: true,
            Exclusivity: enums::ProximityPromptExclusivity::OnePerButton,
            GamepadKeyCode: enums::KeyCode::ButtonX,
            HoldDuration: 0f32,
            KeyboardKeyCode: enums::KeyCode::E,
            MaxActivationDistance: 10f32,
            MaxIndicatorDistance: 0f32,
            ObjectText: "".to_owned(),
            RequiresLineOfSight: true,
            RootLocalizationTable: Ref::none(),
            Style: enums::ProximityPromptStyle::Default,
            UiOffset: Vector2::new(0f32, 0f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ProximityPromptService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Enabled: bool,
    pub MaxIndicatorsVisible: i32,
    pub MaxPromptsVisible: i32,
}
impl_inherits!(ProximityPromptService, Instance);
impl Default for ProximityPromptService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Enabled: true,
            MaxIndicatorsVisible: 16i32,
            MaxPromptsVisible: 16i32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PublishService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(PublishService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PyramidHandleAdornment {
    #[doc(hidden)]
    pub superclass: HandleAdornment,
    pub Height: f32,
    pub Shading: enums::AdornShading,
    pub Sides: i32,
    pub Size: f32,
}
impl_inherits!(PyramidHandleAdornment, HandleAdornment);
impl Default for PyramidHandleAdornment {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase3d {
            superclass,
            Color3: Color3::new(0.050980393f32, 0.4117647f32, 0.6745098f32),
            Transparency: 0f32,
            Visible: true,
        };
        let superclass = PVAdornment {
            superclass,
            Adornee: Ref::none(),
        };
        let superclass = HandleAdornment {
            superclass,
            AdornCullingMode: enums::AdornCullingMode::Automatic,
            AlwaysOnTop: false,
            CFrame: CFrame::identity(),
            SizeRelativeOffset: Vector3::new(0f32, 0f32, 0f32),
            ZIndex: -1i32,
        };
        Self {
            superclass,
            Height: 2f32,
            Shading: enums::AdornShading::Default,
            Sides: 4i32,
            Size: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct QWidgetPluginGui {
    #[doc(hidden)]
    pub superclass: PluginGui,
}
impl_inherits!(QWidgetPluginGui, PluginGui);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RTAnimationTracker {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(RTAnimationTracker, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RayValue {
    #[doc(hidden)]
    pub superclass: ValueBase,
    pub Value: Ray,
}
impl_inherits!(RayValue, ValueBase);
impl Default for RayValue {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        Self {
            superclass,
            Value: Ray::new(
                Vector3::new(0f32, 0f32, 0f32),
                Vector3::new(0f32, 0f32, 0f32),
            ),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RbxAnalyticsService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(RbxAnalyticsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RecommendationPages {
    #[doc(hidden)]
    pub superclass: Pages,
}
impl_inherits!(RecommendationPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RecommendationService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(RecommendationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadata {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ReflectionMetadata, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataCallbacks {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ReflectionMetadataCallbacks, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataClass {
    #[doc(hidden)]
    pub superclass: ReflectionMetadataItem,
    pub ExplorerImageIndex: i32,
    pub ExplorerOrder: i32,
    pub Insertable: bool,
    pub PreferredParent: String,
    pub ServiceVisibility: enums::ServiceVisibility,
}
impl_inherits!(ReflectionMetadataClass, ReflectionMetadataItem);
impl Default for ReflectionMetadataClass {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ReflectionMetadataItem {
            superclass,
            Browsable: true,
            ClassCategory: "".to_owned(),
            ClientOnly: false,
            Constraint: "".to_owned(),
            Deprecated: false,
            EditingDisabled: false,
            EditorType: "".to_owned(),
            FFlag: "".to_owned(),
            IsBackend: false,
            PropertyOrder: 5000i32,
            ScriptContext: "".to_owned(),
            ServerOnly: false,
            SliderScaling: "".to_owned(),
            UiMaximum: 0f64,
            UiMinimum: 0f64,
            UiNumTicks: 0f64,
        };
        Self {
            superclass,
            ExplorerImageIndex: 0i32,
            ExplorerOrder: 2147483647i32,
            Insertable: true,
            PreferredParent: "".to_owned(),
            ServiceVisibility: enums::ServiceVisibility::Always,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataClasses {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ReflectionMetadataClasses, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataEnum {
    #[doc(hidden)]
    pub superclass: ReflectionMetadataItem,
}
impl_inherits!(ReflectionMetadataEnum, ReflectionMetadataItem);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataEnumItem {
    #[doc(hidden)]
    pub superclass: ReflectionMetadataItem,
}
impl_inherits!(ReflectionMetadataEnumItem, ReflectionMetadataItem);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataEnums {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ReflectionMetadataEnums, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataEvents {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ReflectionMetadataEvents, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataFunctions {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ReflectionMetadataFunctions, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataItem {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Browsable: bool,
    pub ClassCategory: String,
    pub ClientOnly: bool,
    pub Constraint: String,
    pub Deprecated: bool,
    pub EditingDisabled: bool,
    pub EditorType: String,
    pub FFlag: String,
    pub IsBackend: bool,
    pub PropertyOrder: i32,
    pub ScriptContext: String,
    pub ServerOnly: bool,
    pub SliderScaling: String,
    pub UiMaximum: f64,
    pub UiMinimum: f64,
    pub UiNumTicks: f64,
}
impl_inherits!(ReflectionMetadataItem, Instance);
impl Default for ReflectionMetadataItem {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Browsable: false,
            ClassCategory: "".to_owned(),
            ClientOnly: false,
            Constraint: "".to_owned(),
            Deprecated: false,
            EditingDisabled: false,
            EditorType: "".to_owned(),
            FFlag: "".to_owned(),
            IsBackend: false,
            PropertyOrder: 0i32,
            ScriptContext: "".to_owned(),
            ServerOnly: false,
            SliderScaling: "".to_owned(),
            UiMaximum: 0f64,
            UiMinimum: 0f64,
            UiNumTicks: 0f64,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataMember {
    #[doc(hidden)]
    pub superclass: ReflectionMetadataItem,
}
impl_inherits!(ReflectionMetadataMember, ReflectionMetadataItem);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataProperties {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ReflectionMetadataProperties, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataYieldFunctions {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ReflectionMetadataYieldFunctions, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ReflectionService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RelativeGui {
    #[doc(hidden)]
    pub superclass: GuiObject,
}
impl_inherits!(RelativeGui, GuiObject);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RemoteCommandService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(RemoteCommandService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RemoteCursorService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(RemoteCursorService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RemoteDebuggerServer {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(RemoteDebuggerServer, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RemoteEvent {
    #[doc(hidden)]
    pub superclass: BaseRemoteEvent,
}
impl_inherits!(RemoteEvent, BaseRemoteEvent);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RemoteFunction {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(RemoteFunction, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RenderSettings {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AutoFrmLevel: i32,
    pub EagerBulkExecution: bool,
    pub EditQualityLevel: enums::QualityLevel,
    pub EnableVrMode: bool,
    pub ExportMergeByMaterial: bool,
    pub FrameRateManager: enums::FramerateManagerMode,
    pub GraphicsMode: enums::GraphicsMode,
    pub MeshCacheSize: i32,
    pub MeshPartDetailLevel: enums::MeshPartDetailLevel,
    pub QualityLevel: enums::QualityLevel,
    pub ReloadAssets: bool,
    pub RenderCsgTrianglesDebug: bool,
    pub ShowBoundingBoxes: bool,
    pub ViewMode: enums::ViewMode,
}
impl_inherits!(RenderSettings, Instance);
impl Default for RenderSettings {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AutoFrmLevel: 0i32,
            EagerBulkExecution: false,
            EditQualityLevel: enums::QualityLevel::Automatic,
            EnableVrMode: false,
            ExportMergeByMaterial: false,
            FrameRateManager: enums::FramerateManagerMode::Automatic,
            GraphicsMode: enums::GraphicsMode::Automatic,
            MeshCacheSize: 0i32,
            MeshPartDetailLevel: enums::MeshPartDetailLevel::DistanceBased,
            QualityLevel: enums::QualityLevel::Automatic,
            ReloadAssets: false,
            RenderCsgTrianglesDebug: false,
            ShowBoundingBoxes: false,
            ViewMode: enums::ViewMode::None,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RenderingTest {
    #[doc(hidden)]
    pub superclass: Instance,
    pub CFrame: CFrame,
    pub ComparisonDiffThreshold: i32,
    pub ComparisonMethod: enums::RenderingTestComparisonMethod,
    pub ComparisonPsnrThreshold: f32,
    pub Description: String,
    pub FieldOfView: f32,
    pub PerfTest: bool,
    pub QualityAuto: bool,
    pub QualityLevel: i32,
    pub RenderingTestFrameCount: i32,
    pub ShouldSkip: bool,
    pub Ticket: String,
    pub Timeout: i32,
}
impl_inherits!(RenderingTest, Instance);
impl Default for RenderingTest {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            CFrame: CFrame::identity(),
            ComparisonDiffThreshold: 10i32,
            ComparisonMethod: enums::RenderingTestComparisonMethod::psnr,
            ComparisonPsnrThreshold: 50f32,
            Description: "".to_owned(),
            FieldOfView: 70f32,
            PerfTest: false,
            QualityAuto: false,
            QualityLevel: 21i32,
            RenderingTestFrameCount: 20i32,
            ShouldSkip: false,
            Ticket: "".to_owned(),
            Timeout: 30i32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReplicatedFirst {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ReplicatedFirst, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReplicatedStorage {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ReplicatedStorage, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReverbSoundEffect {
    #[doc(hidden)]
    pub superclass: SoundEffect,
    pub DecayTime: f32,
    pub Density: f32,
    pub Diffusion: f32,
    pub DryLevel: f32,
    pub WetLevel: f32,
}
impl_inherits!(ReverbSoundEffect, SoundEffect);
impl Default for ReverbSoundEffect {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundEffect {
            superclass,
            Enabled: true,
            Priority: 0i32,
        };
        Self {
            superclass,
            DecayTime: 1.5f32,
            Density: 1f32,
            Diffusion: 1f32,
            DryLevel: -6f32,
            WetLevel: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RibbonNotificationService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(RibbonNotificationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RigidConstraint {
    #[doc(hidden)]
    pub superclass: Constraint,
}
impl_inherits!(RigidConstraint, Constraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RobloxPluginGuiService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(RobloxPluginGuiService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RobloxReplicatedStorage {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(RobloxReplicatedStorage, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RobloxSerializableInstance {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Data: BinaryString,
}
impl_inherits!(RobloxSerializableInstance, Instance);
impl Default for RobloxSerializableInstance {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Data: b"".as_slice().into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RobloxServerStorage {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(RobloxServerStorage, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RocketPropulsion {
    #[doc(hidden)]
    pub superclass: BodyMover,
    pub CartoonFactor: f32,
    pub MaxSpeed: f32,
    pub MaxThrust: f32,
    pub MaxTorque: Vector3,
    pub Target: Ref,
    pub TargetOffset: Vector3,
    pub TargetRadius: f32,
    pub ThrustD: f32,
    pub ThrustP: f32,
    pub TurnD: f32,
    pub TurnP: f32,
}
impl_inherits!(RocketPropulsion, BodyMover);
impl Default for RocketPropulsion {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BodyMover { superclass };
        Self {
            superclass,
            CartoonFactor: 0.7f32,
            MaxSpeed: 30f32,
            MaxThrust: 4000f32,
            MaxTorque: Vector3::new(400000f32, 400000f32, 0f32),
            Target: Ref::none(),
            TargetOffset: Vector3::new(0f32, 0f32, 0f32),
            TargetRadius: 4f32,
            ThrustD: 0.001f32,
            ThrustP: 5f32,
            TurnD: 500f32,
            TurnP: 3000f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RodConstraint {
    #[doc(hidden)]
    pub superclass: Constraint,
    pub Length: f32,
    pub LimitAngle0: f32,
    pub LimitAngle1: f32,
    pub LimitsEnabled: bool,
    pub Thickness: f32,
}
impl_inherits!(RodConstraint, Constraint);
impl Default for RodConstraint {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Constraint {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            Color: BrickColor::from_number(26u16).unwrap(),
            Enabled: true,
            Visible: false,
        };
        Self {
            superclass,
            Length: 5f32,
            LimitAngle0: 90f32,
            LimitAngle1: 90f32,
            LimitsEnabled: false,
            Thickness: 0.1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RomarkRbxAnalyticsService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(RomarkRbxAnalyticsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RomarkService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(RomarkService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RootImportData {
    #[doc(hidden)]
    pub superclass: BaseImportData,
    pub AddModelToInventory: bool,
    pub Anchored: bool,
    pub AnimationIdForRestPose: f32,
    pub ExistingPackageId: String,
    pub ImportAsModelAsset: bool,
    pub ImportAsPackage: bool,
    pub InsertInWorkspace: bool,
    pub InsertWithScenePosition: bool,
    pub InvertNegativeFaces: bool,
    pub KeepZeroInfluenceBones: bool,
    pub MergeMeshes: bool,
    pub PreferredUploadId: i64,
    pub RestPose: enums::RestPose,
    pub RigScale: enums::RigScale,
    pub RigType: enums::RigType,
    pub RigVisualization: bool,
    pub ScaleUnit: enums::MeshScaleUnit,
    pub UseSceneOriginAsPivot: bool,
    pub UsesCages: bool,
    pub ValidateUgcBody: bool,
    pub WorldForward: enums::NormalId,
    pub WorldUp: enums::NormalId,
}
impl_inherits!(RootImportData, BaseImportData);
impl Default for RootImportData {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BaseImportData {
            superclass,
            ImportName: "".to_owned(),
            ShouldImport: false,
        };
        Self {
            superclass,
            AddModelToInventory: false,
            Anchored: false,
            AnimationIdForRestPose: 0f32,
            ExistingPackageId: "".to_owned(),
            ImportAsModelAsset: false,
            ImportAsPackage: false,
            InsertInWorkspace: false,
            InsertWithScenePosition: false,
            InvertNegativeFaces: false,
            KeepZeroInfluenceBones: false,
            MergeMeshes: false,
            PreferredUploadId: 0i64,
            RestPose: enums::RestPose::Default,
            RigScale: enums::RigScale::Default,
            RigType: enums::RigType::R15,
            RigVisualization: false,
            ScaleUnit: enums::MeshScaleUnit::Stud,
            UseSceneOriginAsPivot: false,
            UsesCages: false,
            ValidateUgcBody: false,
            WorldForward: enums::NormalId::Right,
            WorldUp: enums::NormalId::Right,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RopeConstraint {
    #[doc(hidden)]
    pub superclass: Constraint,
    pub Length: f32,
    pub Restitution: f32,
    pub Thickness: f32,
    pub WinchEnabled: bool,
    pub WinchForce: f32,
    pub WinchResponsiveness: f32,
    pub WinchSpeed: f32,
    pub WinchTarget: f32,
}
impl_inherits!(RopeConstraint, Constraint);
impl Default for RopeConstraint {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Constraint {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            Color: BrickColor::from_number(25u16).unwrap(),
            Enabled: true,
            Visible: false,
        };
        Self {
            superclass,
            Length: 5f32,
            Restitution: 0f32,
            Thickness: 0.1f32,
            WinchEnabled: false,
            WinchForce: 10000f32,
            WinchResponsiveness: 45f32,
            WinchSpeed: 2f32,
            WinchTarget: 5f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Rotate {
    #[doc(hidden)]
    pub superclass: JointInstance,
}
impl_inherits!(Rotate, JointInstance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RotateP {
    #[doc(hidden)]
    pub superclass: DynamicRotate,
}
impl_inherits!(RotateP, DynamicRotate);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RotateV {
    #[doc(hidden)]
    pub superclass: DynamicRotate,
}
impl_inherits!(RotateV, DynamicRotate);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RotationCurve {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ValuesAndTimes: BinaryString,
}
impl_inherits!(RotationCurve, Instance);
impl Default for RotationCurve {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ValuesAndTimes: b"\x01\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0".as_slice().into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RtMessagingService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(RtMessagingService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RunService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(RunService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RunningAverageItemDouble {
    #[doc(hidden)]
    pub superclass: StatsItem,
}
impl_inherits!(RunningAverageItemDouble, StatsItem);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RunningAverageItemInt {
    #[doc(hidden)]
    pub superclass: StatsItem,
}
impl_inherits!(RunningAverageItemInt, StatsItem);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RunningAverageTimeIntervalItem {
    #[doc(hidden)]
    pub superclass: StatsItem,
}
impl_inherits!(RunningAverageTimeIntervalItem, StatsItem);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RuntimeContentService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(RuntimeContentService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RuntimeScriptService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(RuntimeScriptService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SafetyService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub IsCaptureModeForReport: bool,
}
impl_inherits!(SafetyService, Instance);
impl Default for SafetyService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            IsCaptureModeForReport: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScreenGui {
    #[doc(hidden)]
    pub superclass: LayerCollector,
    pub ClipToDeviceSafeArea: bool,
    pub DisplayOrder: i32,
    pub SafeAreaCompatibility: enums::SafeAreaCompatibility,
    pub ScreenInsets: enums::ScreenInsets,
}
impl_inherits!(ScreenGui, LayerCollector);
impl Default for ScreenGui {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase2d {
            superclass,
            AutoLocalize: true,
            RootLocalizationTable: Ref::none(),
            SelectionBehaviorDown: enums::SelectionBehavior::Escape,
            SelectionBehaviorLeft: enums::SelectionBehavior::Escape,
            SelectionBehaviorRight: enums::SelectionBehavior::Escape,
            SelectionBehaviorUp: enums::SelectionBehavior::Escape,
            SelectionGroup: false,
        };
        let superclass = LayerCollector {
            superclass,
            Enabled: true,
            ResetOnSpawn: true,
            ZIndexBehavior: enums::ZIndexBehavior::Global,
        };
        Self {
            superclass,
            ClipToDeviceSafeArea: true,
            DisplayOrder: 0i32,
            SafeAreaCompatibility: enums::SafeAreaCompatibility::FullscreenExtension,
            ScreenInsets: enums::ScreenInsets::CoreUISafeInsets,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScreenshotCapture {
    #[doc(hidden)]
    pub superclass: Capture,
}
impl_inherits!(ScreenshotCapture, Capture);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScreenshotHud {
    #[doc(hidden)]
    pub superclass: Instance,
    pub CameraButtonIcon: ContentId,
    pub CameraButtonPosition: UDim2,
    pub CloseButtonPosition: UDim2,
    pub CloseWhenScreenshotTaken: bool,
    pub ExperienceNameOverlayEnabled: bool,
    pub HideCoreGuiForCaptures: bool,
    pub HidePlayerGuiForCaptures: bool,
    pub OverlayFont: enums::Font,
    pub UsernameOverlayEnabled: bool,
    pub Visible: bool,
}
impl_inherits!(ScreenshotHud, Instance);
impl Default for ScreenshotHud {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            CameraButtonIcon: "".into(),
            CameraButtonPosition: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            CloseButtonPosition: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            CloseWhenScreenshotTaken: false,
            ExperienceNameOverlayEnabled: false,
            HideCoreGuiForCaptures: false,
            HidePlayerGuiForCaptures: false,
            OverlayFont: enums::Font::Legacy,
            UsernameOverlayEnabled: false,
            Visible: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Script {
    #[doc(hidden)]
    pub superclass: BaseScript,
    pub Source: String,
}
impl_inherits!(Script, BaseScript);
impl Default for Script {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = LuaSourceContainer {
            superclass,
            ScriptGuid: "".to_owned(),
        };
        let superclass = BaseScript {
            superclass,
            Disabled: false,
            LinkedSource: "".into(),
            RunContext: enums::RunContext::Legacy,
        };
        Self {
            superclass,
            Source: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptBuilder {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ScriptBuilder, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptChangeService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ScriptChangeService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptCloneWatcher {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ScriptCloneWatcher, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptCloneWatcherHelper {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ScriptCloneWatcherHelper, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptCommitService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ScriptCommitService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptContext {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ScriptContext, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptDebugger {
    #[doc(hidden)]
    pub superclass: Instance,
    pub CoreScriptIdentifier: String,
    pub ScriptGuid: String,
}
impl_inherits!(ScriptDebugger, Instance);
impl Default for ScriptDebugger {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            CoreScriptIdentifier: "".to_owned(),
            ScriptGuid: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptDocument {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ScriptDocument, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptEditorService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ScriptEditorService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptProfilerService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ScriptProfilerService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptRegistrationService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ScriptRegistrationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptRuntime {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ScriptRuntime, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ScriptService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScrollingFrame {
    #[doc(hidden)]
    pub superclass: GuiObject,
    pub AutomaticCanvasSize: enums::AutomaticSize,
    pub BottomImageContent: Content,
    pub CanvasPosition: Vector2,
    pub CanvasSize: UDim2,
    pub ElasticBehavior: enums::ElasticBehavior,
    pub HorizontalScrollBarInset: enums::ScrollBarInset,
    pub MidImageContent: Content,
    pub ScrollBarImageColor3: Color3,
    pub ScrollBarImageTransparency: f32,
    pub ScrollBarThickness: i32,
    pub ScrollingDirection: enums::ScrollingDirection,
    pub ScrollingEnabled: bool,
    pub TopImageContent: Content,
    pub VerticalScrollBarInset: enums::ScrollBarInset,
    pub VerticalScrollBarPosition: enums::VerticalScrollBarPosition,
}
impl_inherits!(ScrollingFrame, GuiObject);
impl Default for ScrollingFrame {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase2d {
            superclass,
            AutoLocalize: true,
            RootLocalizationTable: Ref::none(),
            SelectionBehaviorDown: enums::SelectionBehavior::Escape,
            SelectionBehaviorLeft: enums::SelectionBehavior::Escape,
            SelectionBehaviorRight: enums::SelectionBehavior::Escape,
            SelectionBehaviorUp: enums::SelectionBehavior::Escape,
            SelectionGroup: true,
        };
        let superclass = GuiObject {
            superclass,
            Active: false,
            AnchorPoint: Vector2::new(0f32, 0f32),
            AutomaticSize: enums::AutomaticSize::None,
            BackgroundColor3: Color3::new(0.6392157f32, 0.63529414f32, 0.64705884f32),
            BackgroundTransparency: 0f32,
            BorderColor3: Color3::new(0.105882354f32, 0.16470589f32, 0.20784314f32),
            BorderMode: enums::BorderMode::Outline,
            BorderSizePixel: 1i32,
            ClipsDescendants: true,
            Draggable: false,
            Interactable: true,
            LayoutOrder: 0i32,
            NextSelectionDown: Ref::none(),
            NextSelectionLeft: Ref::none(),
            NextSelectionRight: Ref::none(),
            NextSelectionUp: Ref::none(),
            Position: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            Rotation: 0f32,
            Selectable: true,
            SelectionImageObject: Ref::none(),
            SelectionOrder: 0i32,
            Size: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            SizeConstraint: enums::SizeConstraint::RelativeXY,
            Visible: true,
            ZIndex: 1i32,
        };
        Self {
            superclass,
            AutomaticCanvasSize: enums::AutomaticSize::None,
            BottomImageContent: Content::from_uri(
                "rbxasset://textures/ui/Scroll/scroll-bottom.png",
            ),
            CanvasPosition: Vector2::new(0f32, 0f32),
            CanvasSize: UDim2::new(UDim::new(0f32, 0i32), UDim::new(2f32, 0i32)),
            ElasticBehavior: enums::ElasticBehavior::WhenScrollable,
            HorizontalScrollBarInset: enums::ScrollBarInset::None,
            MidImageContent: Content::from_uri("rbxasset://textures/ui/Scroll/scroll-middle.png"),
            ScrollBarImageColor3: Color3::new(1f32, 1f32, 1f32),
            ScrollBarImageTransparency: 0f32,
            ScrollBarThickness: 12i32,
            ScrollingDirection: enums::ScrollingDirection::XY,
            ScrollingEnabled: true,
            TopImageContent: Content::from_uri("rbxasset://textures/ui/Scroll/scroll-top.png"),
            VerticalScrollBarInset: enums::ScrollBarInset::None,
            VerticalScrollBarPosition: enums::VerticalScrollBarPosition::Right,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Seat {
    #[doc(hidden)]
    pub superclass: Part,
    pub Disabled: bool,
}
impl_inherits!(Seat, Part);
impl Default for Seat {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PVInstance { superclass };
        let superclass = BasePart {
            superclass,
            Anchored: false,
            AudioCanCollide: true,
            BackParamA: -0.5f32,
            BackParamB: 0.5f32,
            BackSurface: enums::SurfaceType::Smooth,
            BackSurfaceInput: enums::InputType::NoInput,
            BottomParamA: -0.5f32,
            BottomParamB: 0.5f32,
            BottomSurface: enums::SurfaceType::Inlet,
            BottomSurfaceInput: enums::InputType::NoInput,
            CFrame: CFrame::identity(),
            CanCollide: true,
            CanQuery: true,
            CanTouch: true,
            CastShadow: true,
            CollisionGroup: "Default".to_owned(),
            CollisionGroupId: 0i32,
            CustomPhysicalProperties: PhysicalProperties::Default,
            EnableFluidForces: true,
            FrontParamA: -0.5f32,
            FrontParamB: 0.5f32,
            FrontSurface: enums::SurfaceType::Smooth,
            FrontSurfaceInput: enums::InputType::NoInput,
            LeftParamA: -0.5f32,
            LeftParamB: 0.5f32,
            LeftSurface: enums::SurfaceType::Smooth,
            LeftSurfaceInput: enums::InputType::NoInput,
            Locked: false,
            Massless: false,
            Material: enums::Material::Plastic,
            PivotOffset: CFrame::identity(),
            Reflectance: 0f32,
            RightParamA: -0.5f32,
            RightParamB: 0.5f32,
            RightSurface: enums::SurfaceType::Smooth,
            RightSurfaceInput: enums::InputType::NoInput,
            RootPriority: 0i32,
            RotVelocity: Vector3::new(0f32, 0f32, 0f32),
            TopParamA: -0.5f32,
            TopParamB: 0.5f32,
            TopSurface: enums::SurfaceType::Studs,
            TopSurfaceInput: enums::InputType::NoInput,
            Transparency: 0f32,
            Velocity: Vector3::new(0f32, 0f32, 0f32),
        };
        let superclass = FormFactorPart { superclass };
        let superclass = Part { superclass };
        Self {
            superclass,
            Disabled: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Selection {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(Selection, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionBox {
    #[doc(hidden)]
    pub superclass: InstanceAdornment,
    pub LineThickness: f32,
    pub StudioSelectionBox: bool,
    pub SurfaceColor3: Color3,
    pub SurfaceTransparency: f32,
}
impl_inherits!(SelectionBox, InstanceAdornment);
impl Default for SelectionBox {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase3d {
            superclass,
            Color3: Color3::new(0.050980393f32, 0.4117647f32, 0.6745098f32),
            Transparency: 0f32,
            Visible: true,
        };
        let superclass = InstanceAdornment {
            superclass,
            Adornee: Ref::none(),
        };
        Self {
            superclass,
            LineThickness: 0.15f32,
            StudioSelectionBox: false,
            SurfaceColor3: Color3::new(0.050980397f32, 0.41176474f32, 0.6745098f32),
            SurfaceTransparency: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SelectionHighlightManager {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(SelectionHighlightManager, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionLasso {
    #[doc(hidden)]
    pub superclass: GuiBase3d,
    pub Humanoid: Ref,
}
impl_inherits!(SelectionLasso, GuiBase3d);
impl Default for SelectionLasso {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase3d {
            superclass,
            Color3: Color3::new(0f32, 0f32, 0f32),
            Transparency: 0f32,
            Visible: false,
        };
        Self {
            superclass,
            Humanoid: Ref::none(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionPartLasso {
    #[doc(hidden)]
    pub superclass: SelectionLasso,
    pub Part: Ref,
}
impl_inherits!(SelectionPartLasso, SelectionLasso);
impl Default for SelectionPartLasso {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase3d {
            superclass,
            Color3: Color3::new(0.050980393f32, 0.4117647f32, 0.6745098f32),
            Transparency: 0f32,
            Visible: true,
        };
        let superclass = SelectionLasso {
            superclass,
            Humanoid: Ref::none(),
        };
        Self {
            superclass,
            Part: Ref::none(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionPointLasso {
    #[doc(hidden)]
    pub superclass: SelectionLasso,
    pub Point: Vector3,
}
impl_inherits!(SelectionPointLasso, SelectionLasso);
impl Default for SelectionPointLasso {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase3d {
            superclass,
            Color3: Color3::new(0.050980393f32, 0.4117647f32, 0.6745098f32),
            Transparency: 0f32,
            Visible: true,
        };
        let superclass = SelectionLasso {
            superclass,
            Humanoid: Ref::none(),
        };
        Self {
            superclass,
            Point: Vector3::new(0f32, 0f32, 0f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionSphere {
    #[doc(hidden)]
    pub superclass: PVAdornment,
    pub SurfaceColor3: Color3,
    pub SurfaceTransparency: f32,
}
impl_inherits!(SelectionSphere, PVAdornment);
impl Default for SelectionSphere {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase3d {
            superclass,
            Color3: Color3::new(0.050980393f32, 0.4117647f32, 0.6745098f32),
            Transparency: 0f32,
            Visible: true,
        };
        let superclass = PVAdornment {
            superclass,
            Adornee: Ref::none(),
        };
        Self {
            superclass,
            SurfaceColor3: Color3::new(0.050980397f32, 0.41176474f32, 0.6745098f32),
            SurfaceTransparency: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SensorBase {
    #[doc(hidden)]
    pub superclass: Instance,
    pub UpdateType: enums::SensorUpdateType,
}
impl_inherits!(SensorBase, Instance);
impl Default for SensorBase {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            UpdateType: enums::SensorUpdateType::OnRead,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SerializationService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(SerializationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ServerReplicator {
    #[doc(hidden)]
    pub superclass: NetworkReplicator,
}
impl_inherits!(ServerReplicator, NetworkReplicator);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ServerScriptService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub LoadStringEnabled: bool,
}
impl_inherits!(ServerScriptService, Instance);
impl Default for ServerScriptService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            LoadStringEnabled: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ServerStorage {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ServerStorage, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ServiceProvider {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ServiceProvider, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ServiceVisibilityService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub HiddenServices: BinaryString,
    pub VisibleServices: BinaryString,
}
impl_inherits!(ServiceVisibilityService, Instance);
impl Default for ServiceVisibilityService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            HiddenServices: b"\0\0\0\0".as_slice().into(),
            VisibleServices: b"\0\0\0\0".as_slice().into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SessionCheckService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(SessionCheckService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SessionService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(SessionService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SharedTableRegistry {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(SharedTableRegistry, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Shirt {
    #[doc(hidden)]
    pub superclass: Clothing,
    pub ShirtTemplate: ContentId,
}
impl_inherits!(Shirt, Clothing);
impl Default for Shirt {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = CharacterAppearance { superclass };
        let superclass = Clothing {
            superclass,
            Color3: Color3::new(1f32, 1f32, 1f32),
        };
        Self {
            superclass,
            ShirtTemplate: "".into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ShirtGraphic {
    #[doc(hidden)]
    pub superclass: CharacterAppearance,
    pub Color3: Color3,
    pub Graphic: ContentId,
}
impl_inherits!(ShirtGraphic, CharacterAppearance);
impl Default for ShirtGraphic {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = CharacterAppearance { superclass };
        Self {
            superclass,
            Color3: Color3::new(1f32, 1f32, 1f32),
            Graphic: "".into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SkateboardController {
    #[doc(hidden)]
    pub superclass: Controller,
}
impl_inherits!(SkateboardController, Controller);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SkateboardPlatform {
    #[doc(hidden)]
    pub superclass: Part,
    pub Steer: i32,
    pub StickyWheels: bool,
    pub Throttle: i32,
}
impl_inherits!(SkateboardPlatform, Part);
impl Default for SkateboardPlatform {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PVInstance { superclass };
        let superclass = BasePart {
            superclass,
            Anchored: false,
            AudioCanCollide: true,
            BackParamA: -0.5f32,
            BackParamB: 0.5f32,
            BackSurface: enums::SurfaceType::Smooth,
            BackSurfaceInput: enums::InputType::NoInput,
            BottomParamA: -0.5f32,
            BottomParamB: 0.5f32,
            BottomSurface: enums::SurfaceType::Inlet,
            BottomSurfaceInput: enums::InputType::NoInput,
            CFrame: CFrame::identity(),
            CanCollide: true,
            CanQuery: true,
            CanTouch: true,
            CastShadow: true,
            CollisionGroup: "Default".to_owned(),
            CollisionGroupId: 0i32,
            CustomPhysicalProperties: PhysicalProperties::Default,
            EnableFluidForces: true,
            FrontParamA: -0.5f32,
            FrontParamB: 0.5f32,
            FrontSurface: enums::SurfaceType::Smooth,
            FrontSurfaceInput: enums::InputType::NoInput,
            LeftParamA: -0.5f32,
            LeftParamB: 0.5f32,
            LeftSurface: enums::SurfaceType::Smooth,
            LeftSurfaceInput: enums::InputType::NoInput,
            Locked: false,
            Massless: false,
            Material: enums::Material::Plastic,
            PivotOffset: CFrame::identity(),
            Reflectance: 0f32,
            RightParamA: -0.5f32,
            RightParamB: 0.5f32,
            RightSurface: enums::SurfaceType::Smooth,
            RightSurfaceInput: enums::InputType::NoInput,
            RootPriority: 0i32,
            RotVelocity: Vector3::new(0f32, 0f32, 0f32),
            TopParamA: -0.5f32,
            TopParamB: 0.5f32,
            TopSurface: enums::SurfaceType::Studs,
            TopSurfaceInput: enums::InputType::NoInput,
            Transparency: 0f32,
            Velocity: Vector3::new(0f32, 0f32, 0f32),
        };
        let superclass = FormFactorPart { superclass };
        let superclass = Part { superclass };
        Self {
            superclass,
            Steer: 0i32,
            StickyWheels: true,
            Throttle: 0i32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Skin {
    #[doc(hidden)]
    pub superclass: CharacterAppearance,
    pub SkinColor: BrickColor,
}
impl_inherits!(Skin, CharacterAppearance);
impl Default for Skin {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = CharacterAppearance { superclass };
        Self {
            superclass,
            SkinColor: BrickColor::from_number(226u16).unwrap(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Sky {
    #[doc(hidden)]
    pub superclass: Instance,
    pub CelestialBodiesShown: bool,
    pub MoonAngularSize: f32,
    pub MoonTextureId: ContentId,
    pub SkyboxBk: ContentId,
    pub SkyboxDn: ContentId,
    pub SkyboxFt: ContentId,
    pub SkyboxLf: ContentId,
    pub SkyboxOrientation: Vector3,
    pub SkyboxRt: ContentId,
    pub SkyboxUp: ContentId,
    pub StarCount: i32,
    pub SunAngularSize: f32,
    pub SunTextureId: ContentId,
}
impl_inherits!(Sky, Instance);
impl Default for Sky {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            CelestialBodiesShown: true,
            MoonAngularSize: 11f32,
            MoonTextureId: "rbxasset://sky/moon.jpg".into(),
            SkyboxBk: "rbxasset://textures/sky/sky512_bk.tex".into(),
            SkyboxDn: "rbxasset://textures/sky/sky512_dn.tex".into(),
            SkyboxFt: "rbxasset://textures/sky/sky512_ft.tex".into(),
            SkyboxLf: "rbxasset://textures/sky/sky512_lf.tex".into(),
            SkyboxOrientation: Vector3::new(0f32, 0f32, 0f32),
            SkyboxRt: "rbxasset://textures/sky/sky512_rt.tex".into(),
            SkyboxUp: "rbxasset://textures/sky/sky512_up.tex".into(),
            StarCount: 3000i32,
            SunAngularSize: 21f32,
            SunTextureId: "rbxasset://sky/sun.jpg".into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SlidingBallConstraint {
    #[doc(hidden)]
    pub superclass: Constraint,
    pub ActuatorType: enums::ActuatorType,
    pub LimitsEnabled: bool,
    pub LinearResponsiveness: f32,
    pub LowerLimit: f32,
    pub MotorMaxAcceleration: f32,
    pub MotorMaxForce: f32,
    pub Restitution: f32,
    pub ServoMaxForce: f32,
    pub Size: f32,
    pub SoftlockServoUponReachingTarget: bool,
    pub Speed: f32,
    pub TargetPosition: f32,
    pub UpperLimit: f32,
    pub Velocity: f32,
}
impl_inherits!(SlidingBallConstraint, Constraint);
impl Default for SlidingBallConstraint {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Constraint {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            Color: BrickColor::from_number(194u16).unwrap(),
            Enabled: false,
            Visible: false,
        };
        Self {
            superclass,
            ActuatorType: enums::ActuatorType::None,
            LimitsEnabled: false,
            LinearResponsiveness: 0f32,
            LowerLimit: 0f32,
            MotorMaxAcceleration: 0f32,
            MotorMaxForce: 0f32,
            Restitution: 0f32,
            ServoMaxForce: 0f32,
            Size: 0f32,
            SoftlockServoUponReachingTarget: false,
            Speed: 0f32,
            TargetPosition: 0f32,
            UpperLimit: 0f32,
            Velocity: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SlimContentProvider {
    #[doc(hidden)]
    pub superclass: CacheableContentProvider,
}
impl_inherits!(SlimContentProvider, CacheableContentProvider);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SlimService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(SlimService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Smoke {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Color: Color3,
    pub Enabled: bool,
    pub TimeScale: f32,
}
impl_inherits!(Smoke, Instance);
impl Default for Smoke {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Color: Color3::new(1f32, 1f32, 1f32),
            Enabled: true,
            TimeScale: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SmoothVoxelsUpgraderService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(SmoothVoxelsUpgraderService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Snap {
    #[doc(hidden)]
    pub superclass: JointInstance,
}
impl_inherits!(Snap, JointInstance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SnippetService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(SnippetService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SocialService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(SocialService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SolidModelContentProvider {
    #[doc(hidden)]
    pub superclass: CacheableContentProvider,
}
impl_inherits!(SolidModelContentProvider, CacheableContentProvider);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Sound {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AudioContent: Content,
    pub IsMutedForCapture: bool,
    pub LoopRegion: NumberRange,
    pub Looped: bool,
    pub PlayOnRemove: bool,
    pub PlaybackRegion: NumberRange,
    pub PlaybackRegionsEnabled: bool,
    pub PlaybackSpeed: f32,
    pub Playing: bool,
    pub RollOffMode: enums::RollOffMode,
    pub SoundGroup: Ref,
    pub TimePosition: f64,
    pub Volume: f32,
}
impl_inherits!(Sound, Instance);
impl Default for Sound {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AudioContent: Content::none(),
            IsMutedForCapture: false,
            LoopRegion: NumberRange::new(0f32, 60000f32),
            Looped: false,
            PlayOnRemove: false,
            PlaybackRegion: NumberRange::new(0f32, 60000f32),
            PlaybackRegionsEnabled: false,
            PlaybackSpeed: 1f32,
            Playing: false,
            RollOffMode: enums::RollOffMode::Inverse,
            SoundGroup: Ref::none(),
            TimePosition: 0f64,
            Volume: 0.5f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SoundEffect {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Enabled: bool,
    pub Priority: i32,
}
impl_inherits!(SoundEffect, Instance);
impl Default for SoundEffect {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Enabled: false,
            Priority: 0i32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SoundGroup {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Volume: f32,
}
impl_inherits!(SoundGroup, Instance);
impl Default for SoundGroup {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Volume: 0.5f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SoundService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AcousticSimulationEnabled: bool,
    pub AmbientReverb: enums::ReverbType,
    pub AudioApiByDefault: enums::RolloutState,
    pub CharacterSoundsUseNewApi: enums::RolloutState,
    pub DefaultListenerLocation: enums::ListenerLocation,
    pub DistanceFactor: f32,
    pub DopplerScale: f32,
    pub IsNewExpForAudioApiByDefault: bool,
    pub RespectFilteringEnabled: bool,
    pub RolloffScale: f32,
    pub VolumetricAudio: enums::VolumetricAudio,
}
impl_inherits!(SoundService, Instance);
impl Default for SoundService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AcousticSimulationEnabled: false,
            AmbientReverb: enums::ReverbType::NoReverb,
            AudioApiByDefault: enums::RolloutState::Default,
            CharacterSoundsUseNewApi: enums::RolloutState::Default,
            DefaultListenerLocation: enums::ListenerLocation::Default,
            DistanceFactor: 3.33f32,
            DopplerScale: 1f32,
            IsNewExpForAudioApiByDefault: false,
            RespectFilteringEnabled: false,
            RolloffScale: 1f32,
            VolumetricAudio: enums::VolumetricAudio::Automatic,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SoundShimService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(SoundShimService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Sparkles {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Enabled: bool,
    pub SparkleColor: Color3,
    pub TimeScale: f32,
}
impl_inherits!(Sparkles, Instance);
impl Default for Sparkles {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Enabled: true,
            SparkleColor: Color3::new(0.5647059f32, 0.098039225f32, 1f32),
            TimeScale: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SpawnLocation {
    #[doc(hidden)]
    pub superclass: Part,
    pub AllowTeamChangeOnTouch: bool,
    pub Duration: i32,
    pub Enabled: bool,
    pub Neutral: bool,
    pub TeamColor: BrickColor,
}
impl_inherits!(SpawnLocation, Part);
impl Default for SpawnLocation {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PVInstance { superclass };
        let superclass = BasePart {
            superclass,
            Anchored: false,
            AudioCanCollide: true,
            BackParamA: -0.5f32,
            BackParamB: 0.5f32,
            BackSurface: enums::SurfaceType::Smooth,
            BackSurfaceInput: enums::InputType::NoInput,
            BottomParamA: -0.5f32,
            BottomParamB: 0.5f32,
            BottomSurface: enums::SurfaceType::Inlet,
            BottomSurfaceInput: enums::InputType::NoInput,
            CFrame: CFrame::identity(),
            CanCollide: true,
            CanQuery: true,
            CanTouch: true,
            CastShadow: true,
            CollisionGroup: "Default".to_owned(),
            CollisionGroupId: 0i32,
            CustomPhysicalProperties: PhysicalProperties::Default,
            EnableFluidForces: true,
            FrontParamA: -0.5f32,
            FrontParamB: 0.5f32,
            FrontSurface: enums::SurfaceType::Smooth,
            FrontSurfaceInput: enums::InputType::NoInput,
            LeftParamA: -0.5f32,
            LeftParamB: 0.5f32,
            LeftSurface: enums::SurfaceType::Smooth,
            LeftSurfaceInput: enums::InputType::NoInput,
            Locked: false,
            Massless: false,
            Material: enums::Material::Plastic,
            PivotOffset: CFrame::identity(),
            Reflectance: 0f32,
            RightParamA: -0.5f32,
            RightParamB: 0.5f32,
            RightSurface: enums::SurfaceType::Smooth,
            RightSurfaceInput: enums::InputType::NoInput,
            RootPriority: 0i32,
            RotVelocity: Vector3::new(0f32, 0f32, 0f32),
            TopParamA: -0.5f32,
            TopParamB: 0.5f32,
            TopSurface: enums::SurfaceType::Studs,
            TopSurfaceInput: enums::InputType::NoInput,
            Transparency: 0f32,
            Velocity: Vector3::new(0f32, 0f32, 0f32),
        };
        let superclass = FormFactorPart { superclass };
        let superclass = Part { superclass };
        Self {
            superclass,
            AllowTeamChangeOnTouch: false,
            Duration: 10i32,
            Enabled: true,
            Neutral: true,
            TeamColor: BrickColor::from_number(194u16).unwrap(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SpawnerService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(SpawnerService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SpecialMesh {
    #[doc(hidden)]
    pub superclass: FileMesh,
    pub MeshType: enums::MeshType,
}
impl_inherits!(SpecialMesh, FileMesh);
impl Default for SpecialMesh {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = DataModelMesh {
            superclass,
            Offset: Vector3::new(0f32, 0f32, 0f32),
            Scale: Vector3::new(1f32, 1f32, 1f32),
            VertexColor: Vector3::new(1f32, 1f32, 1f32),
        };
        let superclass = FileMesh {
            superclass,
            MeshId: "".into(),
            TextureId: "".into(),
        };
        Self {
            superclass,
            MeshType: enums::MeshType::Head,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SphereHandleAdornment {
    #[doc(hidden)]
    pub superclass: HandleAdornment,
    pub Radius: f32,
    pub Shading: enums::AdornShading,
}
impl_inherits!(SphereHandleAdornment, HandleAdornment);
impl Default for SphereHandleAdornment {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase3d {
            superclass,
            Color3: Color3::new(0.050980393f32, 0.4117647f32, 0.6745098f32),
            Transparency: 0f32,
            Visible: true,
        };
        let superclass = PVAdornment {
            superclass,
            Adornee: Ref::none(),
        };
        let superclass = HandleAdornment {
            superclass,
            AdornCullingMode: enums::AdornCullingMode::Automatic,
            AlwaysOnTop: false,
            CFrame: CFrame::identity(),
            SizeRelativeOffset: Vector3::new(0f32, 0f32, 0f32),
            ZIndex: -1i32,
        };
        Self {
            superclass,
            Radius: 1f32,
            Shading: enums::AdornShading::Default,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SpotLight {
    #[doc(hidden)]
    pub superclass: Light,
    pub Angle: f32,
    pub Face: enums::NormalId,
    pub Range: f32,
}
impl_inherits!(SpotLight, Light);
impl Default for SpotLight {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Light {
            superclass,
            Brightness: 1f32,
            Color: Color3::new(1f32, 1f32, 1f32),
            Enabled: true,
            Shadows: false,
        };
        Self {
            superclass,
            Angle: 90f32,
            Face: enums::NormalId::Front,
            Range: 16f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SpringConstraint {
    #[doc(hidden)]
    pub superclass: Constraint,
    pub Coils: f32,
    pub Damping: f32,
    pub FreeLength: f32,
    pub LimitsEnabled: bool,
    pub MaxForce: f32,
    pub MaxLength: f32,
    pub MinLength: f32,
    pub Radius: f32,
    pub Stiffness: f32,
    pub Thickness: f32,
}
impl_inherits!(SpringConstraint, Constraint);
impl Default for SpringConstraint {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Constraint {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            Color: BrickColor::from_number(200u16).unwrap(),
            Enabled: true,
            Visible: false,
        };
        Self {
            superclass,
            Coils: 3f32,
            Damping: 0f32,
            FreeLength: 1f32,
            LimitsEnabled: false,
            MaxForce: f32::INFINITY,
            MaxLength: 5f32,
            MinLength: 0f32,
            Radius: 0.4f32,
            Stiffness: 0f32,
            Thickness: 0.1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StackFrame {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StackFrame, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StandalonePluginScripts {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StandalonePluginScripts, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StandardPages {
    #[doc(hidden)]
    pub superclass: Pages,
}
impl_inherits!(StandardPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StartPageService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StartPageService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StarterCharacterScripts {
    #[doc(hidden)]
    pub superclass: StarterPlayerScripts,
}
impl_inherits!(StarterCharacterScripts, StarterPlayerScripts);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StarterGear {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StarterGear, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StarterGui {
    #[doc(hidden)]
    pub superclass: BasePlayerGui,
    pub ResetPlayerGuiOnSpawn: bool,
    pub RtlTextSupport: enums::RtlTextSupport,
    pub ScreenOrientation: enums::ScreenOrientation,
    pub ShowDevelopmentGui: bool,
    pub StudioDefaultStyleSheet: Ref,
    pub StudioInsertWidgetLayerCollectorAutoLinkStyleSheet: Ref,
    pub VirtualCursorMode: enums::VirtualCursorMode,
}
impl_inherits!(StarterGui, BasePlayerGui);
impl Default for StarterGui {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BasePlayerGui { superclass };
        Self {
            superclass,
            ResetPlayerGuiOnSpawn: true,
            RtlTextSupport: enums::RtlTextSupport::Default,
            ScreenOrientation: enums::ScreenOrientation::LandscapeSensor,
            ShowDevelopmentGui: true,
            StudioDefaultStyleSheet: Ref::none(),
            StudioInsertWidgetLayerCollectorAutoLinkStyleSheet: Ref::none(),
            VirtualCursorMode: enums::VirtualCursorMode::Default,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StarterPack {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StarterPack, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StarterPlayer {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AllowCustomAnimations: bool,
    pub AutoJumpEnabled: bool,
    pub AvatarJointUpgradeSerializedRollout: enums::RolloutState,
    pub CameraMaxZoomDistance: f32,
    pub CameraMinZoomDistance: f32,
    pub CameraMode: enums::CameraMode,
    pub CharacterBreakJointsOnDeath: bool,
    pub CharacterJumpHeight: f32,
    pub CharacterJumpPower: f32,
    pub CharacterMaxSlopeAngle: f32,
    pub CharacterUseJumpPower: bool,
    pub CharacterWalkSpeed: f32,
    pub ClassicDeath: bool,
    pub CreateDefaultPlayerModule: bool,
    pub DevCameraOcclusionMode: enums::DevCameraOcclusionMode,
    pub DevComputerCameraMovementMode: enums::DevComputerCameraMovementMode,
    pub DevComputerMovementMode: enums::DevComputerMovementMode,
    pub DevTouchCameraMovementMode: enums::DevTouchCameraMovementMode,
    pub DevTouchMovementMode: enums::DevTouchMovementMode,
    pub EnableDynamicHeads: enums::LoadDynamicHeads,
    pub EnableMouseLockOption: bool,
    pub GameSettingsAssetIdFace: i64,
    pub GameSettingsAssetIdHead: i64,
    pub GameSettingsAssetIdLeftArm: i64,
    pub GameSettingsAssetIdLeftLeg: i64,
    pub GameSettingsAssetIdPants: i64,
    pub GameSettingsAssetIdRightArm: i64,
    pub GameSettingsAssetIdRightLeg: i64,
    pub GameSettingsAssetIdShirt: i64,
    pub GameSettingsAssetIdTeeShirt: i64,
    pub GameSettingsAssetIdTorso: i64,
    pub GameSettingsAvatar: enums::GameAvatarType,
    pub GameSettingsR15Collision: enums::R15CollisionType,
    pub GameSettingsScaleRangeBodyType: NumberRange,
    pub GameSettingsScaleRangeHead: NumberRange,
    pub GameSettingsScaleRangeHeight: NumberRange,
    pub GameSettingsScaleRangeProportion: NumberRange,
    pub GameSettingsScaleRangeWidth: NumberRange,
    pub HealthDisplayDistance: f32,
    pub LoadCharacterAppearance: bool,
    pub LoadCharacterLayeredClothing: enums::LoadCharacterLayeredClothing,
    pub LuaCharacterController: enums::CharacterControlMode,
    pub NameDisplayDistance: f32,
    pub UserEmotesEnabled: bool,
}
impl_inherits!(StarterPlayer, Instance);
impl Default for StarterPlayer {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AllowCustomAnimations: true,
            AutoJumpEnabled: true,
            AvatarJointUpgradeSerializedRollout: enums::RolloutState::Default,
            CameraMaxZoomDistance: 400f32,
            CameraMinZoomDistance: 0.5f32,
            CameraMode: enums::CameraMode::Classic,
            CharacterBreakJointsOnDeath: true,
            CharacterJumpHeight: 7.2f32,
            CharacterJumpPower: 50f32,
            CharacterMaxSlopeAngle: 89f32,
            CharacterUseJumpPower: true,
            CharacterWalkSpeed: 16f32,
            ClassicDeath: true,
            CreateDefaultPlayerModule: true,
            DevCameraOcclusionMode: enums::DevCameraOcclusionMode::Zoom,
            DevComputerCameraMovementMode: enums::DevComputerCameraMovementMode::UserChoice,
            DevComputerMovementMode: enums::DevComputerMovementMode::UserChoice,
            DevTouchCameraMovementMode: enums::DevTouchCameraMovementMode::UserChoice,
            DevTouchMovementMode: enums::DevTouchMovementMode::UserChoice,
            EnableDynamicHeads: enums::LoadDynamicHeads::Default,
            EnableMouseLockOption: true,
            GameSettingsAssetIdFace: 0i64,
            GameSettingsAssetIdHead: 0i64,
            GameSettingsAssetIdLeftArm: 0i64,
            GameSettingsAssetIdLeftLeg: 0i64,
            GameSettingsAssetIdPants: 0i64,
            GameSettingsAssetIdRightArm: 0i64,
            GameSettingsAssetIdRightLeg: 0i64,
            GameSettingsAssetIdShirt: 0i64,
            GameSettingsAssetIdTeeShirt: 0i64,
            GameSettingsAssetIdTorso: 0i64,
            GameSettingsAvatar: enums::GameAvatarType::R15,
            GameSettingsR15Collision: enums::R15CollisionType::OuterBox,
            GameSettingsScaleRangeBodyType: NumberRange::new(0f32, 1f32),
            GameSettingsScaleRangeHead: NumberRange::new(0.95f32, 1f32),
            GameSettingsScaleRangeHeight: NumberRange::new(0.9f32, 1.05f32),
            GameSettingsScaleRangeProportion: NumberRange::new(0f32, 1f32),
            GameSettingsScaleRangeWidth: NumberRange::new(0.7f32, 1f32),
            HealthDisplayDistance: 100f32,
            LoadCharacterAppearance: true,
            LoadCharacterLayeredClothing: enums::LoadCharacterLayeredClothing::Default,
            LuaCharacterController: enums::CharacterControlMode::Default,
            NameDisplayDistance: 100f32,
            UserEmotesEnabled: true,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StarterPlayerScripts {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StarterPlayerScripts, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StartupMessageService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StartupMessageService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Stats {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(Stats, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StatsItem {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StatsItem, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Status {
    #[doc(hidden)]
    pub superclass: Model,
}
impl_inherits!(Status, Model);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StopWatchReporter {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StopWatchReporter, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StreamingService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StreamingService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StringValue {
    #[doc(hidden)]
    pub superclass: ValueBase,
    pub Value: String,
}
impl_inherits!(StringValue, ValueBase);
impl Default for StringValue {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        Self {
            superclass,
            Value: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Studio {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ActionOnAutoResumeSync: enums::ActionOnAutoResumeSync,
    pub ActionOnStopSync: enums::ActionOnStopSync,
    pub ActiveColor: Color3,
    pub ActiveHoverOverColor: Color3,
    pub AlwaysSaveScriptChanges: bool,
    pub AnimateHoverOver: bool,
    pub AutoCleanEmptyLine: bool,
    pub AutoClosingBrackets: bool,
    pub AutoClosingQuotes: bool,
    pub AutoDeleteClosingBracketsAndQuotes: bool,
    pub AutoIndentRule: enums::AutoIndentRule,
    pub AutoRecoveryEnabled: bool,
    pub AutoRecoveryIntervalMinutes: i32,
    pub AutoResumeSyncOnPlaceOpen: bool,
    pub AutoUpdateEnabled: bool,
    pub AutocompleteAcceptanceBehavior: enums::CompletionAcceptanceBehavior,
    pub AutomaticallyTriggerAiCodeCompletion: bool,
    pub BasicObjectsDisplayMode: enums::ListDisplayMode,
    pub CameraAdaptiveSpeed: bool,
    pub CameraMouseWheelSpeed: f32,
    pub CameraOrbitSensitivity: f32,
    pub CameraPanSensitivity: f32,
    pub CameraPanSpeed: f32,
    pub CameraShiftFactor: f32,
    pub CameraShiftSpeed: f32,
    pub CameraSpeed: f32,
    pub CameraSpeedAdjustBinding: enums::CameraSpeedAdjustBinding,
    pub CameraSpeedLockDefault: bool,
    pub CameraTweenFocus: bool,
    pub CameraZoomSpeed: f32,
    pub CameraZoomToMousePosition: bool,
    pub ClearOutputOnStart: bool,
    pub CommandBarLocalState: bool,
    pub DefaultScriptSyncFileType: enums::DefaultScriptSyncFileType,
    pub DeprecatedObjectsShown: bool,
    pub DisplayLanguage: String,
    pub DraggerActiveColor: Color3,
    pub DraggerMajorGridIncrement: i32,
    pub DraggerMaxSoftSnaps: i32,
    pub DraggerPassiveColor: Color3,
    pub DraggerShowHoverRuler: bool,
    pub DraggerShowMeasurement: bool,
    pub DraggerShowTargetSnap: bool,
    pub DraggerSoftSnapMarginFactor: f32,
    pub DraggerSummonMarginFactor: f32,
    pub DraggerTiltRotateDuration: f32,
    pub EnableAutocomplete: bool,
    pub EnableAutocompleteDocView: bool,
    pub EnableCodeAssist: bool,
    pub EnableCoreScriptDebugger: bool,
    pub EnableFindOnType: bool,
    pub EnableHttpSandboxing: bool,
    pub EnableIndentationRulers: bool,
    pub EnableInternalBetaFeatures: bool,
    pub EnableInternalFeatures: bool,
    pub EnableOnTypeAutocomplete: bool,
    pub EnableScriptAnalysis: bool,
    pub EnableScrollbarMarkers: bool,
    pub EnableSelectionTooltips: bool,
    pub EnableSignatureHelp: bool,
    pub EnableSignatureHelpDocView: bool,
    pub EnableStudioStreaming: bool,
    pub EnableTemporaryTabs: bool,
    pub EnableTemporaryTabsInExplorer: bool,
    pub EnableTypeHover: bool,
    pub FormatOnPaste: bool,
    pub FormatOnType: bool,
    pub HighlightCurrentLine: bool,
    pub HighlightOccurances: bool,
    pub HoverAnimateSpeed: enums::HoverAnimateSpeed,
    pub HoverBoxThickness: f32,
    pub HoverLineThickness: i32,
    pub HoverOverColor: Color3,
    pub IndentUsingSpaces: bool,
    pub LargeFileLineCountThreshold: i32,
    pub LargeFileThreshold: i32,
    pub LineThickness: f32,
    pub LoadAllBuiltinPluginsInRunModes: bool,
    pub LoadInternalPlugins: bool,
    pub LoadUserPluginsInRunModes: bool,
    pub LuaDebuggerEnabled: bool,
    pub MainVolume: f32,
    pub MaxFindReplaceAllResults: i32,
    pub MaximumOutputLines: i32,
    pub OnlyPlayAudioFromWindowInFocus: bool,
    pub OutputLayoutMode: enums::OutputLayoutMode,
    pub PermissionLevelShown: enums::PermissionLevelShown,
    pub PhysicalDraggersSelectScopeByDefault: bool,
    pub PivotSnapToGeometryColor: Color3,
    pub PluginDebuggingEnabled: bool,
    pub ReloadBuiltinPluginsOnChange: bool,
    pub ReloadLocalPluginsOnChange: bool,
    pub RespectStudioShortcutsWhenGameHasFocus: bool,
    pub Rulers: String,
    pub RuntimeUndoBehavior: enums::RuntimeUndoBehavior,
    pub ScriptEditorColorPreset: enums::StudioScriptEditorColorPresets,
    pub ScriptEditorShouldShowPluginMethods: bool,
    pub ScriptTimeoutLength: i32,
    pub ScrollPastLastLine: bool,
    pub SelectColor: Color3,
    pub SelectHoverColor: Color3,
    pub SelectionBoxThickness: f32,
    pub SelectionLineThickness: i32,
    pub SetPivotOfImportedParts: bool,
    pub ShowCoreGuiInExplorerWhilePlaying: bool,
    pub ShowCorePackagesInExplorer: bool,
    pub ShowDiagnosticsBar: bool,
    pub ShowFileSyncService: bool,
    pub ShowHiddenObjectsInExplorer: bool,
    pub ShowHoverOver: bool,
    pub ShowLightGuides: bool,
    pub ShowNavigationLabels: bool,
    pub ShowNavigationMesh: bool,
    pub ShowPathfindingLinks: bool,
    pub ShowPluginGuiServiceInExplorer: bool,
    pub ShowPlusButtonOnHoverInExplorer: bool,
    pub ShowSinglySelectedAttachmentParentFrame: bool,
    pub ShowWhitespace: bool,
    pub SkipClosingBracketsAndQuotes: bool,
    pub TabWidth: i32,
    pub TextWrapping: bool,
    pub UseBoundingBoxMoveHandles: bool,
}
impl_inherits!(Studio, Instance);
impl Default for Studio {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ActionOnAutoResumeSync: enums::ActionOnAutoResumeSync::DontResume,
            ActionOnStopSync: enums::ActionOnStopSync::AlwaysAsk,
            ActiveColor: Color3::new(0f32, 0f32, 0f32),
            ActiveHoverOverColor: Color3::new(0f32, 0f32, 0f32),
            AlwaysSaveScriptChanges: false,
            AnimateHoverOver: false,
            AutoCleanEmptyLine: false,
            AutoClosingBrackets: false,
            AutoClosingQuotes: false,
            AutoDeleteClosingBracketsAndQuotes: false,
            AutoIndentRule: enums::AutoIndentRule::Off,
            AutoRecoveryEnabled: false,
            AutoRecoveryIntervalMinutes: 0i32,
            AutoResumeSyncOnPlaceOpen: false,
            AutoUpdateEnabled: false,
            AutocompleteAcceptanceBehavior: enums::CompletionAcceptanceBehavior::Insert,
            AutomaticallyTriggerAiCodeCompletion: false,
            BasicObjectsDisplayMode: enums::ListDisplayMode::Horizontal,
            CameraAdaptiveSpeed: false,
            CameraMouseWheelSpeed: 0f32,
            CameraOrbitSensitivity: 0f32,
            CameraPanSensitivity: 0f32,
            CameraPanSpeed: 0f32,
            CameraShiftFactor: 0f32,
            CameraShiftSpeed: 0f32,
            CameraSpeed: 0f32,
            CameraSpeedAdjustBinding: enums::CameraSpeedAdjustBinding::None,
            CameraSpeedLockDefault: false,
            CameraTweenFocus: false,
            CameraZoomSpeed: 0f32,
            CameraZoomToMousePosition: false,
            ClearOutputOnStart: false,
            CommandBarLocalState: false,
            DefaultScriptSyncFileType: enums::DefaultScriptSyncFileType::Lua,
            DeprecatedObjectsShown: false,
            DisplayLanguage: "".to_owned(),
            DraggerActiveColor: Color3::new(0f32, 0f32, 0f32),
            DraggerMajorGridIncrement: 0i32,
            DraggerMaxSoftSnaps: 0i32,
            DraggerPassiveColor: Color3::new(0f32, 0f32, 0f32),
            DraggerShowHoverRuler: false,
            DraggerShowMeasurement: false,
            DraggerShowTargetSnap: false,
            DraggerSoftSnapMarginFactor: 0f32,
            DraggerSummonMarginFactor: 0f32,
            DraggerTiltRotateDuration: 0f32,
            EnableAutocomplete: false,
            EnableAutocompleteDocView: false,
            EnableCodeAssist: false,
            EnableCoreScriptDebugger: false,
            EnableFindOnType: false,
            EnableHttpSandboxing: false,
            EnableIndentationRulers: false,
            EnableInternalBetaFeatures: false,
            EnableInternalFeatures: false,
            EnableOnTypeAutocomplete: false,
            EnableScriptAnalysis: false,
            EnableScrollbarMarkers: false,
            EnableSelectionTooltips: false,
            EnableSignatureHelp: false,
            EnableSignatureHelpDocView: false,
            EnableStudioStreaming: false,
            EnableTemporaryTabs: false,
            EnableTemporaryTabsInExplorer: false,
            EnableTypeHover: false,
            FormatOnPaste: false,
            FormatOnType: false,
            HighlightCurrentLine: false,
            HighlightOccurances: false,
            HoverAnimateSpeed: enums::HoverAnimateSpeed::VerySlow,
            HoverBoxThickness: 0f32,
            HoverLineThickness: 0i32,
            HoverOverColor: Color3::new(0f32, 0f32, 0f32),
            IndentUsingSpaces: false,
            LargeFileLineCountThreshold: 0i32,
            LargeFileThreshold: 0i32,
            LineThickness: 0f32,
            LoadAllBuiltinPluginsInRunModes: false,
            LoadInternalPlugins: false,
            LoadUserPluginsInRunModes: false,
            LuaDebuggerEnabled: false,
            MainVolume: 0f32,
            MaxFindReplaceAllResults: 0i32,
            MaximumOutputLines: 0i32,
            OnlyPlayAudioFromWindowInFocus: false,
            OutputLayoutMode: enums::OutputLayoutMode::Horizontal,
            PermissionLevelShown: enums::PermissionLevelShown::Game,
            PhysicalDraggersSelectScopeByDefault: false,
            PivotSnapToGeometryColor: Color3::new(0f32, 0f32, 0f32),
            PluginDebuggingEnabled: false,
            ReloadBuiltinPluginsOnChange: false,
            ReloadLocalPluginsOnChange: false,
            RespectStudioShortcutsWhenGameHasFocus: false,
            Rulers: "".to_owned(),
            RuntimeUndoBehavior: enums::RuntimeUndoBehavior::Aggregate,
            ScriptEditorColorPreset: enums::StudioScriptEditorColorPresets::RobloxDefault,
            ScriptEditorShouldShowPluginMethods: false,
            ScriptTimeoutLength: 0i32,
            ScrollPastLastLine: false,
            SelectColor: Color3::new(0f32, 0f32, 0f32),
            SelectHoverColor: Color3::new(0f32, 0f32, 0f32),
            SelectionBoxThickness: 0f32,
            SelectionLineThickness: 0i32,
            SetPivotOfImportedParts: false,
            ShowCoreGuiInExplorerWhilePlaying: false,
            ShowCorePackagesInExplorer: false,
            ShowDiagnosticsBar: false,
            ShowFileSyncService: false,
            ShowHiddenObjectsInExplorer: false,
            ShowHoverOver: false,
            ShowLightGuides: false,
            ShowNavigationLabels: false,
            ShowNavigationMesh: false,
            ShowPathfindingLinks: false,
            ShowPluginGuiServiceInExplorer: false,
            ShowPlusButtonOnHoverInExplorer: false,
            ShowSinglySelectedAttachmentParentFrame: false,
            ShowWhitespace: false,
            SkipClosingBracketsAndQuotes: false,
            TabWidth: 0i32,
            TextWrapping: false,
            UseBoundingBoxMoveHandles: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioAssetService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StudioAssetService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioAttachment {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AutoHideParent: bool,
    pub IsArrowVisible: bool,
    pub Offset: Vector2,
    pub SourceAnchorPoint: Vector2,
    pub TargetAnchorPoint: Vector2,
}
impl_inherits!(StudioAttachment, Instance);
impl Default for StudioAttachment {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AutoHideParent: false,
            IsArrowVisible: false,
            Offset: Vector2::new(0f32, 0f32),
            SourceAnchorPoint: Vector2::new(0f32, 0f32),
            TargetAnchorPoint: Vector2::new(0f32, 0f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioCallout {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StudioCallout, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioCameraService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub LockCameraSpeed: bool,
    pub LoggingEnabled: bool,
}
impl_inherits!(StudioCameraService, Instance);
impl Default for StudioCameraService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            LockCameraSpeed: false,
            LoggingEnabled: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioData {
    #[doc(hidden)]
    pub superclass: Instance,
    pub EnableScriptCollabByDefaultOnLoad: bool,
}
impl_inherits!(StudioData, Instance);
impl Default for StudioData {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            EnableScriptCollabByDefaultOnLoad: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioDeviceEmulatorService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StudioDeviceEmulatorService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioObjectBase {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StudioObjectBase, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioPublishService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub PublishLocked: bool,
}
impl_inherits!(StudioPublishService, Instance);
impl Default for StudioPublishService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            PublishLocked: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioScriptDebugEventListener {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StudioScriptDebugEventListener, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioSdkService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StudioSdkService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Secrets: String,
}
impl_inherits!(StudioService, Instance);
impl Default for StudioService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Secrets: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioTestService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StudioTestService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioTheme {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StudioTheme, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioUserService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StudioUserService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioWidget {
    #[doc(hidden)]
    pub superclass: StudioObjectBase,
}
impl_inherits!(StudioWidget, StudioObjectBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioWidgetsService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StudioWidgetsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StyleBase {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StyleBase, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StyleDerive {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Priority: i32,
    pub StyleSheet: Ref,
}
impl_inherits!(StyleDerive, Instance);
impl Default for StyleDerive {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Priority: 0i32,
            StyleSheet: Ref::none(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StyleLink {
    #[doc(hidden)]
    pub superclass: Instance,
    pub StyleSheet: Ref,
}
impl_inherits!(StyleLink, Instance);
impl Default for StyleLink {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            StyleSheet: Ref::none(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StyleQuery {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AspectRatioRange: NumberRange,
    pub ConditionsSerialize: BinaryString,
    pub MaxSize: Vector2,
    pub MinSize: Vector2,
}
impl_inherits!(StyleQuery, Instance);
impl Default for StyleQuery {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AspectRatioRange: NumberRange::new(0f32, 0f32),
            ConditionsSerialize: b"".as_slice().into(),
            MaxSize: Vector2::new(0f32, 0f32),
            MinSize: Vector2::new(0f32, 0f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StyleRule {
    #[doc(hidden)]
    pub superclass: StyleBase,
    pub Priority: i32,
    pub Selector: String,
}
impl_inherits!(StyleRule, StyleBase);
impl Default for StyleRule {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = StyleBase { superclass };
        Self {
            superclass,
            Priority: 0i32,
            Selector: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StyleSheet {
    #[doc(hidden)]
    pub superclass: StyleBase,
}
impl_inherits!(StyleSheet, StyleBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StylingService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(StylingService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SunRaysEffect {
    #[doc(hidden)]
    pub superclass: PostEffect,
    pub Intensity: f32,
    pub Spread: f32,
}
impl_inherits!(SunRaysEffect, PostEffect);
impl Default for SunRaysEffect {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PostEffect {
            superclass,
            Enabled: true,
        };
        Self {
            superclass,
            Intensity: 0.25f32,
            Spread: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SurfaceAppearance {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AlphaMode: enums::AlphaMode,
    pub Color: Color3,
    pub ColorMapContent: Content,
    pub EmissiveMaskContent: Content,
    pub EmissiveStrength: f32,
    pub EmissiveTint: Color3,
    pub MetalnessMapContent: Content,
    pub NormalMapContent: Content,
    pub RoughnessMapContent: Content,
    pub TexturePack: ContentId,
}
impl_inherits!(SurfaceAppearance, Instance);
impl Default for SurfaceAppearance {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AlphaMode: enums::AlphaMode::Overlay,
            Color: Color3::new(1f32, 1f32, 1f32),
            ColorMapContent: Content::none(),
            EmissiveMaskContent: Content::none(),
            EmissiveStrength: 1f32,
            EmissiveTint: Color3::new(1f32, 1f32, 1f32),
            MetalnessMapContent: Content::none(),
            NormalMapContent: Content::none(),
            RoughnessMapContent: Content::none(),
            TexturePack: "".into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SurfaceGui {
    #[doc(hidden)]
    pub superclass: SurfaceGuiBase,
    pub AlwaysOnTop: bool,
    pub Brightness: f32,
    pub CanvasSize: Vector2,
    pub ClipsDescendants: bool,
    pub LightInfluence: f32,
    pub MaxDistance: f32,
    pub PixelsPerStud: f32,
    pub SizingMode: enums::SurfaceGuiSizingMode,
    pub ToolPunchThroughDistance: f32,
    pub ZOffset: f32,
}
impl_inherits!(SurfaceGui, SurfaceGuiBase);
impl Default for SurfaceGui {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase2d {
            superclass,
            AutoLocalize: true,
            RootLocalizationTable: Ref::none(),
            SelectionBehaviorDown: enums::SelectionBehavior::Escape,
            SelectionBehaviorLeft: enums::SelectionBehavior::Escape,
            SelectionBehaviorRight: enums::SelectionBehavior::Escape,
            SelectionBehaviorUp: enums::SelectionBehavior::Escape,
            SelectionGroup: false,
        };
        let superclass = LayerCollector {
            superclass,
            Enabled: true,
            ResetOnSpawn: true,
            ZIndexBehavior: enums::ZIndexBehavior::Global,
        };
        let superclass = SurfaceGuiBase {
            superclass,
            Active: true,
            Adornee: Ref::none(),
            Face: enums::NormalId::Front,
        };
        Self {
            superclass,
            AlwaysOnTop: false,
            Brightness: 1f32,
            CanvasSize: Vector2::new(800f32, 600f32),
            ClipsDescendants: false,
            LightInfluence: 0f32,
            MaxDistance: 0f32,
            PixelsPerStud: 50f32,
            SizingMode: enums::SurfaceGuiSizingMode::FixedSize,
            ToolPunchThroughDistance: 0f32,
            ZOffset: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SurfaceGuiBase {
    #[doc(hidden)]
    pub superclass: LayerCollector,
    pub Active: bool,
    pub Adornee: Ref,
    pub Face: enums::NormalId,
}
impl_inherits!(SurfaceGuiBase, LayerCollector);
impl Default for SurfaceGuiBase {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase2d {
            superclass,
            AutoLocalize: false,
            RootLocalizationTable: Ref::none(),
            SelectionBehaviorDown: enums::SelectionBehavior::Escape,
            SelectionBehaviorLeft: enums::SelectionBehavior::Escape,
            SelectionBehaviorRight: enums::SelectionBehavior::Escape,
            SelectionBehaviorUp: enums::SelectionBehavior::Escape,
            SelectionGroup: false,
        };
        let superclass = LayerCollector {
            superclass,
            Enabled: false,
            ResetOnSpawn: false,
            ZIndexBehavior: enums::ZIndexBehavior::Global,
        };
        Self {
            superclass,
            Active: false,
            Adornee: Ref::none(),
            Face: enums::NormalId::Right,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SurfaceLight {
    #[doc(hidden)]
    pub superclass: Light,
    pub Angle: f32,
    pub Face: enums::NormalId,
    pub Range: f32,
}
impl_inherits!(SurfaceLight, Light);
impl Default for SurfaceLight {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Light {
            superclass,
            Brightness: 1f32,
            Color: Color3::new(1f32, 1f32, 1f32),
            Enabled: true,
            Shadows: false,
        };
        Self {
            superclass,
            Angle: 90f32,
            Face: enums::NormalId::Front,
            Range: 16f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SurfaceSelection {
    #[doc(hidden)]
    pub superclass: PartAdornment,
    pub TargetSurface: enums::NormalId,
}
impl_inherits!(SurfaceSelection, PartAdornment);
impl Default for SurfaceSelection {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase3d {
            superclass,
            Color3: Color3::new(0.050980393f32, 0.4117647f32, 0.6745098f32),
            Transparency: 0f32,
            Visible: true,
        };
        let superclass = PartAdornment {
            superclass,
            Adornee: Ref::none(),
        };
        Self {
            superclass,
            TargetSurface: enums::NormalId::Right,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SwimController {
    #[doc(hidden)]
    pub superclass: ControllerBase,
    pub AccelerationTime: f32,
    pub PitchMaxTorque: f32,
    pub PitchSpeedFactor: f32,
    pub RollMaxTorque: f32,
    pub RollSpeedFactor: f32,
}
impl_inherits!(SwimController, ControllerBase);
impl Default for SwimController {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ControllerBase {
            superclass,
            BalanceRigidityEnabled: false,
            MoveSpeedFactor: 1f32,
        };
        Self {
            superclass,
            AccelerationTime: 0f32,
            PitchMaxTorque: 10000f32,
            PitchSpeedFactor: 1f32,
            RollMaxTorque: 10000f32,
            RollSpeedFactor: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SyncScriptBuilder {
    #[doc(hidden)]
    pub superclass: ScriptBuilder,
    pub CompileTarget: enums::CompileTarget,
    pub CoverageInfo: bool,
    pub DebugInfo: bool,
    pub PackAsSource: bool,
    pub RawBytecode: bool,
}
impl_inherits!(SyncScriptBuilder, ScriptBuilder);
impl Default for SyncScriptBuilder {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ScriptBuilder { superclass };
        Self {
            superclass,
            CompileTarget: enums::CompileTarget::Client,
            CoverageInfo: false,
            DebugInfo: false,
            PackAsSource: false,
            RawBytecode: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SystemThemeService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(SystemThemeService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TaskScheduler {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ThreadPoolConfig: enums::ThreadPoolConfig,
}
impl_inherits!(TaskScheduler, Instance);
impl Default for TaskScheduler {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ThreadPoolConfig: enums::ThreadPoolConfig::Auto,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Team {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AutoAssignable: bool,
    pub TeamColor: BrickColor,
}
impl_inherits!(Team, Instance);
impl Default for Team {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AutoAssignable: true,
            TeamColor: BrickColor::from_number(1u16).unwrap(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TeamCreateData {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TeamCreateData, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TeamCreatePublishService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TeamCreatePublishService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TeamCreateService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TeamCreateService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Teams {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(Teams, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TelemetryService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TelemetryService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TeleportAsyncResult {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TeleportAsyncResult, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TeleportOptions {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ReservedServerAccessCode: String,
    pub ServerInstanceId: String,
    pub ShouldReserveServer: bool,
}
impl_inherits!(TeleportOptions, Instance);
impl Default for TeleportOptions {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ReservedServerAccessCode: "".to_owned(),
            ServerInstanceId: "".to_owned(),
            ShouldReserveServer: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TeleportService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TeleportService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TemporaryCageMeshProvider {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TemporaryCageMeshProvider, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TemporaryScriptService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TemporaryScriptService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Terrain {
    #[doc(hidden)]
    pub superclass: BasePart,
    pub AcquisitionMethod: enums::TerrainAcquisitionMethod,
    pub Decoration: bool,
    pub GrassLength: f32,
    pub MaterialColors: MaterialColors,
    pub PhysicsGrid: BinaryString,
    pub SmoothGrid: BinaryString,
    pub SmoothVoxelsUpgraded: bool,
    pub WaterColor: Color3,
    pub WaterReflectance: f32,
    pub WaterTransparency: f32,
    pub WaterWaveSize: f32,
    pub WaterWaveSpeed: f32,
}
impl_inherits!(Terrain, BasePart);
impl Default for Terrain {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PVInstance { superclass };
        let superclass = BasePart {
            superclass,
            Anchored: true,
            AudioCanCollide: true,
            BackParamA: -0.5f32,
            BackParamB: 0.5f32,
            BackSurface: enums::SurfaceType::Smooth,
            BackSurfaceInput: enums::InputType::NoInput,
            BottomParamA: -0.5f32,
            BottomParamB: 0.5f32,
            BottomSurface: enums::SurfaceType::Inlet,
            BottomSurfaceInput: enums::InputType::NoInput,
            CFrame: CFrame::identity(),
            CanCollide: true,
            CanQuery: true,
            CanTouch: true,
            CastShadow: true,
            CollisionGroup: "Default".to_owned(),
            CollisionGroupId: 0i32,
            CustomPhysicalProperties: PhysicalProperties::Default,
            EnableFluidForces: true,
            FrontParamA: -0.5f32,
            FrontParamB: 0.5f32,
            FrontSurface: enums::SurfaceType::Smooth,
            FrontSurfaceInput: enums::InputType::NoInput,
            LeftParamA: -0.5f32,
            LeftParamB: 0.5f32,
            LeftSurface: enums::SurfaceType::Smooth,
            LeftSurfaceInput: enums::InputType::NoInput,
            Locked: true,
            Massless: false,
            Material: enums::Material::Plastic,
            PivotOffset: CFrame::identity(),
            Reflectance: 0f32,
            RightParamA: -0.5f32,
            RightParamB: 0.5f32,
            RightSurface: enums::SurfaceType::Smooth,
            RightSurfaceInput: enums::InputType::NoInput,
            RootPriority: 0i32,
            RotVelocity: Vector3::new(0f32, 0f32, 0f32),
            TopParamA: -0.5f32,
            TopParamB: 0.5f32,
            TopSurface: enums::SurfaceType::Studs,
            TopSurfaceInput: enums::InputType::NoInput,
            Transparency: 0f32,
            Velocity: Vector3::new(0f32, 0f32, 0f32),
        };
        Self {
            superclass,
            AcquisitionMethod: enums::TerrainAcquisitionMethod::None,
            Decoration: false,
            GrassLength: 0.7f32,
            MaterialColors: MaterialColors::new(),
            PhysicsGrid: b"\x02\x03\0\0\0\0\0\0\0\0\0\0\0\0".as_slice().into(),
            SmoothGrid: b"\x01\x05".as_slice().into(),
            SmoothVoxelsUpgraded: false,
            WaterColor: Color3::new(0.05f32, 0.33f32, 0.36f32),
            WaterReflectance: 1f32,
            WaterTransparency: 0.3f32,
            WaterWaveSize: 0.15f32,
            WaterWaveSpeed: 10f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TerrainDetail {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ColorMapContent: Content,
    pub EmissiveMaskContent: Content,
    pub EmissiveStrength: f32,
    pub EmissiveTint: Color3,
    pub Face: enums::TerrainFace,
    pub MaterialPattern: enums::MaterialPattern,
    pub MetalnessMapContent: Content,
    pub NormalMapContent: Content,
    pub RoughnessMapContent: Content,
    pub StudsPerTile: f32,
    pub TexturePack: ContentId,
}
impl_inherits!(TerrainDetail, Instance);
impl Default for TerrainDetail {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ColorMapContent: Content::none(),
            EmissiveMaskContent: Content::none(),
            EmissiveStrength: 1f32,
            EmissiveTint: Color3::new(1f32, 1f32, 1f32),
            Face: enums::TerrainFace::Side,
            MaterialPattern: enums::MaterialPattern::Regular,
            MetalnessMapContent: Content::none(),
            NormalMapContent: Content::none(),
            RoughnessMapContent: Content::none(),
            StudsPerTile: 10f32,
            TexturePack: "".into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TerrainIterateOperation {
    #[doc(hidden)]
    pub superclass: Object,
}
impl_inherits!(TerrainIterateOperation, Object);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TerrainModifyOperation {
    #[doc(hidden)]
    pub superclass: Object,
}
impl_inherits!(TerrainModifyOperation, Object);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TerrainReadOperation {
    #[doc(hidden)]
    pub superclass: Object,
}
impl_inherits!(TerrainReadOperation, Object);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TerrainRegion {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ExtentsMax: Vector3int16,
    pub ExtentsMin: Vector3int16,
    pub SmoothGrid: BinaryString,
}
impl_inherits!(TerrainRegion, Instance);
impl Default for TerrainRegion {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ExtentsMax: Vector3int16::new(0i16, 0i16, 0i16),
            ExtentsMin: Vector3int16::new(0i16, 0i16, 0i16),
            SmoothGrid: b"\x01\x05".as_slice().into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TerrainWriteOperation {
    #[doc(hidden)]
    pub superclass: Object,
}
impl_inherits!(TerrainWriteOperation, Object);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TestService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AutoRuns: bool,
    pub Description: String,
    pub ExecuteWithStudioRun: bool,
    pub IsPhysicsEnvironmentalThrottled: bool,
    pub IsSleepAllowed: bool,
    pub NumberOfPlayers: i32,
    pub SimulateSecondsLag: f64,
    pub ThrottlePhysicsToRealtime: bool,
    pub Timeout: f64,
}
impl_inherits!(TestService, Instance);
impl Default for TestService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AutoRuns: true,
            Description: "".to_owned(),
            ExecuteWithStudioRun: false,
            IsPhysicsEnvironmentalThrottled: true,
            IsSleepAllowed: true,
            NumberOfPlayers: 0i32,
            SimulateSecondsLag: 0f64,
            ThrottlePhysicsToRealtime: true,
            Timeout: 10f64,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextBox {
    #[doc(hidden)]
    pub superclass: GuiObject,
    pub ClearTextOnFocus: bool,
    pub FontFace: Font,
    pub LineHeight: f32,
    pub LocalizationMatchIdentifier: String,
    pub LocalizationMatchedSourceText: String,
    pub MaxVisibleGraphemes: i32,
    pub MultiLine: bool,
    pub OpenTypeFeatures: String,
    pub PlaceholderColor3: Color3,
    pub PlaceholderText: String,
    pub RichText: bool,
    pub ShowNativeInput: bool,
    pub Text: String,
    pub TextColor3: Color3,
    pub TextDirection: enums::TextDirection,
    pub TextEditable: bool,
    pub TextScaled: bool,
    pub TextSize: f32,
    pub TextStrokeColor3: Color3,
    pub TextStrokeTransparency: f32,
    pub TextTransparency: f32,
    pub TextTruncate: enums::TextTruncate,
    pub TextWrapped: bool,
    pub TextXAlignment: enums::TextXAlignment,
    pub TextYAlignment: enums::TextYAlignment,
}
impl_inherits!(TextBox, GuiObject);
impl Default for TextBox {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase2d {
            superclass,
            AutoLocalize: true,
            RootLocalizationTable: Ref::none(),
            SelectionBehaviorDown: enums::SelectionBehavior::Escape,
            SelectionBehaviorLeft: enums::SelectionBehavior::Escape,
            SelectionBehaviorRight: enums::SelectionBehavior::Escape,
            SelectionBehaviorUp: enums::SelectionBehavior::Escape,
            SelectionGroup: false,
        };
        let superclass = GuiObject {
            superclass,
            Active: true,
            AnchorPoint: Vector2::new(0f32, 0f32),
            AutomaticSize: enums::AutomaticSize::None,
            BackgroundColor3: Color3::new(0.6392157f32, 0.63529414f32, 0.64705884f32),
            BackgroundTransparency: 0f32,
            BorderColor3: Color3::new(0.105882354f32, 0.16470589f32, 0.20784314f32),
            BorderMode: enums::BorderMode::Outline,
            BorderSizePixel: 1i32,
            ClipsDescendants: false,
            Draggable: false,
            Interactable: true,
            LayoutOrder: 0i32,
            NextSelectionDown: Ref::none(),
            NextSelectionLeft: Ref::none(),
            NextSelectionRight: Ref::none(),
            NextSelectionUp: Ref::none(),
            Position: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            Rotation: 0f32,
            Selectable: true,
            SelectionImageObject: Ref::none(),
            SelectionOrder: 0i32,
            Size: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            SizeConstraint: enums::SizeConstraint::RelativeXY,
            Visible: true,
            ZIndex: 1i32,
        };
        Self {
            superclass,
            ClearTextOnFocus: true,
            FontFace: Font {
                family: "rbxasset://fonts/families/LegacyArial.json".to_owned(),
                weight: FontWeight::Regular,
                style: FontStyle::Normal,
                cached_face_id: Some("rbxasset://fonts/Arimo-Regular.ttf".to_owned()),
            },
            LineHeight: 1f32,
            LocalizationMatchIdentifier: "".to_owned(),
            LocalizationMatchedSourceText: "".to_owned(),
            MaxVisibleGraphemes: -1i32,
            MultiLine: false,
            OpenTypeFeatures: "".to_owned(),
            PlaceholderColor3: Color3::new(0.7f32, 0.7f32, 0.7f32),
            PlaceholderText: "".to_owned(),
            RichText: false,
            ShowNativeInput: true,
            Text: "TextBox".to_owned(),
            TextColor3: Color3::new(0.10588236f32, 0.16470589f32, 0.20784315f32),
            TextDirection: enums::TextDirection::Auto,
            TextEditable: true,
            TextScaled: false,
            TextSize: 8f32,
            TextStrokeColor3: Color3::new(0f32, 0f32, 0f32),
            TextStrokeTransparency: 1f32,
            TextTransparency: 0f32,
            TextTruncate: enums::TextTruncate::None,
            TextWrapped: false,
            TextXAlignment: enums::TextXAlignment::Center,
            TextYAlignment: enums::TextYAlignment::Center,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextBoxService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TextBoxService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextButton {
    #[doc(hidden)]
    pub superclass: GuiButton,
    pub FontFace: Font,
    pub LineHeight: f32,
    pub LocalizationMatchIdentifier: String,
    pub LocalizationMatchedSourceText: String,
    pub MaxVisibleGraphemes: i32,
    pub OpenTypeFeatures: String,
    pub RichText: bool,
    pub Text: String,
    pub TextColor3: Color3,
    pub TextDirection: enums::TextDirection,
    pub TextScaled: bool,
    pub TextSize: f32,
    pub TextStrokeColor3: Color3,
    pub TextStrokeTransparency: f32,
    pub TextTransparency: f32,
    pub TextTruncate: enums::TextTruncate,
    pub TextWrapped: bool,
    pub TextXAlignment: enums::TextXAlignment,
    pub TextYAlignment: enums::TextYAlignment,
}
impl_inherits!(TextButton, GuiButton);
impl Default for TextButton {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase2d {
            superclass,
            AutoLocalize: true,
            RootLocalizationTable: Ref::none(),
            SelectionBehaviorDown: enums::SelectionBehavior::Escape,
            SelectionBehaviorLeft: enums::SelectionBehavior::Escape,
            SelectionBehaviorRight: enums::SelectionBehavior::Escape,
            SelectionBehaviorUp: enums::SelectionBehavior::Escape,
            SelectionGroup: false,
        };
        let superclass = GuiObject {
            superclass,
            Active: true,
            AnchorPoint: Vector2::new(0f32, 0f32),
            AutomaticSize: enums::AutomaticSize::None,
            BackgroundColor3: Color3::new(0.6392157f32, 0.63529414f32, 0.64705884f32),
            BackgroundTransparency: 0f32,
            BorderColor3: Color3::new(0.105882354f32, 0.16470589f32, 0.20784314f32),
            BorderMode: enums::BorderMode::Outline,
            BorderSizePixel: 1i32,
            ClipsDescendants: false,
            Draggable: false,
            Interactable: true,
            LayoutOrder: 0i32,
            NextSelectionDown: Ref::none(),
            NextSelectionLeft: Ref::none(),
            NextSelectionRight: Ref::none(),
            NextSelectionUp: Ref::none(),
            Position: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            Rotation: 0f32,
            Selectable: true,
            SelectionImageObject: Ref::none(),
            SelectionOrder: 0i32,
            Size: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            SizeConstraint: enums::SizeConstraint::RelativeXY,
            Visible: true,
            ZIndex: 1i32,
        };
        let superclass = GuiButton {
            superclass,
            AutoButtonColor: true,
            HoverHapticEffect: Ref::none(),
            Modal: false,
            PressHapticEffect: Ref::none(),
            Selected: false,
            Style: enums::ButtonStyle::Custom,
        };
        Self {
            superclass,
            FontFace: Font {
                family: "rbxasset://fonts/families/LegacyArial.json".to_owned(),
                weight: FontWeight::Regular,
                style: FontStyle::Normal,
                cached_face_id: Some("rbxasset://fonts/Arimo-Regular.ttf".to_owned()),
            },
            LineHeight: 1f32,
            LocalizationMatchIdentifier: "".to_owned(),
            LocalizationMatchedSourceText: "".to_owned(),
            MaxVisibleGraphemes: -1i32,
            OpenTypeFeatures: "".to_owned(),
            RichText: false,
            Text: "Button".to_owned(),
            TextColor3: Color3::new(0.10588236f32, 0.16470589f32, 0.20784315f32),
            TextDirection: enums::TextDirection::Auto,
            TextScaled: false,
            TextSize: 8f32,
            TextStrokeColor3: Color3::new(0f32, 0f32, 0f32),
            TextStrokeTransparency: 1f32,
            TextTransparency: 0f32,
            TextTruncate: enums::TextTruncate::None,
            TextWrapped: false,
            TextXAlignment: enums::TextXAlignment::Center,
            TextYAlignment: enums::TextYAlignment::Center,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextChannel {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TextChannel, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextChatCommand {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AutocompleteVisible: bool,
    pub Enabled: bool,
    pub PrimaryAlias: String,
    pub SecondaryAlias: String,
}
impl_inherits!(TextChatCommand, Instance);
impl Default for TextChatCommand {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AutocompleteVisible: true,
            Enabled: true,
            PrimaryAlias: "".to_owned(),
            SecondaryAlias: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextChatConfigurations {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TextChatConfigurations, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextChatMessage {
    #[doc(hidden)]
    pub superclass: Instance,
    pub BubbleChatMessageProperties: Ref,
    pub ChatWindowMessageProperties: Ref,
    pub TextChannel: Ref,
    pub TextSource: Ref,
}
impl_inherits!(TextChatMessage, Instance);
impl Default for TextChatMessage {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            BubbleChatMessageProperties: Ref::none(),
            ChatWindowMessageProperties: Ref::none(),
            TextChannel: Ref::none(),
            TextSource: Ref::none(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextChatMessageProperties {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TextChatMessageProperties, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextChatService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ChatTranslationFtuxShown: bool,
    pub ChatTranslationToggleEnabled: bool,
    pub ChatVersion: enums::ChatVersion,
    pub CreateDefaultCommands: bool,
    pub CreateDefaultTextChannels: bool,
    pub HasSeenDeprecationDialog: bool,
    pub IsLegacyChatDisabled: bool,
}
impl_inherits!(TextChatService, Instance);
impl Default for TextChatService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ChatTranslationFtuxShown: true,
            ChatTranslationToggleEnabled: false,
            ChatVersion: enums::ChatVersion::LegacyChatService,
            CreateDefaultCommands: true,
            CreateDefaultTextChannels: true,
            HasSeenDeprecationDialog: false,
            IsLegacyChatDisabled: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextFilterResult {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TextFilterResult, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextFilterTranslatedResult {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TextFilterTranslatedResult, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextGenerator {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Seed: i32,
    pub SystemPrompt: String,
    pub Temperature: f32,
    pub TopP: f32,
}
impl_inherits!(TextGenerator, Instance);
impl Default for TextGenerator {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Seed: 0i32,
            SystemPrompt: "".to_owned(),
            Temperature: 0.7f32,
            TopP: 0.9f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextLabel {
    #[doc(hidden)]
    pub superclass: GuiLabel,
    pub FontFace: Font,
    pub LineHeight: f32,
    pub LocalizationMatchIdentifier: String,
    pub LocalizationMatchedSourceText: String,
    pub MaxVisibleGraphemes: i32,
    pub OpenTypeFeatures: String,
    pub RichText: bool,
    pub Text: String,
    pub TextColor3: Color3,
    pub TextDirection: enums::TextDirection,
    pub TextScaled: bool,
    pub TextSize: f32,
    pub TextStrokeColor3: Color3,
    pub TextStrokeTransparency: f32,
    pub TextTransparency: f32,
    pub TextTruncate: enums::TextTruncate,
    pub TextWrapped: bool,
    pub TextXAlignment: enums::TextXAlignment,
    pub TextYAlignment: enums::TextYAlignment,
}
impl_inherits!(TextLabel, GuiLabel);
impl Default for TextLabel {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase2d {
            superclass,
            AutoLocalize: true,
            RootLocalizationTable: Ref::none(),
            SelectionBehaviorDown: enums::SelectionBehavior::Escape,
            SelectionBehaviorLeft: enums::SelectionBehavior::Escape,
            SelectionBehaviorRight: enums::SelectionBehavior::Escape,
            SelectionBehaviorUp: enums::SelectionBehavior::Escape,
            SelectionGroup: false,
        };
        let superclass = GuiObject {
            superclass,
            Active: false,
            AnchorPoint: Vector2::new(0f32, 0f32),
            AutomaticSize: enums::AutomaticSize::None,
            BackgroundColor3: Color3::new(0.6392157f32, 0.63529414f32, 0.64705884f32),
            BackgroundTransparency: 0f32,
            BorderColor3: Color3::new(0.105882354f32, 0.16470589f32, 0.20784314f32),
            BorderMode: enums::BorderMode::Outline,
            BorderSizePixel: 1i32,
            ClipsDescendants: false,
            Draggable: false,
            Interactable: true,
            LayoutOrder: 0i32,
            NextSelectionDown: Ref::none(),
            NextSelectionLeft: Ref::none(),
            NextSelectionRight: Ref::none(),
            NextSelectionUp: Ref::none(),
            Position: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            Rotation: 0f32,
            Selectable: false,
            SelectionImageObject: Ref::none(),
            SelectionOrder: 0i32,
            Size: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            SizeConstraint: enums::SizeConstraint::RelativeXY,
            Visible: true,
            ZIndex: 1i32,
        };
        let superclass = GuiLabel { superclass };
        Self {
            superclass,
            FontFace: Font {
                family: "rbxasset://fonts/families/LegacyArial.json".to_owned(),
                weight: FontWeight::Regular,
                style: FontStyle::Normal,
                cached_face_id: Some("rbxasset://fonts/Arimo-Regular.ttf".to_owned()),
            },
            LineHeight: 1f32,
            LocalizationMatchIdentifier: "".to_owned(),
            LocalizationMatchedSourceText: "".to_owned(),
            MaxVisibleGraphemes: -1i32,
            OpenTypeFeatures: "".to_owned(),
            RichText: false,
            Text: "Label".to_owned(),
            TextColor3: Color3::new(0.10588236f32, 0.16470589f32, 0.20784315f32),
            TextDirection: enums::TextDirection::Auto,
            TextScaled: false,
            TextSize: 8f32,
            TextStrokeColor3: Color3::new(0f32, 0f32, 0f32),
            TextStrokeTransparency: 1f32,
            TextTransparency: 0f32,
            TextTruncate: enums::TextTruncate::None,
            TextWrapped: false,
            TextXAlignment: enums::TextXAlignment::Center,
            TextYAlignment: enums::TextYAlignment::Center,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TextService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextSource {
    #[doc(hidden)]
    pub superclass: Instance,
    pub CanSend: bool,
}
impl_inherits!(TextSource, Instance);
impl Default for TextSource {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            CanSend: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Texture {
    #[doc(hidden)]
    pub superclass: Decal,
    pub OffsetStudsU: f32,
    pub OffsetStudsV: f32,
    pub StudsPerTileU: f32,
    pub StudsPerTileV: f32,
}
impl_inherits!(Texture, Decal);
impl Default for Texture {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = FaceInstance {
            superclass,
            Face: enums::NormalId::Front,
        };
        let superclass = Decal {
            superclass,
            Color3: Color3::new(1f32, 1f32, 1f32),
            MetalnessMapContent: Content::none(),
            NormalMapContent: Content::none(),
            RoughnessMapContent: Content::none(),
            TextureContent: Content::none(),
            TexturePack: "".into(),
            TexturePackMetadata: "".to_owned(),
            Transparency: 0f32,
            UvOffset: Vector2::new(0f32, 0f32),
            UvScale: Vector2::new(1f32, 1f32),
            ZIndex: 1i32,
        };
        Self {
            superclass,
            OffsetStudsU: 0f32,
            OffsetStudsV: 0f32,
            StudsPerTileU: 2f32,
            StudsPerTileV: 2f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextureGenerationPartGroup {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TextureGenerationPartGroup, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextureGenerationService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TextureGenerationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextureGenerationUnwrappingRequest {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TextureGenerationUnwrappingRequest, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ThirdPartyUserService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ThirdPartyUserService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ThreadState {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ThreadState, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TimerService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TimerService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ToastNotificationService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ToastNotificationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Tool {
    #[doc(hidden)]
    pub superclass: BackpackItem,
    pub CanBeDropped: bool,
    pub Enabled: bool,
    pub Grip: CFrame,
    pub ManualActivationOnly: bool,
    pub RequiresHandle: bool,
    pub ToolTip: String,
}
impl_inherits!(Tool, BackpackItem);
impl Default for Tool {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PVInstance { superclass };
        let superclass = Model {
            superclass,
            LevelOfDetail: enums::ModelLevelOfDetail::Automatic,
            ModelMeshCFrame: CFrame::identity(),
            ModelMeshData: SharedString::new(b"".to_vec()),
            ModelMeshSize: Vector3::new(0f32, 0f32, 0f32),
            ModelStreamingMode: enums::ModelStreamingMode::Default,
            NeedsPivotMigration: false,
            PrimaryPart: Ref::none(),
            SlimHash: SharedString::new(b"".to_vec()),
            WorldPivotData: None,
        };
        let superclass = BackpackItem {
            superclass,
            TextureContent: Content::none(),
        };
        Self {
            superclass,
            CanBeDropped: true,
            Enabled: true,
            Grip: CFrame::identity(),
            ManualActivationOnly: false,
            RequiresHandle: true,
            ToolTip: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Torque {
    #[doc(hidden)]
    pub superclass: Constraint,
    pub RelativeTo: enums::ActuatorRelativeTo,
    pub Torque: Vector3,
}
impl_inherits!(Torque, Constraint);
impl Default for Torque {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Constraint {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            Color: BrickColor::from_number(23u16).unwrap(),
            Enabled: true,
            Visible: false,
        };
        Self {
            superclass,
            RelativeTo: enums::ActuatorRelativeTo::Attachment0,
            Torque: Vector3::new(0f32, 0f32, 0f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TorsionSpringConstraint {
    #[doc(hidden)]
    pub superclass: Constraint,
    pub Coils: f32,
    pub Damping: f32,
    pub LimitEnabled: bool,
    pub LimitsEnabled: bool,
    pub MaxAngle: f32,
    pub MaxTorque: f32,
    pub Radius: f32,
    pub Restitution: f32,
    pub Stiffness: f32,
}
impl_inherits!(TorsionSpringConstraint, Constraint);
impl Default for TorsionSpringConstraint {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Constraint {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            Color: BrickColor::from_number(200u16).unwrap(),
            Enabled: true,
            Visible: false,
        };
        Self {
            superclass,
            Coils: 8f32,
            Damping: 0.01f32,
            LimitEnabled: false,
            LimitsEnabled: false,
            MaxAngle: 45f32,
            MaxTorque: f32::INFINITY,
            Radius: 0.4f32,
            Restitution: 0f32,
            Stiffness: 100f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TotalCountTimeIntervalItem {
    #[doc(hidden)]
    pub superclass: StatsItem,
}
impl_inherits!(TotalCountTimeIntervalItem, StatsItem);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TouchInputService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TouchInputService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TouchTransmitter {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TouchTransmitter, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TracerService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TracerService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TrackerLodController {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AudioMode: enums::TrackerLodFlagMode,
    pub VideoExtrapolationMode: enums::TrackerExtrapolationFlagMode,
    pub VideoLodMode: enums::TrackerLodValueMode,
    pub VideoMode: enums::TrackerLodFlagMode,
}
impl_inherits!(TrackerLodController, Instance);
impl Default for TrackerLodController {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AudioMode: enums::TrackerLodFlagMode::ForceFalse,
            VideoExtrapolationMode: enums::TrackerExtrapolationFlagMode::ForceDisabled,
            VideoLodMode: enums::TrackerLodValueMode::Force0,
            VideoMode: enums::TrackerLodFlagMode::ForceFalse,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TrackerStreamAnimation {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TrackerStreamAnimation, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Trail {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Attachment0: Ref,
    pub Attachment1: Ref,
    pub Brightness: f32,
    pub Color: ColorSequence,
    pub Enabled: bool,
    pub FaceCamera: bool,
    pub Lifetime: f32,
    pub LightEmission: f32,
    pub LightInfluence: f32,
    pub MaxLength: f32,
    pub MinLength: f32,
    pub Texture: ContentId,
    pub TextureLength: f32,
    pub TextureMode: enums::TextureMode,
    pub Transparency: NumberSequence,
    pub WidthScale: NumberSequence,
}
impl_inherits!(Trail, Instance);
impl Default for Trail {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            Brightness: 1f32,
            Color: ColorSequence {
                keypoints: vec![
                    ColorSequenceKeypoint::new(0f32, Color3::new(1f32, 1f32, 1f32)),
                    ColorSequenceKeypoint::new(1f32, Color3::new(1f32, 1f32, 1f32)),
                ],
            },
            Enabled: true,
            FaceCamera: false,
            Lifetime: 2f32,
            LightEmission: 0f32,
            LightInfluence: 0f32,
            MaxLength: 0f32,
            MinLength: 0.1f32,
            Texture: "".into(),
            TextureLength: 1f32,
            TextureMode: enums::TextureMode::Stretch,
            Transparency: NumberSequence {
                keypoints: vec![
                    NumberSequenceKeypoint::new(0f32, 0.5f32, 0f32),
                    NumberSequenceKeypoint::new(1f32, 0.5f32, 0f32),
                ],
            },
            WidthScale: NumberSequence {
                keypoints: vec![
                    NumberSequenceKeypoint::new(0f32, 1f32, 0f32),
                    NumberSequenceKeypoint::new(1f32, 1f32, 0f32),
                ],
            },
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Translator {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(Translator, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TremoloSoundEffect {
    #[doc(hidden)]
    pub superclass: SoundEffect,
    pub Depth: f32,
    pub Duty: f32,
    pub Frequency: f32,
}
impl_inherits!(TremoloSoundEffect, SoundEffect);
impl Default for TremoloSoundEffect {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundEffect {
            superclass,
            Enabled: true,
            Priority: 0i32,
        };
        Self {
            superclass,
            Depth: 1f32,
            Duty: 0.5f32,
            Frequency: 5f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TriangleMeshPart {
    #[doc(hidden)]
    pub superclass: BasePart,
    pub AeroMeshData: SharedString,
    pub FluidFidelityInternal: enums::FluidFidelity,
    pub InertiaMigrated: bool,
    pub PhysicalConfigData: SharedString,
    pub UnscaledCofm: Vector3,
    pub UnscaledVolInertiaDiags: Vector3,
    pub UnscaledVolInertiaOffDiags: Vector3,
    pub UnscaledVolume: f32,
}
impl_inherits!(TriangleMeshPart, BasePart);
impl Default for TriangleMeshPart {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PVInstance { superclass };
        let superclass = BasePart {
            superclass,
            Anchored: false,
            AudioCanCollide: false,
            BackParamA: 0f32,
            BackParamB: 0f32,
            BackSurface: enums::SurfaceType::Smooth,
            BackSurfaceInput: enums::InputType::NoInput,
            BottomParamA: 0f32,
            BottomParamB: 0f32,
            BottomSurface: enums::SurfaceType::Smooth,
            BottomSurfaceInput: enums::InputType::NoInput,
            CFrame: CFrame::identity(),
            CanCollide: false,
            CanQuery: false,
            CanTouch: false,
            CastShadow: false,
            CollisionGroup: "".to_owned(),
            CollisionGroupId: 0i32,
            CustomPhysicalProperties: PhysicalProperties::Default,
            EnableFluidForces: false,
            FrontParamA: 0f32,
            FrontParamB: 0f32,
            FrontSurface: enums::SurfaceType::Smooth,
            FrontSurfaceInput: enums::InputType::NoInput,
            LeftParamA: 0f32,
            LeftParamB: 0f32,
            LeftSurface: enums::SurfaceType::Smooth,
            LeftSurfaceInput: enums::InputType::NoInput,
            Locked: false,
            Massless: false,
            Material: enums::Material::Plastic,
            PivotOffset: CFrame::identity(),
            Reflectance: 0f32,
            RightParamA: 0f32,
            RightParamB: 0f32,
            RightSurface: enums::SurfaceType::Smooth,
            RightSurfaceInput: enums::InputType::NoInput,
            RootPriority: 0i32,
            RotVelocity: Vector3::new(0f32, 0f32, 0f32),
            TopParamA: 0f32,
            TopParamB: 0f32,
            TopSurface: enums::SurfaceType::Smooth,
            TopSurfaceInput: enums::InputType::NoInput,
            Transparency: 0f32,
            Velocity: Vector3::new(0f32, 0f32, 0f32),
        };
        Self {
            superclass,
            AeroMeshData: SharedString::new(b"".to_vec()),
            FluidFidelityInternal: enums::FluidFidelity::Automatic,
            InertiaMigrated: false,
            PhysicalConfigData: SharedString::new(b"".to_vec()),
            UnscaledCofm: Vector3::new(0f32, 0f32, 0f32),
            UnscaledVolInertiaDiags: Vector3::new(0f32, 0f32, 0f32),
            UnscaledVolInertiaOffDiags: Vector3::new(0f32, 0f32, 0f32),
            UnscaledVolume: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TrussPart {
    #[doc(hidden)]
    pub superclass: BasePart,
    pub Style: enums::Style,
}
impl_inherits!(TrussPart, BasePart);
impl Default for TrussPart {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PVInstance { superclass };
        let superclass = BasePart {
            superclass,
            Anchored: false,
            AudioCanCollide: true,
            BackParamA: -0.5f32,
            BackParamB: 0.5f32,
            BackSurface: enums::SurfaceType::Universal,
            BackSurfaceInput: enums::InputType::NoInput,
            BottomParamA: -0.5f32,
            BottomParamB: 0.5f32,
            BottomSurface: enums::SurfaceType::Universal,
            BottomSurfaceInput: enums::InputType::NoInput,
            CFrame: CFrame::identity(),
            CanCollide: true,
            CanQuery: true,
            CanTouch: true,
            CastShadow: true,
            CollisionGroup: "Default".to_owned(),
            CollisionGroupId: 0i32,
            CustomPhysicalProperties: PhysicalProperties::Default,
            EnableFluidForces: true,
            FrontParamA: -0.5f32,
            FrontParamB: 0.5f32,
            FrontSurface: enums::SurfaceType::Universal,
            FrontSurfaceInput: enums::InputType::NoInput,
            LeftParamA: -0.5f32,
            LeftParamB: 0.5f32,
            LeftSurface: enums::SurfaceType::Universal,
            LeftSurfaceInput: enums::InputType::NoInput,
            Locked: false,
            Massless: false,
            Material: enums::Material::Plastic,
            PivotOffset: CFrame::identity(),
            Reflectance: 0f32,
            RightParamA: -0.5f32,
            RightParamB: 0.5f32,
            RightSurface: enums::SurfaceType::Universal,
            RightSurfaceInput: enums::InputType::NoInput,
            RootPriority: 0i32,
            RotVelocity: Vector3::new(0f32, 0f32, 0f32),
            TopParamA: -0.5f32,
            TopParamB: 0.5f32,
            TopSurface: enums::SurfaceType::Universal,
            TopSurfaceInput: enums::InputType::NoInput,
            Transparency: 0f32,
            Velocity: Vector3::new(0f32, 0f32, 0f32),
        };
        Self {
            superclass,
            Style: enums::Style::AlternatingSupports,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TutorialService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TutorialService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Tween {
    #[doc(hidden)]
    pub superclass: TweenBase,
}
impl_inherits!(Tween, TweenBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TweenBase {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TweenBase, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TweenService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(TweenService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UGCAvatarService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(UGCAvatarService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UGCValidationService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(UGCValidationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIAspectRatioConstraint {
    #[doc(hidden)]
    pub superclass: UIConstraint,
    pub AspectRatio: f32,
    pub AspectType: enums::AspectType,
    pub DominantAxis: enums::DominantAxis,
}
impl_inherits!(UIAspectRatioConstraint, UIConstraint);
impl Default for UIAspectRatioConstraint {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = UIBase { superclass };
        let superclass = UIComponent { superclass };
        let superclass = UIConstraint { superclass };
        Self {
            superclass,
            AspectRatio: 1f32,
            AspectType: enums::AspectType::FitWithinMaxSize,
            DominantAxis: enums::DominantAxis::Width,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UIBase {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(UIBase, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UIComponent {
    #[doc(hidden)]
    pub superclass: UIBase,
}
impl_inherits!(UIComponent, UIBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UIConstraint {
    #[doc(hidden)]
    pub superclass: UIComponent,
}
impl_inherits!(UIConstraint, UIComponent);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UICorner {
    #[doc(hidden)]
    pub superclass: UIComponent,
    pub CornerRadius: UDim,
}
impl_inherits!(UICorner, UIComponent);
impl Default for UICorner {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = UIBase { superclass };
        let superclass = UIComponent { superclass };
        Self {
            superclass,
            CornerRadius: UDim::new(0f32, 8i32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIDragDetector {
    #[doc(hidden)]
    pub superclass: UIComponent,
    pub ActivatedCursorIconContent: Content,
    pub BoundingBehavior: enums::UIDragDetectorBoundingBehavior,
    pub BoundingUi: Ref,
    pub CursorIconContent: Content,
    pub DragAxis: Vector2,
    pub DragRelativity: enums::UIDragDetectorDragRelativity,
    pub DragRotation: f32,
    pub DragSpace: enums::UIDragDetectorDragSpace,
    pub DragStyle: enums::UIDragDetectorDragStyle,
    pub DragUDim2: UDim2,
    pub Enabled: bool,
    pub MaxDragAngle: f32,
    pub MaxDragTranslation: UDim2,
    pub MinDragAngle: f32,
    pub MinDragTranslation: UDim2,
    pub ReferenceUiInstance: Ref,
    pub ResponseStyle: enums::UIDragDetectorResponseStyle,
    pub SelectionModeDragSpeed: UDim2,
    pub SelectionModeRotateSpeed: f32,
    pub UiDragSpeedAxisMapping: enums::UIDragSpeedAxisMapping,
}
impl_inherits!(UIDragDetector, UIComponent);
impl Default for UIDragDetector {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = UIBase { superclass };
        let superclass = UIComponent { superclass };
        Self {
            superclass,
            ActivatedCursorIconContent: Content::none(),
            BoundingBehavior: enums::UIDragDetectorBoundingBehavior::Automatic,
            BoundingUi: Ref::none(),
            CursorIconContent: Content::none(),
            DragAxis: Vector2::new(1f32, 0f32),
            DragRelativity: enums::UIDragDetectorDragRelativity::Absolute,
            DragRotation: 0f32,
            DragSpace: enums::UIDragDetectorDragSpace::Parent,
            DragStyle: enums::UIDragDetectorDragStyle::TranslatePlane,
            DragUDim2: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            Enabled: true,
            MaxDragAngle: 0f32,
            MaxDragTranslation: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            MinDragAngle: 0f32,
            MinDragTranslation: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            ReferenceUiInstance: Ref::none(),
            ResponseStyle: enums::UIDragDetectorResponseStyle::Offset,
            SelectionModeDragSpeed: UDim2::new(UDim::new(0f32, 300i32), UDim::new(0f32, 300i32)),
            SelectionModeRotateSpeed: 90f32,
            UiDragSpeedAxisMapping: enums::UIDragSpeedAxisMapping::XY,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UIDragDetectorService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(UIDragDetectorService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIFlexItem {
    #[doc(hidden)]
    pub superclass: UIComponent,
    pub FlexMode: enums::UIFlexMode,
    pub GrowRatio: f32,
    pub ItemLineAlignment: enums::ItemLineAlignment,
    pub ShrinkRatio: f32,
}
impl_inherits!(UIFlexItem, UIComponent);
impl Default for UIFlexItem {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = UIBase { superclass };
        let superclass = UIComponent { superclass };
        Self {
            superclass,
            FlexMode: enums::UIFlexMode::None,
            GrowRatio: 0f32,
            ItemLineAlignment: enums::ItemLineAlignment::Automatic,
            ShrinkRatio: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIGradient {
    #[doc(hidden)]
    pub superclass: UIComponent,
    pub Color: ColorSequence,
    pub Enabled: bool,
    pub Offset: Vector2,
    pub Rotation: f32,
    pub Transparency: NumberSequence,
}
impl_inherits!(UIGradient, UIComponent);
impl Default for UIGradient {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = UIBase { superclass };
        let superclass = UIComponent { superclass };
        Self {
            superclass,
            Color: ColorSequence {
                keypoints: vec![
                    ColorSequenceKeypoint::new(0f32, Color3::new(1f32, 1f32, 1f32)),
                    ColorSequenceKeypoint::new(1f32, Color3::new(1f32, 1f32, 1f32)),
                ],
            },
            Enabled: true,
            Offset: Vector2::new(0f32, 0f32),
            Rotation: 0f32,
            Transparency: NumberSequence {
                keypoints: vec![
                    NumberSequenceKeypoint::new(0f32, 0f32, 0f32),
                    NumberSequenceKeypoint::new(1f32, 0f32, 0f32),
                ],
            },
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIGridLayout {
    #[doc(hidden)]
    pub superclass: UIGridStyleLayout,
    pub CellPadding: UDim2,
    pub CellSize: UDim2,
    pub FillDirectionMaxCells: i32,
    pub StartCorner: enums::StartCorner,
}
impl_inherits!(UIGridLayout, UIGridStyleLayout);
impl Default for UIGridLayout {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = UIBase { superclass };
        let superclass = UIComponent { superclass };
        let superclass = UILayout { superclass };
        let superclass = UIGridStyleLayout {
            superclass,
            FillDirection: enums::FillDirection::Horizontal,
            HorizontalAlignment: enums::HorizontalAlignment::Left,
            SortOrder: enums::SortOrder::Name,
            VerticalAlignment: enums::VerticalAlignment::Top,
        };
        Self {
            superclass,
            CellPadding: UDim2::new(UDim::new(0f32, 5i32), UDim::new(0f32, 5i32)),
            CellSize: UDim2::new(UDim::new(0f32, 100i32), UDim::new(0f32, 100i32)),
            FillDirectionMaxCells: 0i32,
            StartCorner: enums::StartCorner::TopLeft,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIGridStyleLayout {
    #[doc(hidden)]
    pub superclass: UILayout,
    pub FillDirection: enums::FillDirection,
    pub HorizontalAlignment: enums::HorizontalAlignment,
    pub SortOrder: enums::SortOrder,
    pub VerticalAlignment: enums::VerticalAlignment,
}
impl_inherits!(UIGridStyleLayout, UILayout);
impl Default for UIGridStyleLayout {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = UIBase { superclass };
        let superclass = UIComponent { superclass };
        let superclass = UILayout { superclass };
        Self {
            superclass,
            FillDirection: enums::FillDirection::Horizontal,
            HorizontalAlignment: enums::HorizontalAlignment::Center,
            SortOrder: enums::SortOrder::Name,
            VerticalAlignment: enums::VerticalAlignment::Center,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UILayout {
    #[doc(hidden)]
    pub superclass: UIComponent,
}
impl_inherits!(UILayout, UIComponent);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIListLayout {
    #[doc(hidden)]
    pub superclass: UIGridStyleLayout,
    pub HorizontalFlex: enums::UIFlexAlignment,
    pub ItemLineAlignment: enums::ItemLineAlignment,
    pub Padding: UDim,
    pub VerticalFlex: enums::UIFlexAlignment,
    pub Wraps: bool,
}
impl_inherits!(UIListLayout, UIGridStyleLayout);
impl Default for UIListLayout {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = UIBase { superclass };
        let superclass = UIComponent { superclass };
        let superclass = UILayout { superclass };
        let superclass = UIGridStyleLayout {
            superclass,
            FillDirection: enums::FillDirection::Vertical,
            HorizontalAlignment: enums::HorizontalAlignment::Left,
            SortOrder: enums::SortOrder::Name,
            VerticalAlignment: enums::VerticalAlignment::Top,
        };
        Self {
            superclass,
            HorizontalFlex: enums::UIFlexAlignment::None,
            ItemLineAlignment: enums::ItemLineAlignment::Automatic,
            Padding: UDim::new(0f32, 0i32),
            VerticalFlex: enums::UIFlexAlignment::None,
            Wraps: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIPadding {
    #[doc(hidden)]
    pub superclass: UIComponent,
    pub PaddingBottom: UDim,
    pub PaddingLeft: UDim,
    pub PaddingRight: UDim,
    pub PaddingTop: UDim,
}
impl_inherits!(UIPadding, UIComponent);
impl Default for UIPadding {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = UIBase { superclass };
        let superclass = UIComponent { superclass };
        Self {
            superclass,
            PaddingBottom: UDim::new(0f32, 0i32),
            PaddingLeft: UDim::new(0f32, 0i32),
            PaddingRight: UDim::new(0f32, 0i32),
            PaddingTop: UDim::new(0f32, 0i32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIPageLayout {
    #[doc(hidden)]
    pub superclass: UIGridStyleLayout,
    pub Animated: bool,
    pub Circular: bool,
    pub EasingDirection: enums::EasingDirection,
    pub EasingStyle: enums::EasingStyle,
    pub GamepadInputEnabled: bool,
    pub Padding: UDim,
    pub ScrollWheelInputEnabled: bool,
    pub TouchInputEnabled: bool,
    pub TweenTime: f32,
}
impl_inherits!(UIPageLayout, UIGridStyleLayout);
impl Default for UIPageLayout {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = UIBase { superclass };
        let superclass = UIComponent { superclass };
        let superclass = UILayout { superclass };
        let superclass = UIGridStyleLayout {
            superclass,
            FillDirection: enums::FillDirection::Horizontal,
            HorizontalAlignment: enums::HorizontalAlignment::Left,
            SortOrder: enums::SortOrder::Name,
            VerticalAlignment: enums::VerticalAlignment::Top,
        };
        Self {
            superclass,
            Animated: true,
            Circular: false,
            EasingDirection: enums::EasingDirection::Out,
            EasingStyle: enums::EasingStyle::Back,
            GamepadInputEnabled: true,
            Padding: UDim::new(0f32, 0i32),
            ScrollWheelInputEnabled: true,
            TouchInputEnabled: true,
            TweenTime: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIScale {
    #[doc(hidden)]
    pub superclass: UIComponent,
    pub Scale: f32,
}
impl_inherits!(UIScale, UIComponent);
impl Default for UIScale {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = UIBase { superclass };
        let superclass = UIComponent { superclass };
        Self {
            superclass,
            Scale: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UISizeConstraint {
    #[doc(hidden)]
    pub superclass: UIConstraint,
    pub MaxSize: Vector2,
    pub MinSize: Vector2,
}
impl_inherits!(UISizeConstraint, UIConstraint);
impl Default for UISizeConstraint {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = UIBase { superclass };
        let superclass = UIComponent { superclass };
        let superclass = UIConstraint { superclass };
        Self {
            superclass,
            MaxSize: Vector2::new(f32::INFINITY, f32::INFINITY),
            MinSize: Vector2::new(0f32, 0f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIStroke {
    #[doc(hidden)]
    pub superclass: UIComponent,
    pub ApplyStrokeMode: enums::ApplyStrokeMode,
    pub BorderOffset: UDim,
    pub BorderStrokePosition: enums::BorderStrokePosition,
    pub Color: Color3,
    pub Enabled: bool,
    pub LineJoinMode: enums::LineJoinMode,
    pub StrokeSizingMode: enums::StrokeSizingMode,
    pub Thickness: f32,
    pub Transparency: f32,
    pub ZIndex: i32,
}
impl_inherits!(UIStroke, UIComponent);
impl Default for UIStroke {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = UIBase { superclass };
        let superclass = UIComponent { superclass };
        Self {
            superclass,
            ApplyStrokeMode: enums::ApplyStrokeMode::Contextual,
            BorderOffset: UDim::new(0f32, 0i32),
            BorderStrokePosition: enums::BorderStrokePosition::Outer,
            Color: Color3::new(0f32, 0f32, 0f32),
            Enabled: true,
            LineJoinMode: enums::LineJoinMode::Round,
            StrokeSizingMode: enums::StrokeSizingMode::FixedSize,
            Thickness: 1f32,
            Transparency: 0f32,
            ZIndex: 1i32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UITableLayout {
    #[doc(hidden)]
    pub superclass: UIGridStyleLayout,
    pub FillEmptySpaceColumns: bool,
    pub FillEmptySpaceRows: bool,
    pub MajorAxis: enums::TableMajorAxis,
    pub Padding: UDim2,
}
impl_inherits!(UITableLayout, UIGridStyleLayout);
impl Default for UITableLayout {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = UIBase { superclass };
        let superclass = UIComponent { superclass };
        let superclass = UILayout { superclass };
        let superclass = UIGridStyleLayout {
            superclass,
            FillDirection: enums::FillDirection::Vertical,
            HorizontalAlignment: enums::HorizontalAlignment::Left,
            SortOrder: enums::SortOrder::Name,
            VerticalAlignment: enums::VerticalAlignment::Top,
        };
        Self {
            superclass,
            FillEmptySpaceColumns: false,
            FillEmptySpaceRows: false,
            MajorAxis: enums::TableMajorAxis::RowMajor,
            Padding: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UITextSizeConstraint {
    #[doc(hidden)]
    pub superclass: UIConstraint,
    pub MaxTextSize: i32,
    pub MinTextSize: i32,
}
impl_inherits!(UITextSizeConstraint, UIConstraint);
impl Default for UITextSizeConstraint {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = UIBase { superclass };
        let superclass = UIComponent { superclass };
        let superclass = UIConstraint { superclass };
        Self {
            superclass,
            MaxTextSize: 100i32,
            MinTextSize: 1i32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UnionOperation {
    #[doc(hidden)]
    pub superclass: PartOperation,
}
impl_inherits!(UnionOperation, PartOperation);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UniqueIdLookupService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(UniqueIdLookupService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UniversalConstraint {
    #[doc(hidden)]
    pub superclass: Constraint,
    pub LimitsEnabled: bool,
    pub MaxAngle: f32,
    pub Radius: f32,
    pub Restitution: f32,
}
impl_inherits!(UniversalConstraint, Constraint);
impl Default for UniversalConstraint {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Constraint {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            Color: BrickColor::from_number(1009u16).unwrap(),
            Enabled: true,
            Visible: false,
        };
        Self {
            superclass,
            LimitsEnabled: false,
            MaxAngle: 45f32,
            Radius: 0.2f32,
            Restitution: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UnreliableRemoteEvent {
    #[doc(hidden)]
    pub superclass: BaseRemoteEvent,
}
impl_inherits!(UnreliableRemoteEvent, BaseRemoteEvent);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UnvalidatedAssetService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub CachedData: String,
}
impl_inherits!(UnvalidatedAssetService, Instance);
impl Default for UnvalidatedAssetService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            CachedData: "{\"lastSaveTime\":0,\"lastKnownPublishRequest\":0,\"users\":[]}"
                .to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UserGameSettings {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AllTutorialsDisabled: bool,
    pub BadgeVisible: bool,
    pub CameraMode: enums::CustomCameraMode,
    pub CameraYInverted: bool,
    pub ChatTranslationEnabled: bool,
    pub ChatTranslationFtuxShown: bool,
    pub ChatTranslationLocale: String,
    pub ChatTranslationToggleEnabled: bool,
    pub ChatVisible: bool,
    pub CompletedTutorials: String,
    pub ComputerCameraMovementChanged: bool,
    pub ComputerCameraMovementMode: enums::ComputerCameraMovementMode,
    pub ComputerMovementChanged: bool,
    pub ComputerMovementMode: enums::ComputerMovementMode,
    pub ControlMode: enums::ControlMode,
    pub DefaultCameraId: String,
    pub FramerateCap: i32,
    pub Fullscreen: bool,
    pub GaId: String,
    pub GamepadCameraSensitivity: f32,
    pub GraphicsOptimizationMode: enums::GraphicsOptimizationMode,
    pub GraphicsQualityLevel: i32,
    pub HapticStrength: f32,
    pub HasEverUsedVr: bool,
    pub MasterVolume: f32,
    pub MasterVolumeStudio: f32,
    pub MaxQualityEnabled: bool,
    pub MicroProfilerWebServerEnabled: bool,
    pub MouseSensitivity: f32,
    pub MouseSensitivityFirstPerson: Vector2,
    pub MouseSensitivityThirdPerson: Vector2,
    pub OnScreenProfilerEnabled: bool,
    pub PartyVoiceVolume: f32,
    pub PeoplePageLayout: enums::PeoplePageLayout,
    pub PerformanceStatsVisible: bool,
    pub PlayerHeight: f32,
    pub PlayerListVisible: bool,
    pub PlayerNamesEnabled: bool,
    pub PreferredTextSize: enums::PreferredTextSize,
    pub PreferredTransparency: f32,
    pub QualityResetLevel: i32,
    pub RccProfilerRecordFrameRate: i32,
    pub RccProfilerRecordTimeFrame: i32,
    pub ReadAloud: bool,
    pub ReducedMotion: bool,
    pub SavedQualityLevel: enums::SavedQualitySetting,
    pub StartMaximized: bool,
    pub StartScreenPosition: Vector2,
    pub StartScreenSize: Vector2,
    pub TouchCameraMovementChanged: bool,
    pub TouchCameraMovementMode: enums::TouchCameraMovementMode,
    pub TouchMovementChanged: bool,
    pub TouchMovementMode: enums::TouchMovementMode,
    pub UiNavigationKeyBindEnabled: bool,
    pub UsedCoreGuiIsVisibleToggle: bool,
    pub UsedCustomGuiIsVisibleToggle: bool,
    pub UsedHideHudShortcut: bool,
    pub VignetteEnabled: bool,
    pub VignetteEnabledCustomOption: bool,
    pub VrComfortSetting: enums::VRComfortSetting,
    pub VrEnabled: bool,
    pub VrRotationIntensity: i32,
    pub VrSafetyBubbleMode: enums::VRSafetyBubbleMode,
    pub VrSmoothRotationEnabled: bool,
    pub VrSmoothRotationEnabledCustomOption: bool,
    pub VrThirdPersonFollowCamEnabled: bool,
    pub VrThirdPersonFollowCamEnabledCustomOption: bool,
}
impl_inherits!(UserGameSettings, Instance);
impl Default for UserGameSettings {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AllTutorialsDisabled: false,
            BadgeVisible: false,
            CameraMode: enums::CustomCameraMode::Default,
            CameraYInverted: false,
            ChatTranslationEnabled: false,
            ChatTranslationFtuxShown: false,
            ChatTranslationLocale: "".to_owned(),
            ChatTranslationToggleEnabled: false,
            ChatVisible: false,
            CompletedTutorials: "".to_owned(),
            ComputerCameraMovementChanged: false,
            ComputerCameraMovementMode: enums::ComputerCameraMovementMode::Default,
            ComputerMovementChanged: false,
            ComputerMovementMode: enums::ComputerMovementMode::Default,
            ControlMode: enums::ControlMode::Classic,
            DefaultCameraId: "".to_owned(),
            FramerateCap: 0i32,
            Fullscreen: false,
            GaId: "".to_owned(),
            GamepadCameraSensitivity: 0f32,
            GraphicsOptimizationMode: enums::GraphicsOptimizationMode::Performance,
            GraphicsQualityLevel: 0i32,
            HapticStrength: 0f32,
            HasEverUsedVr: false,
            MasterVolume: 0f32,
            MasterVolumeStudio: 0f32,
            MaxQualityEnabled: false,
            MicroProfilerWebServerEnabled: false,
            MouseSensitivity: 0f32,
            MouseSensitivityFirstPerson: Vector2::new(0f32, 0f32),
            MouseSensitivityThirdPerson: Vector2::new(0f32, 0f32),
            OnScreenProfilerEnabled: false,
            PartyVoiceVolume: 0f32,
            PeoplePageLayout: enums::PeoplePageLayout::Card,
            PerformanceStatsVisible: false,
            PlayerHeight: 0f32,
            PlayerListVisible: false,
            PlayerNamesEnabled: false,
            PreferredTextSize: enums::PreferredTextSize::Medium,
            PreferredTransparency: 0f32,
            QualityResetLevel: 0i32,
            RccProfilerRecordFrameRate: 0i32,
            RccProfilerRecordTimeFrame: 0i32,
            ReadAloud: false,
            ReducedMotion: false,
            SavedQualityLevel: enums::SavedQualitySetting::Automatic,
            StartMaximized: false,
            StartScreenPosition: Vector2::new(0f32, 0f32),
            StartScreenSize: Vector2::new(0f32, 0f32),
            TouchCameraMovementChanged: false,
            TouchCameraMovementMode: enums::TouchCameraMovementMode::Default,
            TouchMovementChanged: false,
            TouchMovementMode: enums::TouchMovementMode::Default,
            UiNavigationKeyBindEnabled: false,
            UsedCoreGuiIsVisibleToggle: false,
            UsedCustomGuiIsVisibleToggle: false,
            UsedHideHudShortcut: false,
            VignetteEnabled: false,
            VignetteEnabledCustomOption: false,
            VrComfortSetting: enums::VRComfortSetting::Comfort,
            VrEnabled: false,
            VrRotationIntensity: 0i32,
            VrSafetyBubbleMode: enums::VRSafetyBubbleMode::NoOne,
            VrSmoothRotationEnabled: false,
            VrSmoothRotationEnabledCustomOption: false,
            VrThirdPersonFollowCamEnabled: false,
            VrThirdPersonFollowCamEnabledCustomOption: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UserInputService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub LegacyInputEventsEnabled: bool,
    pub MouseBehavior: enums::MouseBehavior,
    pub MouseIconContent: Content,
    pub MouseIconEnabled: bool,
}
impl_inherits!(UserInputService, Instance);
impl Default for UserInputService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            LegacyInputEventsEnabled: false,
            MouseBehavior: enums::MouseBehavior::Default,
            MouseIconContent: Content::none(),
            MouseIconEnabled: false,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UserService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(UserService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UserSettings {
    #[doc(hidden)]
    pub superclass: GenericSettings,
}
impl_inherits!(UserSettings, GenericSettings);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UserStorageService {
    #[doc(hidden)]
    pub superclass: LocalStorageService,
}
impl_inherits!(UserStorageService, LocalStorageService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VRService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub AutomaticScaling: enums::VRScaling,
    pub AvatarGestures: bool,
    pub ControllerModels: enums::VRControllerModelMode,
    pub FadeOutViewOnCollision: bool,
    pub LaserPointer: enums::VRLaserPointerMode,
}
impl_inherits!(VRService, Instance);
impl Default for VRService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            AutomaticScaling: enums::VRScaling::World,
            AvatarGestures: false,
            ControllerModels: enums::VRControllerModelMode::Transparent,
            FadeOutViewOnCollision: true,
            LaserPointer: enums::VRLaserPointerMode::Pointer,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VRStatusService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(VRStatusService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ValueBase {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(ValueBase, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ValueCurve {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ValuesAndTimes: BinaryString,
}
impl_inherits!(ValueCurve, Instance);
impl Default for ValueCurve {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ValuesAndTimes: b"\x02\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0".as_slice().into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Vector3Curve {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(Vector3Curve, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Vector3Value {
    #[doc(hidden)]
    pub superclass: ValueBase,
    pub Value: Vector3,
}
impl_inherits!(Vector3Value, ValueBase);
impl Default for Vector3Value {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        Self {
            superclass,
            Value: Vector3::new(0f32, 0f32, 0f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VectorForce {
    #[doc(hidden)]
    pub superclass: Constraint,
    pub ApplyAtCenterOfMass: bool,
    pub Force: Vector3,
    pub RelativeTo: enums::ActuatorRelativeTo,
}
impl_inherits!(VectorForce, Constraint);
impl Default for VectorForce {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Constraint {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            Color: BrickColor::from_number(23u16).unwrap(),
            Enabled: true,
            Visible: false,
        };
        Self {
            superclass,
            ApplyAtCenterOfMass: false,
            Force: Vector3::new(1000f32, 0f32, 0f32),
            RelativeTo: enums::ActuatorRelativeTo::Attachment0,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VehicleController {
    #[doc(hidden)]
    pub superclass: Controller,
}
impl_inherits!(VehicleController, Controller);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VehicleSeat {
    #[doc(hidden)]
    pub superclass: BasePart,
    pub Disabled: bool,
    pub HeadsUpDisplay: bool,
    pub MaxSpeed: f32,
    pub Steer: i32,
    pub SteerFloat: f32,
    pub Throttle: i32,
    pub ThrottleFloat: f32,
    pub Torque: f32,
    pub TurnSpeed: f32,
}
impl_inherits!(VehicleSeat, BasePart);
impl Default for VehicleSeat {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PVInstance { superclass };
        let superclass = BasePart {
            superclass,
            Anchored: false,
            AudioCanCollide: true,
            BackParamA: -0.5f32,
            BackParamB: 0.5f32,
            BackSurface: enums::SurfaceType::Smooth,
            BackSurfaceInput: enums::InputType::NoInput,
            BottomParamA: -0.5f32,
            BottomParamB: 0.5f32,
            BottomSurface: enums::SurfaceType::Inlet,
            BottomSurfaceInput: enums::InputType::NoInput,
            CFrame: CFrame::identity(),
            CanCollide: true,
            CanQuery: true,
            CanTouch: true,
            CastShadow: true,
            CollisionGroup: "Default".to_owned(),
            CollisionGroupId: 0i32,
            CustomPhysicalProperties: PhysicalProperties::Default,
            EnableFluidForces: true,
            FrontParamA: -0.5f32,
            FrontParamB: 0.5f32,
            FrontSurface: enums::SurfaceType::Smooth,
            FrontSurfaceInput: enums::InputType::NoInput,
            LeftParamA: -0.5f32,
            LeftParamB: 0.5f32,
            LeftSurface: enums::SurfaceType::Smooth,
            LeftSurfaceInput: enums::InputType::NoInput,
            Locked: false,
            Massless: false,
            Material: enums::Material::Plastic,
            PivotOffset: CFrame::identity(),
            Reflectance: 0f32,
            RightParamA: -0.5f32,
            RightParamB: 0.5f32,
            RightSurface: enums::SurfaceType::Smooth,
            RightSurfaceInput: enums::InputType::NoInput,
            RootPriority: 0i32,
            RotVelocity: Vector3::new(0f32, 0f32, 0f32),
            TopParamA: -0.5f32,
            TopParamB: 0.5f32,
            TopSurface: enums::SurfaceType::Studs,
            TopSurfaceInput: enums::InputType::NoInput,
            Transparency: 0f32,
            Velocity: Vector3::new(0f32, 0f32, 0f32),
        };
        Self {
            superclass,
            Disabled: false,
            HeadsUpDisplay: true,
            MaxSpeed: 25f32,
            Steer: 0i32,
            SteerFloat: 0f32,
            Throttle: 0i32,
            ThrottleFloat: 0f32,
            Torque: 10f32,
            TurnSpeed: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VelocityMotor {
    #[doc(hidden)]
    pub superclass: JointInstance,
    pub CurrentAngle: f32,
    pub DesiredAngle: f32,
    pub Hole: Ref,
    pub MaxVelocity: f32,
}
impl_inherits!(VelocityMotor, JointInstance);
impl Default for VelocityMotor {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = JointInstance {
            superclass,
            C0: CFrame::identity(),
            C1: CFrame::identity(),
            Enabled: true,
            Part0: Ref::none(),
            Part1: Ref::none(),
        };
        Self {
            superclass,
            CurrentAngle: 0f32,
            DesiredAngle: 0f32,
            Hole: Ref::none(),
            MaxVelocity: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VersionControlService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(VersionControlService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VideoCapture {
    #[doc(hidden)]
    pub superclass: Capture,
}
impl_inherits!(VideoCapture, Capture);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VideoCaptureService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(VideoCaptureService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VideoDeviceInput {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Active: bool,
    pub CameraId: String,
    pub CaptureQuality: enums::VideoDeviceCaptureQuality,
}
impl_inherits!(VideoDeviceInput, Instance);
impl Default for VideoDeviceInput {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Active: false,
            CameraId: "".to_owned(),
            CaptureQuality: enums::VideoDeviceCaptureQuality::Default,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VideoDisplay {
    #[doc(hidden)]
    pub superclass: GuiObject,
    pub ResampleMode: enums::ResamplerMode,
    pub ScaleType: enums::ScaleType,
    pub TileSize: UDim2,
    pub VideoColor3: Color3,
    pub VideoRectOffset: Vector2,
    pub VideoRectSize: Vector2,
    pub VideoTransparency: f32,
}
impl_inherits!(VideoDisplay, GuiObject);
impl Default for VideoDisplay {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase2d {
            superclass,
            AutoLocalize: true,
            RootLocalizationTable: Ref::none(),
            SelectionBehaviorDown: enums::SelectionBehavior::Escape,
            SelectionBehaviorLeft: enums::SelectionBehavior::Escape,
            SelectionBehaviorRight: enums::SelectionBehavior::Escape,
            SelectionBehaviorUp: enums::SelectionBehavior::Escape,
            SelectionGroup: false,
        };
        let superclass = GuiObject {
            superclass,
            Active: false,
            AnchorPoint: Vector2::new(0f32, 0f32),
            AutomaticSize: enums::AutomaticSize::None,
            BackgroundColor3: Color3::new(0.6392157f32, 0.63529414f32, 0.64705884f32),
            BackgroundTransparency: 0f32,
            BorderColor3: Color3::new(0.105882354f32, 0.16470589f32, 0.20784314f32),
            BorderMode: enums::BorderMode::Outline,
            BorderSizePixel: 1i32,
            ClipsDescendants: false,
            Draggable: false,
            Interactable: true,
            LayoutOrder: 0i32,
            NextSelectionDown: Ref::none(),
            NextSelectionLeft: Ref::none(),
            NextSelectionRight: Ref::none(),
            NextSelectionUp: Ref::none(),
            Position: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            Rotation: 0f32,
            Selectable: false,
            SelectionImageObject: Ref::none(),
            SelectionOrder: 0i32,
            Size: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            SizeConstraint: enums::SizeConstraint::RelativeXY,
            Visible: true,
            ZIndex: 1i32,
        };
        Self {
            superclass,
            ResampleMode: enums::ResamplerMode::Default,
            ScaleType: enums::ScaleType::Stretch,
            TileSize: UDim2::new(UDim::new(1f32, 0i32), UDim::new(1f32, 0i32)),
            VideoColor3: Color3::new(1f32, 1f32, 1f32),
            VideoRectOffset: Vector2::new(0f32, 0f32),
            VideoRectSize: Vector2::new(0f32, 0f32),
            VideoTransparency: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VideoFrame {
    #[doc(hidden)]
    pub superclass: GuiObject,
    pub Looped: bool,
    pub Playing: bool,
    pub TimePosition: f64,
    pub VideoContent: Content,
    pub Volume: f32,
}
impl_inherits!(VideoFrame, GuiObject);
impl Default for VideoFrame {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase2d {
            superclass,
            AutoLocalize: true,
            RootLocalizationTable: Ref::none(),
            SelectionBehaviorDown: enums::SelectionBehavior::Escape,
            SelectionBehaviorLeft: enums::SelectionBehavior::Escape,
            SelectionBehaviorRight: enums::SelectionBehavior::Escape,
            SelectionBehaviorUp: enums::SelectionBehavior::Escape,
            SelectionGroup: false,
        };
        let superclass = GuiObject {
            superclass,
            Active: false,
            AnchorPoint: Vector2::new(0f32, 0f32),
            AutomaticSize: enums::AutomaticSize::None,
            BackgroundColor3: Color3::new(0.6392157f32, 0.63529414f32, 0.64705884f32),
            BackgroundTransparency: 0f32,
            BorderColor3: Color3::new(0.105882354f32, 0.16470589f32, 0.20784314f32),
            BorderMode: enums::BorderMode::Outline,
            BorderSizePixel: 1i32,
            ClipsDescendants: false,
            Draggable: false,
            Interactable: true,
            LayoutOrder: 0i32,
            NextSelectionDown: Ref::none(),
            NextSelectionLeft: Ref::none(),
            NextSelectionRight: Ref::none(),
            NextSelectionUp: Ref::none(),
            Position: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            Rotation: 0f32,
            Selectable: false,
            SelectionImageObject: Ref::none(),
            SelectionOrder: 0i32,
            Size: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            SizeConstraint: enums::SizeConstraint::RelativeXY,
            Visible: true,
            ZIndex: 1i32,
        };
        Self {
            superclass,
            Looped: false,
            Playing: false,
            TimePosition: 0f64,
            VideoContent: Content::none(),
            Volume: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VideoPlayer {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Looping: bool,
    pub PlaybackSpeed: f32,
    pub TimePosition: f64,
    pub VideoContent: Content,
    pub Volume: f32,
}
impl_inherits!(VideoPlayer, Instance);
impl Default for VideoPlayer {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Looping: false,
            PlaybackSpeed: 1f32,
            TimePosition: 0f64,
            VideoContent: Content::none(),
            Volume: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VideoSampler {
    #[doc(hidden)]
    pub superclass: Object,
}
impl_inherits!(VideoSampler, Object);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VideoScreenCaptureService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(VideoScreenCaptureService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VideoService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(VideoService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ViewportFrame {
    #[doc(hidden)]
    pub superclass: GuiObject,
    pub Ambient: Color3,
    pub CameraCFrame: CFrame,
    pub CameraFieldOfView: f32,
    pub ImageColor3: Color3,
    pub ImageTransparency: f32,
    pub LightColor: Color3,
    pub LightDirection: Vector3,
}
impl_inherits!(ViewportFrame, GuiObject);
impl Default for ViewportFrame {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase2d {
            superclass,
            AutoLocalize: true,
            RootLocalizationTable: Ref::none(),
            SelectionBehaviorDown: enums::SelectionBehavior::Escape,
            SelectionBehaviorLeft: enums::SelectionBehavior::Escape,
            SelectionBehaviorRight: enums::SelectionBehavior::Escape,
            SelectionBehaviorUp: enums::SelectionBehavior::Escape,
            SelectionGroup: false,
        };
        let superclass = GuiObject {
            superclass,
            Active: false,
            AnchorPoint: Vector2::new(0f32, 0f32),
            AutomaticSize: enums::AutomaticSize::None,
            BackgroundColor3: Color3::new(0.6392157f32, 0.63529414f32, 0.64705884f32),
            BackgroundTransparency: 0f32,
            BorderColor3: Color3::new(0.105882354f32, 0.16470589f32, 0.20784314f32),
            BorderMode: enums::BorderMode::Outline,
            BorderSizePixel: 1i32,
            ClipsDescendants: false,
            Draggable: false,
            Interactable: true,
            LayoutOrder: 0i32,
            NextSelectionDown: Ref::none(),
            NextSelectionLeft: Ref::none(),
            NextSelectionRight: Ref::none(),
            NextSelectionUp: Ref::none(),
            Position: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            Rotation: 0f32,
            Selectable: false,
            SelectionImageObject: Ref::none(),
            SelectionOrder: 0i32,
            Size: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
            SizeConstraint: enums::SizeConstraint::RelativeXY,
            Visible: true,
            ZIndex: 1i32,
        };
        Self {
            superclass,
            Ambient: Color3::new(0.78431374f32, 0.78431374f32, 0.78431374f32),
            CameraCFrame: CFrame::identity(),
            CameraFieldOfView: 1.2217306f32,
            ImageColor3: Color3::new(1f32, 1f32, 1f32),
            ImageTransparency: 0f32,
            LightColor: Color3::new(0.54901963f32, 0.54901963f32, 0.54901963f32),
            LightDirection: Vector3::new(-1f32, -1f32, -1f32),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VirtualInputManager {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(VirtualInputManager, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VirtualUser {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(VirtualUser, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VisibilityCheckDispatcher {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(VisibilityCheckDispatcher, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Visit {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(Visit, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VisualizationMode {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Enabled: bool,
    pub Title: String,
    pub ToolTip: String,
}
impl_inherits!(VisualizationMode, Instance);
impl Default for VisualizationMode {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Enabled: false,
            Title: "".to_owned(),
            ToolTip: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VisualizationModeCategory {
    #[doc(hidden)]
    pub superclass: Instance,
    pub Enabled: bool,
    pub Title: String,
}
impl_inherits!(VisualizationModeCategory, Instance);
impl Default for VisualizationModeCategory {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            Enabled: false,
            Title: "".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VisualizationModeService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(VisualizationModeService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VoiceChatInternal {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(VoiceChatInternal, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VoiceChatService {
    #[doc(hidden)]
    pub superclass: Instance,
    pub DefaultDistanceAttenuation: enums::VoiceChatDistanceAttenuationType,
    pub EnableDefaultVoice: bool,
    pub UseAudioApi: enums::AudioApiRollout,
}
impl_inherits!(VoiceChatService, Instance);
impl Default for VoiceChatService {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            DefaultDistanceAttenuation: enums::VoiceChatDistanceAttenuationType::Inverse,
            EnableDefaultVoice: true,
            UseAudioApi: enums::AudioApiRollout::Automatic,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WebSocketClient {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(WebSocketClient, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WebSocketService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(WebSocketService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WebStreamClient {
    #[doc(hidden)]
    pub superclass: Object,
}
impl_inherits!(WebStreamClient, Object);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WebViewService {
    #[doc(hidden)]
    pub superclass: Instance,
}
impl_inherits!(WebViewService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WedgePart {
    #[doc(hidden)]
    pub superclass: FormFactorPart,
}
impl_inherits!(WedgePart, FormFactorPart);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Weld {
    #[doc(hidden)]
    pub superclass: JointInstance,
}
impl_inherits!(Weld, JointInstance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WeldConstraint {
    #[doc(hidden)]
    pub superclass: Instance,
    pub CFrame0: CFrame,
    pub State: i32,
}
impl_inherits!(WeldConstraint, Instance);
impl Default for WeldConstraint {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            CFrame0: CFrame::identity(),
            State: 3i32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Wire {
    #[doc(hidden)]
    pub superclass: Instance,
    pub SourceInstance: Ref,
    pub SourceName: String,
    pub TargetInstance: Ref,
    pub TargetName: String,
}
impl_inherits!(Wire, Instance);
impl Default for Wire {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            SourceInstance: Ref::none(),
            SourceName: "Output".to_owned(),
            TargetInstance: Ref::none(),
            TargetName: "Input".to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WireframeHandleAdornment {
    #[doc(hidden)]
    pub superclass: HandleAdornment,
    pub Scale: Vector3,
    pub Thickness: f32,
}
impl_inherits!(WireframeHandleAdornment, HandleAdornment);
impl Default for WireframeHandleAdornment {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiBase { superclass };
        let superclass = GuiBase3d {
            superclass,
            Color3: Color3::new(0.050980393f32, 0.4117647f32, 0.6745098f32),
            Transparency: 0f32,
            Visible: true,
        };
        let superclass = PVAdornment {
            superclass,
            Adornee: Ref::none(),
        };
        let superclass = HandleAdornment {
            superclass,
            AdornCullingMode: enums::AdornCullingMode::Automatic,
            AlwaysOnTop: false,
            CFrame: CFrame::identity(),
            SizeRelativeOffset: Vector3::new(0f32, 0f32, 0f32),
            ZIndex: -1i32,
        };
        Self {
            superclass,
            Scale: Vector3::new(1f32, 1f32, 1f32),
            Thickness: 1f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Workspace {
    #[doc(hidden)]
    pub superclass: WorldRoot,
    pub AirDensity: f32,
    pub AirTurbulenceIntensity: f32,
    pub AllowThirdPartySales: bool,
    pub AuthorityMode: enums::AuthorityMode,
    pub AvatarUnificationMode: enums::AvatarUnificationMode,
    pub ClientAnimatorThrottling: enums::ClientAnimatorThrottlingMode,
    pub CollisionGroupData: BinaryString,
    pub CollisionGroups: String,
    pub CurrentCamera: Ref,
    pub DistributedGameTime: f64,
    pub ExplicitAutoJoints: bool,
    pub FallHeightEnabled: bool,
    pub FallenPartsDestroyHeight: f32,
    pub FluidForces: enums::FluidForces,
    pub GlobalWind: Vector3,
    pub Gravity: f32,
    pub IkControlConstraintSupport: enums::IKControlConstraintSupport,
    pub LuauTypeCheckMode: enums::LuauTypeCheckMode,
    pub MeshPartHeadsAndAccessories: enums::MeshPartHeadsAndAccessories,
    pub ModelStreamingBehavior: enums::ModelStreamingBehavior,
    pub MoverConstraintRootBehavior: enums::MoverConstraintRootBehaviorMode,
    pub PathfindingUseImprovedSearch: enums::PathfindingUseImprovedSearch,
    pub PhysicsImprovedSleep: enums::RolloutState,
    pub PhysicsSteppingMethod: enums::PhysicsSteppingMethod,
    pub PlayerCharacterDestroyBehavior: enums::PlayerCharacterDestroyBehavior,
    pub PrimalPhysicsSolver: enums::PrimalPhysicsSolver,
    pub RejectCharacterDeletions: enums::RejectCharacterDeletions,
    pub RenderingCacheOptimizations: enums::RenderingCacheOptimizationMode,
    pub ReplicateInstanceDestroySetting: enums::ReplicateInstanceDestroySetting,
    pub Retargeting: enums::AnimatorRetargetingMode,
    pub SandboxedInstanceMode: enums::SandboxedInstanceMode,
    pub StreamOutBehavior: enums::StreamOutBehavior,
    pub StreamingEnabled: bool,
    pub StreamingIntegrityMode: enums::StreamingIntegrityMode,
    pub StreamingMinRadius: i32,
    pub StreamingTargetRadius: i32,
    pub TerrainWeldsFixed: bool,
    pub TouchEventsUseCollisionGroups: enums::RolloutState,
    pub TouchesUseCollisionGroups: bool,
    pub UseNewLuauTypeSolver: enums::RolloutState,
}
impl_inherits!(Workspace, WorldRoot);
impl Default for Workspace {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PVInstance { superclass };
        let superclass = Model {
            superclass,
            LevelOfDetail: enums::ModelLevelOfDetail::Automatic,
            ModelMeshCFrame: CFrame::identity(),
            ModelMeshData: SharedString::new(b"".to_vec()),
            ModelMeshSize: Vector3::new(0f32, 0f32, 0f32),
            ModelStreamingMode: enums::ModelStreamingMode::Default,
            NeedsPivotMigration: false,
            PrimaryPart: Ref::none(),
            SlimHash: SharedString::new(b"".to_vec()),
            WorldPivotData: None,
        };
        let superclass = WorldRoot { superclass };
        Self {
            superclass,
            AirDensity: 0.0012f32,
            AirTurbulenceIntensity: 0f32,
            AllowThirdPartySales: false,
            AuthorityMode: enums::AuthorityMode::Automatic,
            AvatarUnificationMode: enums::AvatarUnificationMode::Default,
            ClientAnimatorThrottling: enums::ClientAnimatorThrottlingMode::Default,
            CollisionGroupData: b"\x01\x01\0\x04\xFF\xFF\xFF\xFF\x07Default"
                .as_slice()
                .into(),
            CollisionGroups: "".to_owned(),
            CurrentCamera: Ref::none(),
            DistributedGameTime: 0f64,
            ExplicitAutoJoints: true,
            FallHeightEnabled: true,
            FallenPartsDestroyHeight: -500f32,
            FluidForces: enums::FluidForces::Default,
            GlobalWind: Vector3::new(0f32, 0f32, 0f32),
            Gravity: 196.2f32,
            IkControlConstraintSupport: enums::IKControlConstraintSupport::Default,
            LuauTypeCheckMode: enums::LuauTypeCheckMode::Default,
            MeshPartHeadsAndAccessories: enums::MeshPartHeadsAndAccessories::Default,
            ModelStreamingBehavior: enums::ModelStreamingBehavior::Default,
            MoverConstraintRootBehavior: enums::MoverConstraintRootBehaviorMode::Default,
            PathfindingUseImprovedSearch: enums::PathfindingUseImprovedSearch::Default,
            PhysicsImprovedSleep: enums::RolloutState::Default,
            PhysicsSteppingMethod: enums::PhysicsSteppingMethod::Default,
            PlayerCharacterDestroyBehavior: enums::PlayerCharacterDestroyBehavior::Default,
            PrimalPhysicsSolver: enums::PrimalPhysicsSolver::Default,
            RejectCharacterDeletions: enums::RejectCharacterDeletions::Default,
            RenderingCacheOptimizations: enums::RenderingCacheOptimizationMode::Default,
            ReplicateInstanceDestroySetting: enums::ReplicateInstanceDestroySetting::Default,
            Retargeting: enums::AnimatorRetargetingMode::Default,
            SandboxedInstanceMode: enums::SandboxedInstanceMode::Default,
            StreamOutBehavior: enums::StreamOutBehavior::Default,
            StreamingEnabled: false,
            StreamingIntegrityMode: enums::StreamingIntegrityMode::Default,
            StreamingMinRadius: 64i32,
            StreamingTargetRadius: 1024i32,
            TerrainWeldsFixed: true,
            TouchEventsUseCollisionGroups: enums::RolloutState::Default,
            TouchesUseCollisionGroups: false,
            UseNewLuauTypeSolver: enums::RolloutState::Enabled,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WorkspaceAnnotation {
    #[doc(hidden)]
    pub superclass: Annotation,
}
impl_inherits!(WorkspaceAnnotation, Annotation);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WorldModel {
    #[doc(hidden)]
    pub superclass: WorldRoot,
}
impl_inherits!(WorldModel, WorldRoot);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WorldRoot {
    #[doc(hidden)]
    pub superclass: Model,
}
impl_inherits!(WorldRoot, Model);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WrapDeformer {
    #[doc(hidden)]
    pub superclass: BaseWrap,
}
impl_inherits!(WrapDeformer, BaseWrap);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WrapLayer {
    #[doc(hidden)]
    pub superclass: BaseWrap,
    pub AutoSkin: enums::WrapLayerAutoSkin,
    pub BindOffset: CFrame,
    pub Enabled: bool,
    pub Order: i32,
    pub Puffiness: f32,
    pub ReferenceMeshContent: Content,
    pub ReferenceOrigin: CFrame,
    pub ShrinkFactor: f32,
    pub TemporaryReferenceId: ContentId,
}
impl_inherits!(WrapLayer, BaseWrap);
impl Default for WrapLayer {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BaseWrap {
            superclass,
            CageMeshContent: Content::none(),
            CageOrigin: CFrame::identity(),
            HsrAssetId: "".into(),
            HsrData: SharedString::new(b"".to_vec()),
            HsrMeshIdData: SharedString::new(b"".to_vec()),
            ImportOrigin: CFrame::identity(),
            TemporaryCageMeshId: "".into(),
        };
        Self {
            superclass,
            AutoSkin: enums::WrapLayerAutoSkin::Disabled,
            BindOffset: CFrame::identity(),
            Enabled: true,
            Order: 1i32,
            Puffiness: 1f32,
            ReferenceMeshContent: Content::none(),
            ReferenceOrigin: CFrame::identity(),
            ShrinkFactor: 0f32,
            TemporaryReferenceId: "".into(),
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WrapTarget {
    #[doc(hidden)]
    pub superclass: BaseWrap,
    pub Stiffness: f32,
}
impl_inherits!(WrapTarget, BaseWrap);
impl Default for WrapTarget {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BaseWrap {
            superclass,
            CageMeshContent: Content::none(),
            CageOrigin: CFrame::identity(),
            HsrAssetId: "".into(),
            HsrData: SharedString::new(b"".to_vec()),
            HsrMeshIdData: SharedString::new(b"".to_vec()),
            ImportOrigin: CFrame::identity(),
            TemporaryCageMeshId: "".into(),
        };
        Self {
            superclass,
            Stiffness: 0f32,
        }
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WrapTextureTransfer {
    #[doc(hidden)]
    pub superclass: Instance,
    pub ReferenceCageMeshContent: Content,
    pub UvMaxBound: Vector2,
    pub UvMinBound: Vector2,
}
impl_inherits!(WrapTextureTransfer, Instance);
impl Default for WrapTextureTransfer {
    fn default() -> Self {
        let superclass = Object::default();
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: -1i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        Self {
            superclass,
            ReferenceCageMeshContent: Content::none(),
            UvMaxBound: Vector2::new(f32::NEG_INFINITY, f32::NEG_INFINITY),
            UvMinBound: Vector2::new(f32::INFINITY, f32::INFINITY),
        }
    }
}
