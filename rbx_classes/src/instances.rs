use super::enums;
use crate::impl_inherits;
use core::ops::{Deref, DerefMut};
use rbx_types::*;
#[doc = r" Invoke a macro with every class ident,"]
#[doc = r" i.e"]
#[doc = r" ```rust"]
#[doc = r" for_each_class!(my_macro);"]
#[doc = r" ```"]
#[doc = r" invokes"]
#[doc = r" ```rust"]
#[doc = r" my_macro!(Accoutrement, Part, WedgePart, ...);"]
#[doc = r" ```"]
#[macro_export]
macro_rules! for_each_class {
    ($ my_macro : ident) => {
        $my_macro!(
            Accessory,
            AccessoryDescription,
            AccountService,
            Accoutrement,
            AchievementService,
            ActivityHistoryEventService,
            Actor,
            AdGui,
            AdPortal,
            AdService,
            AdvancedDragger,
            AirController,
            AlignOrientation,
            AlignPosition,
            AnalyticsService,
            AngularVelocity,
            Animation,
            AnimationClip,
            AnimationClipProvider,
            AnimationConstraint,
            AnimationController,
            AnimationFromVideoCreatorService,
            AnimationFromVideoCreatorStudioService,
            AnimationGraphDefinition,
            AnimationImportData,
            AnimationNode,
            AnimationNodeDefinition,
            AnimationRigData,
            AnimationStreamTrack,
            AnimationTrack,
            Animator,
            Annotation,
            AnnotationsService,
            AppLifecycleObserverService,
            AppRatingPromptService,
            AppStorageService,
            AppUpdateService,
            ArcHandles,
            AssetCounterService,
            AssetDeliveryProxy,
            AssetImportService,
            AssetImportSession,
            AssetManagerService,
            AssetPatchSettings,
            AssetService,
            AssetSoundEffect,
            Atmosphere,
            AtmosphereSensor,
            Attachment,
            AudioAnalyzer,
            AudioChannelMixer,
            AudioChannelSplitter,
            AudioChorus,
            AudioCompressor,
            AudioDeviceInput,
            AudioDeviceOutput,
            AudioDistortion,
            AudioEcho,
            AudioEmitter,
            AudioEqualizer,
            AudioFader,
            AudioFilter,
            AudioFlanger,
            AudioFocusService,
            AudioGate,
            AudioLimiter,
            AudioListener,
            AudioPages,
            AudioPitchShifter,
            AudioPlayer,
            AudioRecorder,
            AudioReverb,
            AudioSearchParams,
            AudioSpeechToText,
            AudioTextToSpeech,
            AudioTremolo,
            AuroraScript,
            AuroraScriptObject,
            AuroraScriptService,
            AuroraService,
            AvatarAccessoryRules,
            AvatarAnimationRules,
            AvatarBodyRules,
            AvatarChatService,
            AvatarClothingRules,
            AvatarCollisionRules,
            AvatarCreationService,
            AvatarEditorService,
            AvatarImportService,
            AvatarRules,
            AvatarSettings,
            Backpack,
            BackpackItem,
            BadgeService,
            BallSocketConstraint,
            BanHistoryPages,
            BaseImportData,
            BasePart,
            BasePlayerGui,
            BaseRemoteEvent,
            BaseScript,
            BaseWrap,
            Beam,
            BevelMesh,
            BillboardGui,
            BinaryStringValue,
            BindableEvent,
            BindableFunction,
            BlockMesh,
            BloomEffect,
            BlurEffect,
            BodyAngularVelocity,
            BodyColors,
            BodyForce,
            BodyGyro,
            BodyMover,
            BodyPartDescription,
            BodyPosition,
            BodyThrust,
            BodyVelocity,
            Bone,
            BoolValue,
            BoxHandleAdornment,
            Breakpoint,
            BrickColorValue,
            BrowserService,
            BubbleChatConfiguration,
            BubbleChatMessageProperties,
            BugReporterService,
            BulkImportService,
            BuoyancySensor,
            CFrameValue,
            CSGDictionaryService,
            CacheableContentProvider,
            CalloutService,
            Camera,
            CanvasGroup,
            Capture,
            CaptureService,
            CapturesPages,
            CatalogPages,
            ChangeHistoryService,
            ChangeHistoryStreamingService,
            ChannelSelectorSoundEffect,
            ChannelTabsConfiguration,
            CharacterAppearance,
            CharacterMesh,
            Chat,
            ChatInputBarConfiguration,
            ChatWindowConfiguration,
            ChatWindowMessageProperties,
            ChatbotUIService,
            ChorusSoundEffect,
            ClickDetector,
            ClientReplicator,
            ClimbController,
            Clothing,
            CloudCRUDService,
            CloudLocalizationTable,
            Clouds,
            ClusterPacketCache,
            Collaborator,
            CollaboratorsService,
            CollectionService,
            Color3Value,
            ColorCorrectionEffect,
            ColorGradingEffect,
            CommerceService,
            CompressorSoundEffect,
            ConeHandleAdornment,
            ConfigService,
            ConfigSnapshot,
            Configuration,
            ConfigureServerService,
            ConnectivityService,
            Constraint,
            ContentProvider,
            ContextActionService,
            Controller,
            ControllerBase,
            ControllerManager,
            ControllerPartSensor,
            ControllerSensor,
            ControllerService,
            ConversationalAIAcceptanceService,
            CookiesService,
            CoreGui,
            CorePackages,
            CoreScript,
            CoreScriptDebuggingManagerHelper,
            CoreScriptSyncService,
            CornerWedgePart,
            CreationDBService,
            CreatorStoreService,
            CrossDMScriptChangeListener,
            CurveAnimation,
            CustomEvent,
            CustomEventReceiver,
            CustomLog,
            CustomSoundEffect,
            CylinderHandleAdornment,
            CylinderMesh,
            CylindricalConstraint,
            DataModel,
            DataModelMesh,
            DataModelPatchService,
            DataModelSession,
            DataStore,
            DataStoreGetOptions,
            DataStoreIncrementOptions,
            DataStoreInfo,
            DataStoreKey,
            DataStoreKeyInfo,
            DataStoreKeyPages,
            DataStoreListingPages,
            DataStoreObjectVersionInfo,
            DataStoreOptions,
            DataStorePages,
            DataStoreService,
            DataStoreSetOptions,
            DataStoreVersionPages,
            Debris,
            DebugSettings,
            DebuggablePluginWatcher,
            DebuggerBreakpoint,
            DebuggerConnection,
            DebuggerConnectionManager,
            DebuggerLuaResponse,
            DebuggerManager,
            DebuggerUIService,
            DebuggerVariable,
            DebuggerWatch,
            Decal,
            DepthOfFieldEffect,
            DeviceIdService,
            Dialog,
            DialogChoice,
            DistortionSoundEffect,
            DockWidgetPluginGui,
            DoubleConstrainedValue,
            DraftsService,
            DragDetector,
            Dragger,
            DraggerService,
            DynamicRotate,
            EchoSoundEffect,
            EditableImage,
            EditableMesh,
            EditableService,
            EmotesPages,
            EncodingService,
            EqualizerSoundEffect,
            EulerRotationCurve,
            EventIngestService,
            ExampleV2Service,
            ExecutedRemoteCommand,
            ExperienceAuthService,
            ExperienceInviteOptions,
            ExperienceNotificationService,
            ExperienceService,
            ExperienceStateCaptureService,
            ExperienceStateRecordingService,
            ExplorerFilter,
            ExplorerFilterAutocompleter,
            ExplorerServiceVisibilityService,
            Explosion,
            FaceAnimatorService,
            FaceControls,
            FaceInstance,
            FacialAgeEstimationService,
            FacialAnimationRecordingService,
            FacialAnimationStreamingServiceStats,
            FacialAnimationStreamingServiceV2,
            FacialAnimationStreamingSubsessionStats,
            FacsImportData,
            Feature,
            FeatureRestrictionManager,
            File,
            FileMesh,
            Fire,
            Flag,
            FlagStand,
            FlagStandService,
            FlangeSoundEffect,
            FloatCurve,
            FloorWire,
            FluidForceSensor,
            FlyweightService,
            Folder,
            ForceField,
            FormFactorPart,
            Frame,
            FriendPages,
            FriendService,
            FunctionalTest,
            GamePassService,
            GameSettings,
            GamepadService,
            GenerationService,
            GenericChallengeService,
            GenericSettings,
            Geometry,
            GeometryService,
            GetTextBoundsParams,
            GlobalDataStore,
            GlobalSettings,
            Glue,
            GroundController,
            GroupImportData,
            GroupService,
            GuiBase,
            GuiBase2d,
            GuiBase3d,
            GuiButton,
            GuiLabel,
            GuiMain,
            GuiObject,
            GuiService,
            GuidRegistryService,
            HSRDataContentProvider,
            HandRigDescription,
            HandleAdornment,
            Handles,
            HandlesBase,
            HapticEffect,
            HapticService,
            HarmonyService,
            Hat,
            HeapProfilerService,
            HeatmapService,
            HeightmapImporterService,
            HiddenSurfaceRemovalAsset,
            Highlight,
            HingeConstraint,
            Hint,
            Hole,
            Hopper,
            HopperBin,
            HttpRbxApiService,
            HttpRequest,
            HttpService,
            Humanoid,
            HumanoidController,
            HumanoidDescription,
            HumanoidRigDescription,
            IKControl,
            ILegacyStudioBridge,
            IXPService,
            ImageButton,
            ImageHandleAdornment,
            ImageLabel,
            ImportSession,
            IncrementalPatchBuilder,
            InputAction,
            InputBinding,
            InputContext,
            InputObject,
            InsertService,
            Instance,
            InstanceAdornment,
            InstanceExtensionsService,
            IntConstrainedValue,
            IntValue,
            InternalSyncItem,
            InternalSyncService,
            IntersectOperation,
            InventoryPages,
            JointImportData,
            JointInstance,
            JointsService,
            KeyboardService,
            Keyframe,
            KeyframeMarker,
            KeyframeSequence,
            KeyframeSequenceProvider,
            LSPFileSyncService,
            LanguageService,
            LayerCollector,
            LegacyStudioBridge,
            Light,
            Lighting,
            LineForce,
            LineHandleAdornment,
            LinearVelocity,
            LinkingService,
            LiveScriptingService,
            LiveSyncService,
            LocalDebuggerConnection,
            LocalScript,
            LocalStorageService,
            LocalizationService,
            LocalizationTable,
            LodDataEntity,
            LodDataService,
            LogReporterService,
            LogService,
            LoginService,
            LuaSettings,
            LuaSourceContainer,
            LuaWebService,
            LuauScriptAnalyzerService,
            MLModelDeliveryService,
            MLService,
            MLSession,
            ManualGlue,
            ManualSurfaceJointInstance,
            ManualWeld,
            MarkerCurve,
            MarketplaceService,
            MatchmakingService,
            MaterialGenerationService,
            MaterialImportData,
            MaterialService,
            MaterialVariant,
            MemStorageConnection,
            MemStorageService,
            MemoryStoreHashMap,
            MemoryStoreHashMapPages,
            MemoryStoreQueue,
            MemoryStoreService,
            MemoryStoreSortedMap,
            MeshContentProvider,
            MeshImportData,
            MeshPart,
            Message,
            MessageBusConnection,
            MessageBusService,
            MessagingService,
            MetaBreakpoint,
            MetaBreakpointContext,
            MetaBreakpointManager,
            MicroProfilerService,
            Model,
            ModerationService,
            ModuleScript,
            Motor,
            Motor6D,
            MotorFeature,
            Mouse,
            MouseService,
            MultipleDocumentInterfaceInstance,
            NegateOperation,
            NetworkClient,
            NetworkMarker,
            NetworkPeer,
            NetworkReplicator,
            NetworkServer,
            NetworkSettings,
            NoCollisionConstraint,
            Noise,
            NonReplicatedCSGDictionaryService,
            NotificationService,
            NumberPose,
            NumberValue,
            Object,
            ObjectValue,
            OmniRecommendationsService,
            OpenCloudApiV1,
            OpenCloudService,
            OperationGraph,
            OrderedDataStore,
            OutfitPages,
            PVAdornment,
            PVInstance,
            PackageLink,
            PackageService,
            PackageUIService,
            Pages,
            Pants,
            ParabolaAdornment,
            Part,
            PartAdornment,
            PartOperation,
            PartOperationAsset,
            ParticleEmitter,
            PartyEmulatorService,
            PatchBundlerFileWatch,
            PatchMapping,
            Path,
            Path2D,
            PathfindingLink,
            PathfindingModifier,
            PathfindingService,
            PausedState,
            PausedStateBreakpoint,
            PausedStateException,
            PerformanceControlService,
            PermissionsService,
            PhysicsService,
            PhysicsSettings,
            PitchShiftSoundEffect,
            PlaceAssetIdsService,
            PlaceStatsService,
            PlacesService,
            Plane,
            PlaneConstraint,
            Platform,
            PlatformCloudStorageService,
            PlatformFriendsService,
            Player,
            PlayerData,
            PlayerDataRecord,
            PlayerDataRecordConfig,
            PlayerDataService,
            PlayerEmulatorService,
            PlayerGui,
            PlayerHydrationService,
            PlayerMouse,
            PlayerScripts,
            PlayerViewService,
            Players,
            Plugin,
            PluginAction,
            PluginCapabilities,
            PluginDebugService,
            PluginDragEvent,
            PluginGui,
            PluginGuiService,
            PluginManagementService,
            PluginManager,
            PluginManagerInterface,
            PluginMenu,
            PluginMouse,
            PluginPolicyService,
            PluginToolbar,
            PluginToolbarButton,
            PointLight,
            PointsService,
            PolicyService,
            Pose,
            PoseBase,
            PostEffect,
            PrismaticConstraint,
            ProcessInstancePhysicsService,
            ProximityPrompt,
            ProximityPromptService,
            PublishService,
            PyramidHandleAdornment,
            QWidgetPluginGui,
            RTAnimationTracker,
            RayValue,
            RbxAnalyticsService,
            RecommendationPages,
            RecommendationService,
            ReflectionMetadata,
            ReflectionMetadataCallbacks,
            ReflectionMetadataClass,
            ReflectionMetadataClasses,
            ReflectionMetadataEnum,
            ReflectionMetadataEnumItem,
            ReflectionMetadataEnums,
            ReflectionMetadataEvents,
            ReflectionMetadataFunctions,
            ReflectionMetadataItem,
            ReflectionMetadataMember,
            ReflectionMetadataProperties,
            ReflectionMetadataYieldFunctions,
            ReflectionService,
            RelativeGui,
            RemoteCommandService,
            RemoteCursorService,
            RemoteDebuggerServer,
            RemoteEvent,
            RemoteFunction,
            RenderSettings,
            RenderingTest,
            ReplicatedFirst,
            ReplicatedStorage,
            ReverbSoundEffect,
            RibbonNotificationService,
            RigidConstraint,
            RobloxPluginGuiService,
            RobloxReplicatedStorage,
            RobloxSerializableInstance,
            RobloxServerStorage,
            RocketPropulsion,
            RodConstraint,
            RomarkRbxAnalyticsService,
            RomarkService,
            RootImportData,
            RopeConstraint,
            Rotate,
            RotateP,
            RotateV,
            RotationCurve,
            RtMessagingService,
            RunService,
            RunningAverageItemDouble,
            RunningAverageItemInt,
            RunningAverageTimeIntervalItem,
            RuntimeContentService,
            RuntimeScriptService,
            SafetyService,
            ScreenGui,
            ScreenshotCapture,
            ScreenshotHud,
            Script,
            ScriptBuilder,
            ScriptChangeService,
            ScriptCloneWatcher,
            ScriptCloneWatcherHelper,
            ScriptCommitService,
            ScriptContext,
            ScriptDebugger,
            ScriptDocument,
            ScriptEditorService,
            ScriptProfilerService,
            ScriptRegistrationService,
            ScriptRuntime,
            ScriptService,
            ScrollingFrame,
            Seat,
            Selection,
            SelectionBox,
            SelectionHighlightManager,
            SelectionLasso,
            SelectionPartLasso,
            SelectionPointLasso,
            SelectionSphere,
            SensorBase,
            SerializationService,
            ServerReplicator,
            ServerScriptService,
            ServerStorage,
            ServiceProvider,
            ServiceVisibilityService,
            SessionCheckService,
            SessionService,
            SharedTableRegistry,
            Shirt,
            ShirtGraphic,
            SkateboardController,
            SkateboardPlatform,
            Skin,
            Sky,
            SlidingBallConstraint,
            SlimContentProvider,
            SlimService,
            Smoke,
            SmoothVoxelsUpgraderService,
            Snap,
            SnippetService,
            SocialService,
            SolidModelContentProvider,
            Sound,
            SoundEffect,
            SoundGroup,
            SoundService,
            SoundShimService,
            Sparkles,
            SpawnLocation,
            SpawnerService,
            SpecialMesh,
            SphereHandleAdornment,
            SpotLight,
            SpringConstraint,
            StackFrame,
            StandalonePluginScripts,
            StandardPages,
            StartPageService,
            StarterCharacterScripts,
            StarterGear,
            StarterGui,
            StarterPack,
            StarterPlayer,
            StarterPlayerScripts,
            StartupMessageService,
            Stats,
            StatsItem,
            Status,
            StopWatchReporter,
            StreamingService,
            StringValue,
            Studio,
            StudioAssetService,
            StudioAttachment,
            StudioCallout,
            StudioCameraService,
            StudioData,
            StudioDeviceEmulatorService,
            StudioObjectBase,
            StudioPublishService,
            StudioScriptDebugEventListener,
            StudioSdkService,
            StudioService,
            StudioTestService,
            StudioTheme,
            StudioUserService,
            StudioWidget,
            StudioWidgetsService,
            StyleBase,
            StyleDerive,
            StyleLink,
            StyleQuery,
            StyleRule,
            StyleSheet,
            StylingService,
            SunRaysEffect,
            SurfaceAppearance,
            SurfaceGui,
            SurfaceGuiBase,
            SurfaceLight,
            SurfaceSelection,
            SwimController,
            SyncScriptBuilder,
            SystemThemeService,
            TaskScheduler,
            Team,
            TeamCreateData,
            TeamCreatePublishService,
            TeamCreateService,
            Teams,
            TelemetryService,
            TeleportAsyncResult,
            TeleportOptions,
            TeleportService,
            TemporaryCageMeshProvider,
            TemporaryScriptService,
            Terrain,
            TerrainDetail,
            TerrainIterateOperation,
            TerrainModifyOperation,
            TerrainReadOperation,
            TerrainRegion,
            TerrainWriteOperation,
            TestService,
            TextBox,
            TextBoxService,
            TextButton,
            TextChannel,
            TextChatCommand,
            TextChatConfigurations,
            TextChatMessage,
            TextChatMessageProperties,
            TextChatService,
            TextFilterResult,
            TextFilterTranslatedResult,
            TextGenerator,
            TextLabel,
            TextService,
            TextSource,
            Texture,
            TextureGenerationPartGroup,
            TextureGenerationService,
            TextureGenerationUnwrappingRequest,
            ThirdPartyUserService,
            ThreadState,
            TimerService,
            ToastNotificationService,
            Tool,
            Torque,
            TorsionSpringConstraint,
            TotalCountTimeIntervalItem,
            TouchInputService,
            TouchTransmitter,
            TracerService,
            TrackerLodController,
            TrackerStreamAnimation,
            Trail,
            Translator,
            TremoloSoundEffect,
            TriangleMeshPart,
            TrussPart,
            TutorialService,
            Tween,
            TweenBase,
            TweenService,
            UGCAvatarService,
            UGCValidationService,
            UIAspectRatioConstraint,
            UIBase,
            UIComponent,
            UIConstraint,
            UICorner,
            UIDragDetector,
            UIDragDetectorService,
            UIFlexItem,
            UIGradient,
            UIGridLayout,
            UIGridStyleLayout,
            UILayout,
            UIListLayout,
            UIPadding,
            UIPageLayout,
            UIScale,
            UISizeConstraint,
            UIStroke,
            UITableLayout,
            UITextSizeConstraint,
            UnionOperation,
            UniqueIdLookupService,
            UniversalConstraint,
            UnreliableRemoteEvent,
            UnvalidatedAssetService,
            UserGameSettings,
            UserInputService,
            UserService,
            UserSettings,
            UserStorageService,
            VRService,
            VRStatusService,
            ValueBase,
            ValueCurve,
            Vector3Curve,
            Vector3Value,
            VectorForce,
            VehicleController,
            VehicleSeat,
            VelocityMotor,
            VersionControlService,
            VideoCapture,
            VideoCaptureService,
            VideoDeviceInput,
            VideoDisplay,
            VideoFrame,
            VideoPlayer,
            VideoSampler,
            VideoScreenCaptureService,
            VideoService,
            ViewportFrame,
            VirtualInputManager,
            VirtualUser,
            VisibilityCheckDispatcher,
            Visit,
            VisualizationMode,
            VisualizationModeCategory,
            VisualizationModeService,
            VoiceChatInternal,
            VoiceChatService,
            WebSocketClient,
            WebSocketService,
            WebStreamClient,
            WebViewService,
            WedgePart,
            Weld,
            WeldConstraint,
            Wire,
            WireframeHandleAdornment,
            Workspace,
            WorkspaceAnnotation,
            WorldModel,
            WorldRoot,
            WrapDeformer,
            WrapLayer,
            WrapTarget,
            WrapTextureTransfer
        );
    };
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Accessory<I> {
    superclass: Accoutrement<I>,
    pub AccessoryType: enums::AccessoryType,
}
impl_inherits!(Accessory<I>, Accoutrement<I>);
impl<I: Default> Default for Accessory<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AccessoryDescription<I> {
    superclass: Instance<I>,
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
impl_inherits!(AccessoryDescription<I>, Instance<I>);
impl<I: Default> Default for AccessoryDescription<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AccountService<I> {
    superclass: Instance<I>,
}
impl_inherits!(AccountService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Accoutrement<I> {
    superclass: Instance<I>,
    pub AttachmentPoint: CFrame,
}
impl_inherits!(Accoutrement<I>, Instance<I>);
impl<I: Default> Default for Accoutrement<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AchievementService<I> {
    superclass: Instance<I>,
}
impl_inherits!(AchievementService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ActivityHistoryEventService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ActivityHistoryEventService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Actor<I> {
    superclass: Model<I>,
}
impl_inherits!(Actor<I>, Model<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AdGui<I> {
    superclass: SurfaceGuiBase<I>,
    pub AdShape: enums::AdShape,
    pub EnableVideoAds: bool,
    pub FallbackImage: ContentId,
}
impl_inherits!(AdGui<I>, SurfaceGuiBase<I>);
impl<I: Default> Default for AdGui<I> {
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
pub struct AdPortal<I> {
    superclass: Instance<I>,
}
impl_inherits!(AdPortal<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AdService<I> {
    superclass: Instance<I>,
}
impl_inherits!(AdService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AdvancedDragger<I> {
    superclass: Instance<I>,
}
impl_inherits!(AdvancedDragger<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AirController<I> {
    superclass: ControllerBase<I>,
    pub BalanceMaxTorque: f32,
    pub BalanceSpeed: f32,
    pub MaintainAngularMomentum: bool,
    pub MaintainLinearMomentum: bool,
    pub MoveMaxForce: f32,
    pub TurnMaxTorque: f32,
    pub TurnSpeedFactor: f32,
}
impl_inherits!(AirController<I>, ControllerBase<I>);
impl<I: Default> Default for AirController<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AlignOrientation<I> {
    superclass: Constraint<I>,
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
impl_inherits!(AlignOrientation<I>, Constraint<I>);
impl<I: Default> Default for AlignOrientation<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AlignPosition<I> {
    superclass: Constraint<I>,
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
impl_inherits!(AlignPosition<I>, Constraint<I>);
impl<I: Default> Default for AlignPosition<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AnalyticsService<I> {
    superclass: Instance<I>,
    pub ApiKey: String,
}
impl_inherits!(AnalyticsService<I>, Instance<I>);
impl<I: Default> Default for AnalyticsService<I> {
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
pub struct AngularVelocity<I> {
    superclass: Constraint<I>,
    pub AngularVelocity: Vector3,
    pub MaxTorque: f32,
    pub ReactionTorqueEnabled: bool,
    pub RelativeTo: enums::ActuatorRelativeTo,
}
impl_inherits!(AngularVelocity<I>, Constraint<I>);
impl<I: Default> Default for AngularVelocity<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Animation<I> {
    superclass: Instance<I>,
    pub AnimationId: ContentId,
}
impl_inherits!(Animation<I>, Instance<I>);
impl<I: Default> Default for Animation<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AnimationClip<I> {
    superclass: Instance<I>,
    pub GuidBinaryString: BinaryString,
    pub Loop: bool,
    pub Priority: enums::AnimationPriority,
}
impl_inherits!(AnimationClip<I>, Instance<I>);
impl<I: Default> Default for AnimationClip<I> {
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
pub struct AnimationClipProvider<I> {
    superclass: Instance<I>,
}
impl_inherits!(AnimationClipProvider<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationConstraint<I> {
    superclass: Constraint<I>,
    pub IsKinematic: bool,
    pub MaxForce: f32,
    pub MaxTorque: f32,
    pub Transform: CFrame,
}
impl_inherits!(AnimationConstraint<I>, Constraint<I>);
impl<I: Default> Default for AnimationConstraint<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AnimationController<I> {
    superclass: Instance<I>,
}
impl_inherits!(AnimationController<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnimationFromVideoCreatorService<I> {
    superclass: Instance<I>,
}
impl_inherits!(AnimationFromVideoCreatorService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnimationFromVideoCreatorStudioService<I> {
    superclass: Instance<I>,
}
impl_inherits!(AnimationFromVideoCreatorStudioService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnimationGraphDefinition<I> {
    superclass: AnimationClip<I>,
}
impl_inherits!(AnimationGraphDefinition<I>, AnimationClip<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnimationImportData<I> {
    superclass: BaseImportData<I>,
}
impl_inherits!(AnimationImportData<I>, BaseImportData<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnimationNode<I> {
    superclass: Object<I>,
}
impl_inherits!(AnimationNode<I>, Object<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationNodeDefinition<I> {
    superclass: Instance<I>,
    pub InputPinData: BinaryString,
    pub NodeType: enums::AnimationNodeType,
}
impl_inherits!(AnimationNodeDefinition<I>, Instance<I>);
impl<I: Default> Default for AnimationNodeDefinition<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AnimationRigData<I> {
    superclass: Instance<I>,
    pub Label: BinaryString,
    pub Name: BinaryString,
    pub Parent: BinaryString,
    pub PostTransform: BinaryString,
    pub PreTransform: BinaryString,
    pub Transform: BinaryString,
}
impl_inherits!(AnimationRigData<I>, Instance<I>);
impl<I: Default> Default for AnimationRigData<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AnimationStreamTrack<I> {
    superclass: Instance<I>,
}
impl_inherits!(AnimationStreamTrack<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationTrack<I> {
    superclass: Instance<I>,
    pub Priority: enums::AnimationPriority,
}
impl_inherits!(AnimationTrack<I>, Instance<I>);
impl<I: Default> Default for AnimationTrack<I> {
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
pub struct Animator<I> {
    superclass: Instance<I>,
    pub PreferLodEnabled: bool,
}
impl_inherits!(Animator<I>, Instance<I>);
impl<I: Default> Default for Animator<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Annotation<I> {
    superclass: Instance<I>,
}
impl_inherits!(Annotation<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnnotationsService<I> {
    superclass: Instance<I>,
}
impl_inherits!(AnnotationsService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AppLifecycleObserverService<I> {
    superclass: Instance<I>,
}
impl_inherits!(AppLifecycleObserverService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AppRatingPromptService<I> {
    superclass: Instance<I>,
}
impl_inherits!(AppRatingPromptService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AppStorageService<I> {
    superclass: LocalStorageService<I>,
}
impl_inherits!(AppStorageService<I>, LocalStorageService<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AppUpdateService<I> {
    superclass: Instance<I>,
}
impl_inherits!(AppUpdateService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ArcHandles<I> {
    superclass: HandlesBase<I>,
    pub Axes: Axes,
}
impl_inherits!(ArcHandles<I>, HandlesBase<I>);
impl<I: Default> Default for ArcHandles<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AssetCounterService<I> {
    superclass: Instance<I>,
}
impl_inherits!(AssetCounterService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetDeliveryProxy<I> {
    superclass: Instance<I>,
    pub Interface: String,
    pub Port: i32,
    pub StartServer: bool,
}
impl_inherits!(AssetDeliveryProxy<I>, Instance<I>);
impl<I: Default> Default for AssetDeliveryProxy<I> {
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
pub struct AssetImportService<I> {
    superclass: Instance<I>,
}
impl_inherits!(AssetImportService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AssetImportSession<I> {
    superclass: ImportSession<I>,
}
impl_inherits!(AssetImportSession<I>, ImportSession<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AssetManagerService<I> {
    superclass: Instance<I>,
}
impl_inherits!(AssetManagerService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetPatchSettings<I> {
    superclass: Instance<I>,
    pub ContentId: String,
    pub OutputPath: String,
    pub PatchId: String,
}
impl_inherits!(AssetPatchSettings<I>, Instance<I>);
impl<I: Default> Default for AssetPatchSettings<I> {
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
pub struct AssetService<I> {
    superclass: Instance<I>,
    pub AllowInsertFreeAssets: bool,
}
impl_inherits!(AssetService<I>, Instance<I>);
impl<I: Default> Default for AssetService<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AssetSoundEffect<I> {
    superclass: CustomSoundEffect<I>,
}
impl_inherits!(AssetSoundEffect<I>, CustomSoundEffect<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Atmosphere<I> {
    superclass: Instance<I>,
    pub Color: Color3,
    pub Decay: Color3,
    pub Density: f32,
    pub Glare: f32,
    pub Haze: f32,
    pub Offset: f32,
}
impl_inherits!(Atmosphere<I>, Instance<I>);
impl<I: Default> Default for Atmosphere<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AtmosphereSensor<I> {
    superclass: SensorBase<I>,
}
impl_inherits!(AtmosphereSensor<I>, SensorBase<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Attachment<I> {
    superclass: Instance<I>,
    pub CFrame: CFrame,
    pub Visible: bool,
}
impl_inherits!(Attachment<I>, Instance<I>);
impl<I: Default> Default for Attachment<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioAnalyzer<I> {
    superclass: Instance<I>,
    pub SpectrumEnabled: bool,
    pub WindowSize: enums::AudioWindowSize,
}
impl_inherits!(AudioAnalyzer<I>, Instance<I>);
impl<I: Default> Default for AudioAnalyzer<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioChannelMixer<I> {
    superclass: Instance<I>,
    pub Layout: enums::AudioChannelLayout,
}
impl_inherits!(AudioChannelMixer<I>, Instance<I>);
impl<I: Default> Default for AudioChannelMixer<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioChannelSplitter<I> {
    superclass: Instance<I>,
    pub Layout: enums::AudioChannelLayout,
}
impl_inherits!(AudioChannelSplitter<I>, Instance<I>);
impl<I: Default> Default for AudioChannelSplitter<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioChorus<I> {
    superclass: Instance<I>,
    pub Bypass: bool,
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
impl_inherits!(AudioChorus<I>, Instance<I>);
impl<I: Default> Default for AudioChorus<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioCompressor<I> {
    superclass: Instance<I>,
    pub Attack: f32,
    pub Bypass: bool,
    pub MakeupGain: f32,
    pub Ratio: f32,
    pub Release: f32,
    pub Threshold: f32,
}
impl_inherits!(AudioCompressor<I>, Instance<I>);
impl<I: Default> Default for AudioCompressor<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioDeviceInput<I> {
    superclass: Instance<I>,
    pub AccessType: enums::AccessModifierType,
    pub Active: bool,
    pub Muted: bool,
    pub Player: Ref,
    pub Volume: f32,
}
impl_inherits!(AudioDeviceInput<I>, Instance<I>);
impl<I: Default> Default for AudioDeviceInput<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioDeviceOutput<I> {
    superclass: Instance<I>,
    pub Player: Ref,
}
impl_inherits!(AudioDeviceOutput<I>, Instance<I>);
impl<I: Default> Default for AudioDeviceOutput<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioDistortion<I> {
    superclass: Instance<I>,
    pub Bypass: bool,
    pub Level: f32,
}
impl_inherits!(AudioDistortion<I>, Instance<I>);
impl<I: Default> Default for AudioDistortion<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioEcho<I> {
    superclass: Instance<I>,
    pub Bypass: bool,
    pub DelayTime: f32,
    pub DryLevel: f32,
    pub Feedback: f32,
    pub RampTime: f32,
    pub WetLevel: f32,
}
impl_inherits!(AudioEcho<I>, Instance<I>);
impl<I: Default> Default for AudioEcho<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioEmitter<I> {
    superclass: Instance<I>,
    pub AcousticSimulationEnabled: bool,
    pub AngleAttenuation: BinaryString,
    pub AudioInteractionGroup: String,
    pub DistanceAttenuation: BinaryString,
    pub PositionOverride: Ref,
    pub SimulationFidelity: enums::AudioSimulationFidelity,
}
impl_inherits!(AudioEmitter<I>, Instance<I>);
impl<I: Default> Default for AudioEmitter<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioEqualizer<I> {
    superclass: Instance<I>,
    pub Bypass: bool,
    pub HighGain: f32,
    pub LowGain: f32,
    pub MidGain: f32,
    pub MidRange: NumberRange,
}
impl_inherits!(AudioEqualizer<I>, Instance<I>);
impl<I: Default> Default for AudioEqualizer<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioFader<I> {
    superclass: Instance<I>,
    pub Bypass: bool,
    pub Volume: f32,
}
impl_inherits!(AudioFader<I>, Instance<I>);
impl<I: Default> Default for AudioFader<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioFilter<I> {
    superclass: Instance<I>,
    pub Bypass: bool,
    pub FilterType: enums::AudioFilterType,
    pub Frequency: f32,
    pub Gain: f32,
    pub Q: f32,
}
impl_inherits!(AudioFilter<I>, Instance<I>);
impl<I: Default> Default for AudioFilter<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioFlanger<I> {
    superclass: Instance<I>,
    pub Bypass: bool,
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
impl_inherits!(AudioFlanger<I>, Instance<I>);
impl<I: Default> Default for AudioFlanger<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioFocusService<I> {
    superclass: Instance<I>,
}
impl_inherits!(AudioFocusService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioGate<I> {
    superclass: Instance<I>,
    pub Attack: f32,
    pub Bypass: bool,
    pub Release: f32,
    pub Threshold: NumberRange,
}
impl_inherits!(AudioGate<I>, Instance<I>);
impl<I: Default> Default for AudioGate<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioLimiter<I> {
    superclass: Instance<I>,
    pub Bypass: bool,
    pub MaxLevel: f32,
    pub Release: f32,
}
impl_inherits!(AudioLimiter<I>, Instance<I>);
impl<I: Default> Default for AudioLimiter<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioListener<I> {
    superclass: Instance<I>,
    pub AcousticSimulationEnabled: bool,
    pub AngleAttenuation: BinaryString,
    pub AudioInteractionGroup: String,
    pub DistanceAttenuation: BinaryString,
    pub PositionOverride: Ref,
    pub SimulationFidelity: enums::AudioSimulationFidelity,
}
impl_inherits!(AudioListener<I>, Instance<I>);
impl<I: Default> Default for AudioListener<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioPages<I> {
    superclass: Pages<I>,
}
impl_inherits!(AudioPages<I>, Pages<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioPitchShifter<I> {
    superclass: Instance<I>,
    pub Bypass: bool,
    pub Pitch: f32,
    pub WindowSize: enums::AudioWindowSize,
}
impl_inherits!(AudioPitchShifter<I>, Instance<I>);
impl<I: Default> Default for AudioPitchShifter<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioPlayer<I> {
    superclass: Instance<I>,
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
impl_inherits!(AudioPlayer<I>, Instance<I>);
impl<I: Default> Default for AudioPlayer<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioRecorder<I> {
    superclass: Instance<I>,
    pub IsRecording: bool,
}
impl_inherits!(AudioRecorder<I>, Instance<I>);
impl<I: Default> Default for AudioRecorder<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioReverb<I> {
    superclass: Instance<I>,
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
impl_inherits!(AudioReverb<I>, Instance<I>);
impl<I: Default> Default for AudioReverb<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioSearchParams<I> {
    superclass: Instance<I>,
    pub Album: String,
    pub Artist: String,
    pub AudioSubType: enums::AudioSubType,
    pub MaxDuration: i32,
    pub MinDuration: i32,
    pub SearchKeyword: String,
    pub Tag: String,
    pub Title: String,
}
impl_inherits!(AudioSearchParams<I>, Instance<I>);
impl<I: Default> Default for AudioSearchParams<I> {
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
pub struct AudioSpeechToText<I> {
    superclass: Instance<I>,
    pub Enabled: bool,
    pub Text: String,
}
impl_inherits!(AudioSpeechToText<I>, Instance<I>);
impl<I: Default> Default for AudioSpeechToText<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioTextToSpeech<I> {
    superclass: Instance<I>,
    pub Looping: bool,
    pub Pitch: f32,
    pub PlaybackSpeed: f32,
    pub Speed: f32,
    pub Text: String,
    pub TimePosition: f64,
    pub VoiceId: String,
    pub Volume: f32,
}
impl_inherits!(AudioTextToSpeech<I>, Instance<I>);
impl<I: Default> Default for AudioTextToSpeech<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AudioTremolo<I> {
    superclass: Instance<I>,
    pub Bypass: bool,
    pub Depth: f32,
    pub Duty: f32,
    pub Frequency: f32,
    pub Shape: f32,
    pub Skew: f32,
    pub Square: f32,
}
impl_inherits!(AudioTremolo<I>, Instance<I>);
impl<I: Default> Default for AudioTremolo<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AuroraScript<I> {
    superclass: LuaSourceContainer<I>,
    pub AuroraScriptBindingsSerialize: BinaryString,
    pub EnableCulling: bool,
    pub EnableLod: bool,
    pub LodCriticality: i32,
    pub Priority: i32,
    pub Source: String,
}
impl_inherits!(AuroraScript<I>, LuaSourceContainer<I>);
impl<I: Default> Default for AuroraScript<I> {
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
pub struct AuroraScriptObject<I> {
    superclass: Instance<I>,
    pub BehaviorWeak: Ref,
    pub BoundInstanceWeak: Ref,
    pub FrameId: i32,
    pub LodLevel: i32,
    pub PriorFrameInvoked: i32,
}
impl_inherits!(AuroraScriptObject<I>, Instance<I>);
impl<I: Default> Default for AuroraScriptObject<I> {
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
pub struct AuroraScriptService<I> {
    superclass: Instance<I>,
}
impl_inherits!(AuroraScriptService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AuroraService<I> {
    superclass: Instance<I>,
    pub HashRoundingPoint: f64,
    pub IgnoreRotation: bool,
    pub LockStepIdOffset: bool,
    pub RollbackOffset: i32,
}
impl_inherits!(AuroraService<I>, Instance<I>);
impl<I: Default> Default for AuroraService<I> {
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
pub struct AvatarAccessoryRules<I> {
    superclass: Instance<I>,
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
impl_inherits!(AvatarAccessoryRules<I>, Instance<I>);
impl<I: Default> Default for AvatarAccessoryRules<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AvatarAnimationRules<I> {
    superclass: Instance<I>,
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
impl_inherits!(AvatarAnimationRules<I>, Instance<I>);
impl<I: Default> Default for AvatarAnimationRules<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AvatarBodyRules<I> {
    superclass: Instance<I>,
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
impl_inherits!(AvatarBodyRules<I>, Instance<I>);
impl<I: Default> Default for AvatarBodyRules<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AvatarChatService<I> {
    superclass: Instance<I>,
}
impl_inherits!(AvatarChatService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarClothingRules<I> {
    superclass: Instance<I>,
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
impl_inherits!(AvatarClothingRules<I>, Instance<I>);
impl<I: Default> Default for AvatarClothingRules<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AvatarCollisionRules<I> {
    superclass: Instance<I>,
    pub CollisionMode: enums::AvatarSettingsCollisionMode,
    pub HitAndTouchDetectionMode: enums::AvatarSettingsHitAndTouchDetectionMode,
    pub LegacyCollisionMode: enums::AvatarSettingsLegacyCollisionMode,
    pub SingleColliderSize: Vector3,
}
impl_inherits!(AvatarCollisionRules<I>, Instance<I>);
impl<I: Default> Default for AvatarCollisionRules<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AvatarCreationService<I> {
    superclass: Instance<I>,
}
impl_inherits!(AvatarCreationService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AvatarEditorService<I> {
    superclass: Instance<I>,
}
impl_inherits!(AvatarEditorService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AvatarImportService<I> {
    superclass: Instance<I>,
}
impl_inherits!(AvatarImportService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarRules<I> {
    superclass: Instance<I>,
    pub AvatarType: enums::GameAvatarType,
}
impl_inherits!(AvatarRules<I>, Instance<I>);
impl<I: Default> Default for AvatarRules<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct AvatarSettings<I> {
    superclass: Instance<I>,
}
impl_inherits!(AvatarSettings<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Backpack<I> {
    superclass: Instance<I>,
}
impl_inherits!(Backpack<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BackpackItem<I> {
    superclass: Model<I>,
    pub TextureContent: Content,
}
impl_inherits!(BackpackItem<I>, Model<I>);
impl<I: Default> Default for BackpackItem<I> {
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
pub struct BadgeService<I> {
    superclass: Instance<I>,
}
impl_inherits!(BadgeService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BallSocketConstraint<I> {
    superclass: Constraint<I>,
    pub LimitsEnabled: bool,
    pub MaxFrictionTorqueXml: f32,
    pub Radius: f32,
    pub Restitution: f32,
    pub TwistLimitsEnabled: bool,
    pub TwistLowerAngle: f32,
    pub TwistUpperAngle: f32,
    pub UpperAngle: f32,
}
impl_inherits!(BallSocketConstraint<I>, Constraint<I>);
impl<I: Default> Default for BallSocketConstraint<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct BanHistoryPages<I> {
    superclass: Pages<I>,
}
impl_inherits!(BanHistoryPages<I>, Pages<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BaseImportData<I> {
    superclass: Instance<I>,
    pub ImportName: String,
    pub ShouldImport: bool,
}
impl_inherits!(BaseImportData<I>, Instance<I>);
impl<I: Default> Default for BaseImportData<I> {
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
pub struct BasePart<I> {
    superclass: PVInstance<I>,
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
impl_inherits!(BasePart<I>, PVInstance<I>);
impl<I: Default> Default for BasePart<I> {
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
pub struct BasePlayerGui<I> {
    superclass: Instance<I>,
}
impl_inherits!(BasePlayerGui<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BaseRemoteEvent<I> {
    superclass: Instance<I>,
}
impl_inherits!(BaseRemoteEvent<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BaseScript<I> {
    superclass: LuaSourceContainer<I>,
    pub Disabled: bool,
    pub LinkedSource: ContentId,
    pub RunContext: enums::RunContext,
}
impl_inherits!(BaseScript<I>, LuaSourceContainer<I>);
impl<I: Default> Default for BaseScript<I> {
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
pub struct BaseWrap<I> {
    superclass: Instance<I>,
    pub CageMeshContent: Content,
    pub CageOrigin: CFrame,
    pub HsrAssetId: ContentId,
    pub HsrData: SharedString,
    pub HsrMeshIdData: SharedString,
    pub ImportOrigin: CFrame,
    pub TemporaryCageMeshId: ContentId,
}
impl_inherits!(BaseWrap<I>, Instance<I>);
impl<I: Default> Default for BaseWrap<I> {
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
pub struct Beam<I> {
    superclass: Instance<I>,
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
impl_inherits!(Beam<I>, Instance<I>);
impl<I: Default> Default for Beam<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct BevelMesh<I> {
    superclass: DataModelMesh<I>,
    pub Bevel: f32,
    pub BevelRoundness: f32,
    pub Bulge: f32,
}
impl_inherits!(BevelMesh<I>, DataModelMesh<I>);
impl<I: Default> Default for BevelMesh<I> {
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
pub struct BillboardGui<I> {
    superclass: LayerCollector<I>,
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
impl_inherits!(BillboardGui<I>, LayerCollector<I>);
impl<I: Default> Default for BillboardGui<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct BinaryStringValue<I> {
    superclass: ValueBase<I>,
    pub Value: BinaryString,
}
impl_inherits!(BinaryStringValue<I>, ValueBase<I>);
impl<I: Default> Default for BinaryStringValue<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct BindableEvent<I> {
    superclass: Instance<I>,
}
impl_inherits!(BindableEvent<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BindableFunction<I> {
    superclass: Instance<I>,
}
impl_inherits!(BindableFunction<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BlockMesh<I> {
    superclass: BevelMesh<I>,
}
impl_inherits!(BlockMesh<I>, BevelMesh<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BloomEffect<I> {
    superclass: PostEffect<I>,
    pub Intensity: f32,
    pub Size: f32,
    pub Threshold: f32,
}
impl_inherits!(BloomEffect<I>, PostEffect<I>);
impl<I: Default> Default for BloomEffect<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct BlurEffect<I> {
    superclass: PostEffect<I>,
    pub Size: f32,
}
impl_inherits!(BlurEffect<I>, PostEffect<I>);
impl<I: Default> Default for BlurEffect<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct BodyAngularVelocity<I> {
    superclass: BodyMover<I>,
    pub AngularVelocity: Vector3,
    pub MaxTorque: Vector3,
    pub P: f32,
}
impl_inherits!(BodyAngularVelocity<I>, BodyMover<I>);
impl<I: Default> Default for BodyAngularVelocity<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct BodyColors<I> {
    superclass: CharacterAppearance<I>,
    pub HeadColor3: Color3,
    pub LeftArmColor3: Color3,
    pub LeftLegColor3: Color3,
    pub RightArmColor3: Color3,
    pub RightLegColor3: Color3,
    pub TorsoColor3: Color3,
}
impl_inherits!(BodyColors<I>, CharacterAppearance<I>);
impl<I: Default> Default for BodyColors<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct BodyForce<I> {
    superclass: BodyMover<I>,
    pub Force: Vector3,
}
impl_inherits!(BodyForce<I>, BodyMover<I>);
impl<I: Default> Default for BodyForce<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct BodyGyro<I> {
    superclass: BodyMover<I>,
    pub CFrame: CFrame,
    pub D: f32,
    pub MaxTorque: Vector3,
    pub P: f32,
}
impl_inherits!(BodyGyro<I>, BodyMover<I>);
impl<I: Default> Default for BodyGyro<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct BodyMover<I> {
    superclass: Instance<I>,
}
impl_inherits!(BodyMover<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyPartDescription<I> {
    superclass: Instance<I>,
    pub AssetId: i64,
    pub BodyPart: enums::BodyPart,
    pub Color: Color3,
    pub HeadShape: String,
    pub Instance: Ref,
}
impl_inherits!(BodyPartDescription<I>, Instance<I>);
impl<I: Default> Default for BodyPartDescription<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct BodyPosition<I> {
    superclass: BodyMover<I>,
    pub D: f32,
    pub MaxForce: Vector3,
    pub P: f32,
    pub Position: Vector3,
}
impl_inherits!(BodyPosition<I>, BodyMover<I>);
impl<I: Default> Default for BodyPosition<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct BodyThrust<I> {
    superclass: BodyMover<I>,
    pub Force: Vector3,
    pub Location: Vector3,
}
impl_inherits!(BodyThrust<I>, BodyMover<I>);
impl<I: Default> Default for BodyThrust<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct BodyVelocity<I> {
    superclass: BodyMover<I>,
    pub MaxForce: Vector3,
    pub P: f32,
    pub Velocity: Vector3,
}
impl_inherits!(BodyVelocity<I>, BodyMover<I>);
impl<I: Default> Default for BodyVelocity<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Bone<I> {
    superclass: Attachment<I>,
}
impl_inherits!(Bone<I>, Attachment<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BoolValue<I> {
    superclass: ValueBase<I>,
    pub Value: bool,
}
impl_inherits!(BoolValue<I>, ValueBase<I>);
impl<I: Default> Default for BoolValue<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct BoxHandleAdornment<I> {
    superclass: HandleAdornment<I>,
    pub Shading: enums::AdornShading,
    pub Size: Vector3,
}
impl_inherits!(BoxHandleAdornment<I>, HandleAdornment<I>);
impl<I: Default> Default for BoxHandleAdornment<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Breakpoint<I> {
    superclass: Instance<I>,
}
impl_inherits!(Breakpoint<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BrickColorValue<I> {
    superclass: ValueBase<I>,
    pub Value: BrickColor,
}
impl_inherits!(BrickColorValue<I>, ValueBase<I>);
impl<I: Default> Default for BrickColorValue<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct BrowserService<I> {
    superclass: Instance<I>,
}
impl_inherits!(BrowserService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BubbleChatConfiguration<I> {
    superclass: TextChatConfigurations<I>,
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
impl_inherits!(BubbleChatConfiguration<I>, TextChatConfigurations<I>);
impl<I: Default> Default for BubbleChatConfiguration<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct BubbleChatMessageProperties<I> {
    superclass: TextChatMessageProperties<I>,
}
impl_inherits!(BubbleChatMessageProperties<I>, TextChatMessageProperties<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BugReporterService<I> {
    superclass: Instance<I>,
}
impl_inherits!(BugReporterService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BulkImportService<I> {
    superclass: Instance<I>,
}
impl_inherits!(BulkImportService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BuoyancySensor<I> {
    superclass: SensorBase<I>,
    pub FullySubmerged: bool,
    pub TouchingSurface: bool,
}
impl_inherits!(BuoyancySensor<I>, SensorBase<I>);
impl<I: Default> Default for BuoyancySensor<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct CFrameValue<I> {
    superclass: ValueBase<I>,
    pub Value: CFrame,
}
impl_inherits!(CFrameValue<I>, ValueBase<I>);
impl<I: Default> Default for CFrameValue<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct CSGDictionaryService<I> {
    superclass: FlyweightService<I>,
}
impl_inherits!(CSGDictionaryService<I>, FlyweightService<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CacheableContentProvider<I> {
    superclass: Instance<I>,
}
impl_inherits!(CacheableContentProvider<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CalloutService<I> {
    superclass: Instance<I>,
}
impl_inherits!(CalloutService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Camera<I> {
    superclass: PVInstance<I>,
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
impl_inherits!(Camera<I>, PVInstance<I>);
impl<I: Default> Default for Camera<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct CanvasGroup<I> {
    superclass: GuiObject<I>,
    pub GroupColor3: Color3,
    pub GroupTransparency: f32,
}
impl_inherits!(CanvasGroup<I>, GuiObject<I>);
impl<I: Default> Default for CanvasGroup<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Capture<I> {
    superclass: Object<I>,
}
impl_inherits!(Capture<I>, Object<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CaptureService<I> {
    superclass: Instance<I>,
}
impl_inherits!(CaptureService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CapturesPages<I> {
    superclass: Pages<I>,
}
impl_inherits!(CapturesPages<I>, Pages<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CatalogPages<I> {
    superclass: Pages<I>,
}
impl_inherits!(CatalogPages<I>, Pages<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ChangeHistoryService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ChangeHistoryService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ChangeHistoryStreamingService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ChangeHistoryStreamingService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChannelSelectorSoundEffect<I> {
    superclass: CustomSoundEffect<I>,
    pub Channel: i32,
}
impl_inherits!(ChannelSelectorSoundEffect<I>, CustomSoundEffect<I>);
impl<I: Default> Default for ChannelSelectorSoundEffect<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ChannelTabsConfiguration<I> {
    superclass: TextChatConfigurations<I>,
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
impl_inherits!(ChannelTabsConfiguration<I>, TextChatConfigurations<I>);
impl<I: Default> Default for ChannelTabsConfiguration<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct CharacterAppearance<I> {
    superclass: Instance<I>,
}
impl_inherits!(CharacterAppearance<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CharacterMesh<I> {
    superclass: CharacterAppearance<I>,
    pub BaseTextureId: i64,
    pub BodyPart: enums::BodyPart,
    pub MeshId: i64,
    pub OverlayTextureId: i64,
}
impl_inherits!(CharacterMesh<I>, CharacterAppearance<I>);
impl<I: Default> Default for CharacterMesh<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Chat<I> {
    superclass: Instance<I>,
    pub BubbleChatEnabled: bool,
    pub IsAutoMigrated: bool,
    pub LoadDefaultChat: bool,
}
impl_inherits!(Chat<I>, Instance<I>);
impl<I: Default> Default for Chat<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ChatInputBarConfiguration<I> {
    superclass: TextChatConfigurations<I>,
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
impl_inherits!(ChatInputBarConfiguration<I>, TextChatConfigurations<I>);
impl<I: Default> Default for ChatInputBarConfiguration<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ChatWindowConfiguration<I> {
    superclass: TextChatConfigurations<I>,
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
impl_inherits!(ChatWindowConfiguration<I>, TextChatConfigurations<I>);
impl<I: Default> Default for ChatWindowConfiguration<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ChatWindowMessageProperties<I> {
    superclass: TextChatMessageProperties<I>,
}
impl_inherits!(ChatWindowMessageProperties<I>, TextChatMessageProperties<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ChatbotUIService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ChatbotUIService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChorusSoundEffect<I> {
    superclass: SoundEffect<I>,
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
impl_inherits!(ChorusSoundEffect<I>, SoundEffect<I>);
impl<I: Default> Default for ChorusSoundEffect<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ClickDetector<I> {
    superclass: Instance<I>,
    pub CursorIconContent: Content,
    pub MaxActivationDistance: f32,
}
impl_inherits!(ClickDetector<I>, Instance<I>);
impl<I: Default> Default for ClickDetector<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ClientReplicator<I> {
    superclass: NetworkReplicator<I>,
}
impl_inherits!(ClientReplicator<I>, NetworkReplicator<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ClimbController<I> {
    superclass: ControllerBase<I>,
    pub AccelerationTime: f32,
    pub BalanceMaxTorque: f32,
    pub BalanceSpeed: f32,
    pub MoveMaxForce: f32,
}
impl_inherits!(ClimbController<I>, ControllerBase<I>);
impl<I: Default> Default for ClimbController<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Clothing<I> {
    superclass: CharacterAppearance<I>,
    pub Color3: Color3,
}
impl_inherits!(Clothing<I>, CharacterAppearance<I>);
impl<I: Default> Default for Clothing<I> {
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
pub struct CloudCRUDService<I> {
    superclass: Instance<I>,
}
impl_inherits!(CloudCRUDService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CloudLocalizationTable<I> {
    superclass: LocalizationTable<I>,
}
impl_inherits!(CloudLocalizationTable<I>, LocalizationTable<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Clouds<I> {
    superclass: Instance<I>,
    pub Color: Color3,
    pub Cover: f32,
    pub Density: f32,
    pub Enabled: bool,
}
impl_inherits!(Clouds<I>, Instance<I>);
impl<I: Default> Default for Clouds<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ClusterPacketCache<I> {
    superclass: Instance<I>,
}
impl_inherits!(ClusterPacketCache<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Collaborator<I> {
    superclass: Instance<I>,
}
impl_inherits!(Collaborator<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CollaboratorsService<I> {
    superclass: Instance<I>,
}
impl_inherits!(CollaboratorsService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CollectionService<I> {
    superclass: Instance<I>,
}
impl_inherits!(CollectionService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Color3Value<I> {
    superclass: ValueBase<I>,
    pub Value: Color3,
}
impl_inherits!(Color3Value<I>, ValueBase<I>);
impl<I: Default> Default for Color3Value<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ColorCorrectionEffect<I> {
    superclass: PostEffect<I>,
    pub Brightness: f32,
    pub Contrast: f32,
    pub Saturation: f32,
    pub TintColor: Color3,
}
impl_inherits!(ColorCorrectionEffect<I>, PostEffect<I>);
impl<I: Default> Default for ColorCorrectionEffect<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ColorGradingEffect<I> {
    superclass: PostEffect<I>,
    pub TonemapperPreset: enums::TonemapperPreset,
}
impl_inherits!(ColorGradingEffect<I>, PostEffect<I>);
impl<I: Default> Default for ColorGradingEffect<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct CommerceService<I> {
    superclass: Instance<I>,
}
impl_inherits!(CommerceService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CompressorSoundEffect<I> {
    superclass: SoundEffect<I>,
    pub Attack: f32,
    pub GainMakeup: f32,
    pub Ratio: f32,
    pub Release: f32,
    pub SideChain: Ref,
    pub Threshold: f32,
}
impl_inherits!(CompressorSoundEffect<I>, SoundEffect<I>);
impl<I: Default> Default for CompressorSoundEffect<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ConeHandleAdornment<I> {
    superclass: HandleAdornment<I>,
    pub Height: f32,
    pub Hollow: bool,
    pub Radius: f32,
    pub Shading: enums::AdornShading,
}
impl_inherits!(ConeHandleAdornment<I>, HandleAdornment<I>);
impl<I: Default> Default for ConeHandleAdornment<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ConfigService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ConfigService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ConfigSnapshot<I> {
    superclass: Object<I>,
}
impl_inherits!(ConfigSnapshot<I>, Object<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Configuration<I> {
    superclass: Instance<I>,
}
impl_inherits!(Configuration<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ConfigureServerService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ConfigureServerService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ConnectivityService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ConnectivityService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Constraint<I> {
    superclass: Instance<I>,
    pub Attachment0: Ref,
    pub Attachment1: Ref,
    pub Color: BrickColor,
    pub Enabled: bool,
    pub Visible: bool,
}
impl_inherits!(Constraint<I>, Instance<I>);
impl<I: Default> Default for Constraint<I> {
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
pub struct ContentProvider<I> {
    superclass: Instance<I>,
}
impl_inherits!(ContentProvider<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ContextActionService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ContextActionService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Controller<I> {
    superclass: Instance<I>,
}
impl_inherits!(Controller<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ControllerBase<I> {
    superclass: Instance<I>,
    pub BalanceRigidityEnabled: bool,
    pub MoveSpeedFactor: f32,
}
impl_inherits!(ControllerBase<I>, Instance<I>);
impl<I: Default> Default for ControllerBase<I> {
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
pub struct ControllerManager<I> {
    superclass: Instance<I>,
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
impl_inherits!(ControllerManager<I>, Instance<I>);
impl<I: Default> Default for ControllerManager<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ControllerPartSensor<I> {
    superclass: ControllerSensor<I>,
    pub HitFrame: CFrame,
    pub HitNormal: Vector3,
    pub SearchDistance: f32,
    pub SensedPart: Ref,
    pub SensorMode: enums::SensorMode,
}
impl_inherits!(ControllerPartSensor<I>, ControllerSensor<I>);
impl<I: Default> Default for ControllerPartSensor<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ControllerSensor<I> {
    superclass: SensorBase<I>,
}
impl_inherits!(ControllerSensor<I>, SensorBase<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ControllerService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ControllerService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ConversationalAIAcceptanceService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ConversationalAIAcceptanceService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CookiesService<I> {
    superclass: Instance<I>,
}
impl_inherits!(CookiesService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CoreGui<I> {
    superclass: BasePlayerGui<I>,
    pub SelectionImageObject: Ref,
}
impl_inherits!(CoreGui<I>, BasePlayerGui<I>);
impl<I: Default> Default for CoreGui<I> {
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
pub struct CorePackages<I> {
    superclass: Instance<I>,
}
impl_inherits!(CorePackages<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CoreScript<I> {
    superclass: BaseScript<I>,
}
impl_inherits!(CoreScript<I>, BaseScript<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CoreScriptDebuggingManagerHelper<I> {
    superclass: Instance<I>,
}
impl_inherits!(CoreScriptDebuggingManagerHelper<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CoreScriptSyncService<I> {
    superclass: Instance<I>,
}
impl_inherits!(CoreScriptSyncService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CornerWedgePart<I> {
    superclass: BasePart<I>,
}
impl_inherits!(CornerWedgePart<I>, BasePart<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CreationDBService<I> {
    superclass: Instance<I>,
}
impl_inherits!(CreationDBService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CreatorStoreService<I> {
    superclass: Instance<I>,
}
impl_inherits!(CreatorStoreService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CrossDMScriptChangeListener<I> {
    superclass: Instance<I>,
}
impl_inherits!(CrossDMScriptChangeListener<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CurveAnimation<I> {
    superclass: AnimationClip<I>,
}
impl_inherits!(CurveAnimation<I>, AnimationClip<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CustomEvent<I> {
    superclass: Instance<I>,
    pub PersistedCurrentValue: f32,
}
impl_inherits!(CustomEvent<I>, Instance<I>);
impl<I: Default> Default for CustomEvent<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct CustomEventReceiver<I> {
    superclass: Instance<I>,
    pub Source: Ref,
}
impl_inherits!(CustomEventReceiver<I>, Instance<I>);
impl<I: Default> Default for CustomEventReceiver<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct CustomLog<I> {
    superclass: Instance<I>,
}
impl_inherits!(CustomLog<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CustomSoundEffect<I> {
    superclass: SoundEffect<I>,
}
impl_inherits!(CustomSoundEffect<I>, SoundEffect<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CylinderHandleAdornment<I> {
    superclass: HandleAdornment<I>,
    pub Angle: f32,
    pub Height: f32,
    pub InnerRadius: f32,
    pub Radius: f32,
    pub Shading: enums::AdornShading,
}
impl_inherits!(CylinderHandleAdornment<I>, HandleAdornment<I>);
impl<I: Default> Default for CylinderHandleAdornment<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct CylinderMesh<I> {
    superclass: BevelMesh<I>,
}
impl_inherits!(CylinderMesh<I>, BevelMesh<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CylindricalConstraint<I> {
    superclass: SlidingBallConstraint<I>,
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
impl_inherits!(CylindricalConstraint<I>, SlidingBallConstraint<I>);
impl<I: Default> Default for CylindricalConstraint<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct DataModel<I> {
    superclass: ServiceProvider<I>,
}
impl_inherits!(DataModel<I>, ServiceProvider<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataModelMesh<I> {
    superclass: Instance<I>,
    pub Offset: Vector3,
    pub Scale: Vector3,
    pub VertexColor: Vector3,
}
impl_inherits!(DataModelMesh<I>, Instance<I>);
impl<I: Default> Default for DataModelMesh<I> {
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
pub struct DataModelPatchService<I> {
    superclass: Instance<I>,
}
impl_inherits!(DataModelPatchService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataModelSession<I> {
    superclass: Instance<I>,
}
impl_inherits!(DataModelSession<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStore<I> {
    superclass: GlobalDataStore<I>,
}
impl_inherits!(DataStore<I>, GlobalDataStore<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreGetOptions<I> {
    superclass: Instance<I>,
    pub UseCache: bool,
}
impl_inherits!(DataStoreGetOptions<I>, Instance<I>);
impl<I: Default> Default for DataStoreGetOptions<I> {
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
pub struct DataStoreIncrementOptions<I> {
    superclass: Instance<I>,
}
impl_inherits!(DataStoreIncrementOptions<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreInfo<I> {
    superclass: Instance<I>,
}
impl_inherits!(DataStoreInfo<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreKey<I> {
    superclass: Instance<I>,
}
impl_inherits!(DataStoreKey<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreKeyInfo<I> {
    superclass: Instance<I>,
}
impl_inherits!(DataStoreKeyInfo<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreKeyPages<I> {
    superclass: Pages<I>,
}
impl_inherits!(DataStoreKeyPages<I>, Pages<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreListingPages<I> {
    superclass: Pages<I>,
}
impl_inherits!(DataStoreListingPages<I>, Pages<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreObjectVersionInfo<I> {
    superclass: Instance<I>,
}
impl_inherits!(DataStoreObjectVersionInfo<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreOptions<I> {
    superclass: Instance<I>,
    pub AllScopes: bool,
}
impl_inherits!(DataStoreOptions<I>, Instance<I>);
impl<I: Default> Default for DataStoreOptions<I> {
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
pub struct DataStorePages<I> {
    superclass: Pages<I>,
}
impl_inherits!(DataStorePages<I>, Pages<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreService<I> {
    superclass: Instance<I>,
    pub AutomaticRetry: bool,
    pub LegacyNamingScheme: bool,
}
impl_inherits!(DataStoreService<I>, Instance<I>);
impl<I: Default> Default for DataStoreService<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct DataStoreSetOptions<I> {
    superclass: Instance<I>,
}
impl_inherits!(DataStoreSetOptions<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreVersionPages<I> {
    superclass: Pages<I>,
}
impl_inherits!(DataStoreVersionPages<I>, Pages<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Debris<I> {
    superclass: Instance<I>,
    pub MaxItems: i32,
}
impl_inherits!(Debris<I>, Instance<I>);
impl<I: Default> Default for Debris<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct DebugSettings<I> {
    superclass: Instance<I>,
    pub IsScriptStackTracingEnabled: bool,
    pub ReportSoundWarnings: bool,
    pub TickCountPreciseOverride: enums::TickCountSampleMethod,
}
impl_inherits!(DebugSettings<I>, Instance<I>);
impl<I: Default> Default for DebugSettings<I> {
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
pub struct DebuggablePluginWatcher<I> {
    superclass: Instance<I>,
}
impl_inherits!(DebuggablePluginWatcher<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerBreakpoint<I> {
    superclass: Instance<I>,
    pub Condition: String,
    pub ContinueExecution: bool,
    pub IsContextDependentBreakpoint: bool,
    pub IsEnabled: bool,
    pub Line: i32,
    pub LogExpression: String,
}
impl_inherits!(DebuggerBreakpoint<I>, Instance<I>);
impl<I: Default> Default for DebuggerBreakpoint<I> {
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
pub struct DebuggerConnection<I> {
    superclass: Instance<I>,
}
impl_inherits!(DebuggerConnection<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerConnectionManager<I> {
    superclass: Instance<I>,
    pub Timeout: f64,
}
impl_inherits!(DebuggerConnectionManager<I>, Instance<I>);
impl<I: Default> Default for DebuggerConnectionManager<I> {
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
pub struct DebuggerLuaResponse<I> {
    superclass: Instance<I>,
}
impl_inherits!(DebuggerLuaResponse<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DebuggerManager<I> {
    superclass: Instance<I>,
}
impl_inherits!(DebuggerManager<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DebuggerUIService<I> {
    superclass: Instance<I>,
}
impl_inherits!(DebuggerUIService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DebuggerVariable<I> {
    superclass: Instance<I>,
}
impl_inherits!(DebuggerVariable<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerWatch<I> {
    superclass: Instance<I>,
    pub Expression: String,
}
impl_inherits!(DebuggerWatch<I>, Instance<I>);
impl<I: Default> Default for DebuggerWatch<I> {
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
pub struct Decal<I> {
    superclass: FaceInstance<I>,
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
impl_inherits!(Decal<I>, FaceInstance<I>);
impl<I: Default> Default for Decal<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct DepthOfFieldEffect<I> {
    superclass: PostEffect<I>,
    pub FarIntensity: f32,
    pub FocusDistance: f32,
    pub InFocusRadius: f32,
    pub NearIntensity: f32,
}
impl_inherits!(DepthOfFieldEffect<I>, PostEffect<I>);
impl<I: Default> Default for DepthOfFieldEffect<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct DeviceIdService<I> {
    superclass: Instance<I>,
}
impl_inherits!(DeviceIdService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Dialog<I> {
    superclass: Instance<I>,
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
impl_inherits!(Dialog<I>, Instance<I>);
impl<I: Default> Default for Dialog<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct DialogChoice<I> {
    superclass: Instance<I>,
    pub GoodbyeChoiceActive: bool,
    pub GoodbyeDialog: String,
    pub ResponseDialog: String,
    pub UserDialog: String,
}
impl_inherits!(DialogChoice<I>, Instance<I>);
impl<I: Default> Default for DialogChoice<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct DistortionSoundEffect<I> {
    superclass: SoundEffect<I>,
    pub Level: f32,
}
impl_inherits!(DistortionSoundEffect<I>, SoundEffect<I>);
impl<I: Default> Default for DistortionSoundEffect<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct DockWidgetPluginGui<I> {
    superclass: PluginGui<I>,
}
impl_inherits!(DockWidgetPluginGui<I>, PluginGui<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DoubleConstrainedValue<I> {
    superclass: ValueBase<I>,
    pub MaxValue: f64,
    pub MinValue: f64,
    pub Value: f64,
}
impl_inherits!(DoubleConstrainedValue<I>, ValueBase<I>);
impl<I: Default> Default for DoubleConstrainedValue<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct DraftsService<I> {
    superclass: Instance<I>,
}
impl_inherits!(DraftsService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DragDetector<I> {
    superclass: ClickDetector<I>,
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
impl_inherits!(DragDetector<I>, ClickDetector<I>);
impl<I: Default> Default for DragDetector<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Dragger<I> {
    superclass: Instance<I>,
}
impl_inherits!(Dragger<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DraggerService<I> {
    superclass: Instance<I>,
}
impl_inherits!(DraggerService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DynamicRotate<I> {
    superclass: JointInstance<I>,
    pub BaseAngle: f32,
}
impl_inherits!(DynamicRotate<I>, JointInstance<I>);
impl<I: Default> Default for DynamicRotate<I> {
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
pub struct EchoSoundEffect<I> {
    superclass: SoundEffect<I>,
    pub Delay: f32,
    pub DryLevel: f32,
    pub Feedback: f32,
    pub WetLevel: f32,
}
impl_inherits!(EchoSoundEffect<I>, SoundEffect<I>);
impl<I: Default> Default for EchoSoundEffect<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct EditableImage<I> {
    superclass: Object<I>,
    pub ImageData: BinaryString,
}
impl_inherits!(EditableImage<I>, Object<I>);
impl<I: Default> Default for EditableImage<I> {
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
pub struct EditableMesh<I> {
    superclass: Object<I>,
    pub MeshData: SharedString,
    pub SkinningEnabled: bool,
}
impl_inherits!(EditableMesh<I>, Object<I>);
impl<I: Default> Default for EditableMesh<I> {
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
pub struct EditableService<I> {
    superclass: Instance<I>,
}
impl_inherits!(EditableService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct EmotesPages<I> {
    superclass: InventoryPages<I>,
}
impl_inherits!(EmotesPages<I>, InventoryPages<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct EncodingService<I> {
    superclass: Instance<I>,
}
impl_inherits!(EncodingService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EqualizerSoundEffect<I> {
    superclass: SoundEffect<I>,
    pub HighGain: f32,
    pub LowGain: f32,
    pub MidGain: f32,
}
impl_inherits!(EqualizerSoundEffect<I>, SoundEffect<I>);
impl<I: Default> Default for EqualizerSoundEffect<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct EulerRotationCurve<I> {
    superclass: Instance<I>,
    pub RotationOrder: enums::RotationOrder,
}
impl_inherits!(EulerRotationCurve<I>, Instance<I>);
impl<I: Default> Default for EulerRotationCurve<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct EventIngestService<I> {
    superclass: Instance<I>,
}
impl_inherits!(EventIngestService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExampleV2Service<I> {
    superclass: Instance<I>,
}
impl_inherits!(ExampleV2Service<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExecutedRemoteCommand<I> {
    superclass: Object<I>,
}
impl_inherits!(ExecutedRemoteCommand<I>, Object<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExperienceAuthService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ExperienceAuthService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ExperienceInviteOptions<I> {
    superclass: Instance<I>,
    pub InviteMessageId: String,
    pub InviteUser: i64,
    pub LaunchData: String,
    pub PromptMessage: String,
}
impl_inherits!(ExperienceInviteOptions<I>, Instance<I>);
impl<I: Default> Default for ExperienceInviteOptions<I> {
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
pub struct ExperienceNotificationService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ExperienceNotificationService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExperienceService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ExperienceService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExperienceStateCaptureService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ExperienceStateCaptureService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExperienceStateRecordingService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ExperienceStateRecordingService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExplorerFilter<I> {
    superclass: Instance<I>,
}
impl_inherits!(ExplorerFilter<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExplorerFilterAutocompleter<I> {
    superclass: Instance<I>,
}
impl_inherits!(ExplorerFilterAutocompleter<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExplorerServiceVisibilityService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ExplorerServiceVisibilityService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Explosion<I> {
    superclass: Instance<I>,
    pub BlastPressure: f32,
    pub BlastRadius: f32,
    pub DestroyJointRadiusPercent: f32,
    pub ExplosionType: enums::ExplosionType,
    pub Position: Vector3,
    pub TimeScale: f32,
    pub Visible: bool,
}
impl_inherits!(Explosion<I>, Instance<I>);
impl<I: Default> Default for Explosion<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct FaceAnimatorService<I> {
    superclass: Instance<I>,
}
impl_inherits!(FaceAnimatorService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FaceControls<I> {
    superclass: Instance<I>,
}
impl_inherits!(FaceControls<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FaceInstance<I> {
    superclass: Instance<I>,
    pub Face: enums::NormalId,
}
impl_inherits!(FaceInstance<I>, Instance<I>);
impl<I: Default> Default for FaceInstance<I> {
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
pub struct FacialAgeEstimationService<I> {
    superclass: Instance<I>,
}
impl_inherits!(FacialAgeEstimationService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FacialAnimationRecordingService<I> {
    superclass: Instance<I>,
}
impl_inherits!(FacialAnimationRecordingService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FacialAnimationStreamingServiceStats<I> {
    superclass: Instance<I>,
}
impl_inherits!(FacialAnimationStreamingServiceStats<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FacialAnimationStreamingServiceV2<I> {
    superclass: Instance<I>,
    pub ServiceState: i32,
}
impl_inherits!(FacialAnimationStreamingServiceV2<I>, Instance<I>);
impl<I: Default> Default for FacialAnimationStreamingServiceV2<I> {
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
pub struct FacialAnimationStreamingSubsessionStats<I> {
    superclass: Instance<I>,
}
impl_inherits!(FacialAnimationStreamingSubsessionStats<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FacsImportData<I> {
    superclass: BaseImportData<I>,
}
impl_inherits!(FacsImportData<I>, BaseImportData<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Feature<I> {
    superclass: Instance<I>,
    pub FaceId: enums::NormalId,
    pub InOut: enums::InOut,
    pub LeftRight: enums::LeftRight,
    pub TopBottom: enums::TopBottom,
}
impl_inherits!(Feature<I>, Instance<I>);
impl<I: Default> Default for Feature<I> {
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
pub struct FeatureRestrictionManager<I> {
    superclass: Instance<I>,
}
impl_inherits!(FeatureRestrictionManager<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct File<I> {
    superclass: Instance<I>,
}
impl_inherits!(File<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FileMesh<I> {
    superclass: DataModelMesh<I>,
    pub MeshId: ContentId,
    pub TextureId: ContentId,
}
impl_inherits!(FileMesh<I>, DataModelMesh<I>);
impl<I: Default> Default for FileMesh<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Fire<I> {
    superclass: Instance<I>,
    pub Color: Color3,
    pub Enabled: bool,
    pub SecondaryColor: Color3,
    pub TimeScale: f32,
}
impl_inherits!(Fire<I>, Instance<I>);
impl<I: Default> Default for Fire<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Flag<I> {
    superclass: Tool<I>,
    pub TeamColor: BrickColor,
}
impl_inherits!(Flag<I>, Tool<I>);
impl<I: Default> Default for Flag<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct FlagStand<I> {
    superclass: Part<I>,
    pub TeamColor: BrickColor,
}
impl_inherits!(FlagStand<I>, Part<I>);
impl<I: Default> Default for FlagStand<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct FlagStandService<I> {
    superclass: Instance<I>,
}
impl_inherits!(FlagStandService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FlangeSoundEffect<I> {
    superclass: SoundEffect<I>,
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
impl_inherits!(FlangeSoundEffect<I>, SoundEffect<I>);
impl<I: Default> Default for FlangeSoundEffect<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct FloatCurve<I> {
    superclass: Instance<I>,
    pub ValuesAndTimes: BinaryString,
}
impl_inherits!(FloatCurve<I>, Instance<I>);
impl<I: Default> Default for FloatCurve<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct FloorWire<I> {
    superclass: GuiBase3d<I>,
    pub CycleOffset: f32,
    pub From: Ref,
    pub StudsBetweenTextures: f32,
    pub Texture: ContentId,
    pub TextureSize: Vector2,
    pub To: Ref,
    pub Velocity: f32,
    pub WireRadius: f32,
}
impl_inherits!(FloorWire<I>, GuiBase3d<I>);
impl<I: Default> Default for FloorWire<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct FluidForceSensor<I> {
    superclass: SensorBase<I>,
}
impl_inherits!(FluidForceSensor<I>, SensorBase<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FlyweightService<I> {
    superclass: Instance<I>,
}
impl_inherits!(FlyweightService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Folder<I> {
    superclass: Instance<I>,
}
impl_inherits!(Folder<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ForceField<I> {
    superclass: Instance<I>,
    pub Visible: bool,
}
impl_inherits!(ForceField<I>, Instance<I>);
impl<I: Default> Default for ForceField<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct FormFactorPart<I> {
    superclass: BasePart<I>,
}
impl_inherits!(FormFactorPart<I>, BasePart<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Frame<I> {
    superclass: GuiObject<I>,
    pub Style: enums::FrameStyle,
}
impl_inherits!(Frame<I>, GuiObject<I>);
impl<I: Default> Default for Frame<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct FriendPages<I> {
    superclass: Pages<I>,
}
impl_inherits!(FriendPages<I>, Pages<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FriendService<I> {
    superclass: Instance<I>,
}
impl_inherits!(FriendService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FunctionalTest<I> {
    superclass: Instance<I>,
    pub Description: String,
    pub HasMigratedSettingsToTestService: bool,
}
impl_inherits!(FunctionalTest<I>, Instance<I>);
impl<I: Default> Default for FunctionalTest<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct GamePassService<I> {
    superclass: Instance<I>,
}
impl_inherits!(GamePassService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GameSettings<I> {
    superclass: Instance<I>,
    pub VideoCaptureEnabled: bool,
}
impl_inherits!(GameSettings<I>, Instance<I>);
impl<I: Default> Default for GameSettings<I> {
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
pub struct GamepadService<I> {
    superclass: Instance<I>,
    pub GamepadCursorEnabled: bool,
}
impl_inherits!(GamepadService<I>, Instance<I>);
impl<I: Default> Default for GamepadService<I> {
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
pub struct GenerationService<I> {
    superclass: Instance<I>,
}
impl_inherits!(GenerationService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GenericChallengeService<I> {
    superclass: Instance<I>,
}
impl_inherits!(GenericChallengeService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GenericSettings<I> {
    superclass: ServiceProvider<I>,
}
impl_inherits!(GenericSettings<I>, ServiceProvider<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Geometry<I> {
    superclass: Instance<I>,
}
impl_inherits!(Geometry<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GeometryService<I> {
    superclass: Instance<I>,
}
impl_inherits!(GeometryService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GetTextBoundsParams<I> {
    superclass: Instance<I>,
    pub Font: Font,
    pub RichText: bool,
    pub Size: f32,
    pub Text: String,
    pub Width: f32,
}
impl_inherits!(GetTextBoundsParams<I>, Instance<I>);
impl<I: Default> Default for GetTextBoundsParams<I> {
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
pub struct GlobalDataStore<I> {
    superclass: Instance<I>,
}
impl_inherits!(GlobalDataStore<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GlobalSettings<I> {
    superclass: GenericSettings<I>,
}
impl_inherits!(GlobalSettings<I>, GenericSettings<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Glue<I> {
    superclass: JointInstance<I>,
    pub F0: Vector3,
    pub F1: Vector3,
    pub F2: Vector3,
    pub F3: Vector3,
}
impl_inherits!(Glue<I>, JointInstance<I>);
impl<I: Default> Default for Glue<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct GroundController<I> {
    superclass: ControllerBase<I>,
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
impl_inherits!(GroundController<I>, ControllerBase<I>);
impl<I: Default> Default for GroundController<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct GroupImportData<I> {
    superclass: BaseImportData<I>,
    pub Anchored: bool,
    pub ImportAsModelAsset: bool,
    pub InsertInWorkspace: bool,
}
impl_inherits!(GroupImportData<I>, BaseImportData<I>);
impl<I: Default> Default for GroupImportData<I> {
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
pub struct GroupService<I> {
    superclass: Instance<I>,
}
impl_inherits!(GroupService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GuiBase<I> {
    superclass: Instance<I>,
}
impl_inherits!(GuiBase<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiBase2d<I> {
    superclass: GuiBase<I>,
    pub AutoLocalize: bool,
    pub RootLocalizationTable: Ref,
    pub SelectionBehaviorDown: enums::SelectionBehavior,
    pub SelectionBehaviorLeft: enums::SelectionBehavior,
    pub SelectionBehaviorRight: enums::SelectionBehavior,
    pub SelectionBehaviorUp: enums::SelectionBehavior,
    pub SelectionGroup: bool,
}
impl_inherits!(GuiBase2d<I>, GuiBase<I>);
impl<I: Default> Default for GuiBase2d<I> {
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
pub struct GuiBase3d<I> {
    superclass: GuiBase<I>,
    pub Color3: Color3,
    pub Transparency: f32,
    pub Visible: bool,
}
impl_inherits!(GuiBase3d<I>, GuiBase<I>);
impl<I: Default> Default for GuiBase3d<I> {
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
pub struct GuiButton<I> {
    superclass: GuiObject<I>,
    pub AutoButtonColor: bool,
    pub HoverHapticEffect: Ref,
    pub Modal: bool,
    pub PressHapticEffect: Ref,
    pub Selected: bool,
    pub Style: enums::ButtonStyle,
}
impl_inherits!(GuiButton<I>, GuiObject<I>);
impl<I: Default> Default for GuiButton<I> {
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
pub struct GuiLabel<I> {
    superclass: GuiObject<I>,
}
impl_inherits!(GuiLabel<I>, GuiObject<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GuiMain<I> {
    superclass: ScreenGui<I>,
}
impl_inherits!(GuiMain<I>, ScreenGui<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiObject<I> {
    superclass: GuiBase2d<I>,
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
impl_inherits!(GuiObject<I>, GuiBase2d<I>);
impl<I: Default> Default for GuiObject<I> {
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
pub struct GuiService<I> {
    superclass: Instance<I>,
    pub AutoSelectGuiEnabled: bool,
    pub GuiNavigationEnabled: bool,
    pub SelectedObject: Ref,
}
impl_inherits!(GuiService<I>, Instance<I>);
impl<I: Default> Default for GuiService<I> {
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
pub struct GuidRegistryService<I> {
    superclass: Instance<I>,
}
impl_inherits!(GuidRegistryService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HSRDataContentProvider<I> {
    superclass: CacheableContentProvider<I>,
}
impl_inherits!(HSRDataContentProvider<I>, CacheableContentProvider<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HandRigDescription<I> {
    superclass: Instance<I>,
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
impl_inherits!(HandRigDescription<I>, Instance<I>);
impl<I: Default> Default for HandRigDescription<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct HandleAdornment<I> {
    superclass: PVAdornment<I>,
    pub AdornCullingMode: enums::AdornCullingMode,
    pub AlwaysOnTop: bool,
    pub CFrame: CFrame,
    pub SizeRelativeOffset: Vector3,
    pub ZIndex: i32,
}
impl_inherits!(HandleAdornment<I>, PVAdornment<I>);
impl<I: Default> Default for HandleAdornment<I> {
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
pub struct Handles<I> {
    superclass: HandlesBase<I>,
    pub Faces: Faces,
    pub Style: enums::HandlesStyle,
}
impl_inherits!(Handles<I>, HandlesBase<I>);
impl<I: Default> Default for Handles<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct HandlesBase<I> {
    superclass: PartAdornment<I>,
}
impl_inherits!(HandlesBase<I>, PartAdornment<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HapticEffect<I> {
    superclass: Instance<I>,
    pub Looped: bool,
    pub Position: Vector3,
    pub Radius: f32,
    pub Type: enums::HapticEffectType,
    pub Waveform: Ref,
    pub WaveformData: BinaryString,
}
impl_inherits!(HapticEffect<I>, Instance<I>);
impl<I: Default> Default for HapticEffect<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct HapticService<I> {
    superclass: Instance<I>,
}
impl_inherits!(HapticService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HarmonyService<I> {
    superclass: Instance<I>,
}
impl_inherits!(HarmonyService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Hat<I> {
    superclass: Accoutrement<I>,
}
impl_inherits!(Hat<I>, Accoutrement<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HeapProfilerService<I> {
    superclass: Instance<I>,
}
impl_inherits!(HeapProfilerService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HeatmapService<I> {
    superclass: Instance<I>,
}
impl_inherits!(HeatmapService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HeightmapImporterService<I> {
    superclass: Instance<I>,
}
impl_inherits!(HeightmapImporterService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HiddenSurfaceRemovalAsset<I> {
    superclass: Instance<I>,
    pub HsrData: BinaryString,
    pub HsrMeshIdData: BinaryString,
}
impl_inherits!(HiddenSurfaceRemovalAsset<I>, Instance<I>);
impl<I: Default> Default for HiddenSurfaceRemovalAsset<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Highlight<I> {
    superclass: Instance<I>,
    pub Adornee: Ref,
    pub DepthMode: enums::HighlightDepthMode,
    pub Enabled: bool,
    pub FillColor: Color3,
    pub FillTransparency: f32,
    pub OutlineColor: Color3,
    pub OutlineTransparency: f32,
}
impl_inherits!(Highlight<I>, Instance<I>);
impl<I: Default> Default for Highlight<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct HingeConstraint<I> {
    superclass: Constraint<I>,
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
impl_inherits!(HingeConstraint<I>, Constraint<I>);
impl<I: Default> Default for HingeConstraint<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Hint<I> {
    superclass: Message<I>,
}
impl_inherits!(Hint<I>, Message<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Hole<I> {
    superclass: Feature<I>,
}
impl_inherits!(Hole<I>, Feature<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Hopper<I> {
    superclass: Instance<I>,
}
impl_inherits!(Hopper<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HopperBin<I> {
    superclass: BackpackItem<I>,
    pub Active: bool,
    pub BinType: enums::BinType,
}
impl_inherits!(HopperBin<I>, BackpackItem<I>);
impl<I: Default> Default for HopperBin<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct HttpRbxApiService<I> {
    superclass: Instance<I>,
}
impl_inherits!(HttpRbxApiService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HttpRequest<I> {
    superclass: Instance<I>,
}
impl_inherits!(HttpRequest<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HttpService<I> {
    superclass: Instance<I>,
    pub HttpEnabled: bool,
}
impl_inherits!(HttpService<I>, Instance<I>);
impl<I: Default> Default for HttpService<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Humanoid<I> {
    superclass: Instance<I>,
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
impl_inherits!(Humanoid<I>, Instance<I>);
impl<I: Default> Default for Humanoid<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct HumanoidController<I> {
    superclass: Controller<I>,
}
impl_inherits!(HumanoidController<I>, Controller<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HumanoidDescription<I> {
    superclass: Instance<I>,
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
impl_inherits!(HumanoidDescription<I>, Instance<I>);
impl<I: Default> Default for HumanoidDescription<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct HumanoidRigDescription<I> {
    superclass: Instance<I>,
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
impl_inherits!(HumanoidRigDescription<I>, Instance<I>);
impl<I: Default> Default for HumanoidRigDescription<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct IKControl<I> {
    superclass: Instance<I>,
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
impl_inherits!(IKControl<I>, Instance<I>);
impl<I: Default> Default for IKControl<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ILegacyStudioBridge<I> {
    superclass: Instance<I>,
}
impl_inherits!(ILegacyStudioBridge<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct IXPService<I> {
    superclass: Instance<I>,
}
impl_inherits!(IXPService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ImageButton<I> {
    superclass: GuiButton<I>,
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
impl_inherits!(ImageButton<I>, GuiButton<I>);
impl<I: Default> Default for ImageButton<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ImageHandleAdornment<I> {
    superclass: HandleAdornment<I>,
    pub Image: ContentId,
    pub Size: Vector2,
}
impl_inherits!(ImageHandleAdornment<I>, HandleAdornment<I>);
impl<I: Default> Default for ImageHandleAdornment<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ImageLabel<I> {
    superclass: GuiLabel<I>,
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
impl_inherits!(ImageLabel<I>, GuiLabel<I>);
impl<I: Default> Default for ImageLabel<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ImportSession<I> {
    superclass: Instance<I>,
}
impl_inherits!(ImportSession<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct IncrementalPatchBuilder<I> {
    superclass: Instance<I>,
    pub AddPathsToBundle: bool,
    pub BuildDebouncePeriod: f64,
    pub HighCompression: bool,
    pub SerializePatch: bool,
    pub UseFileLevelCompressionInsteadOfChunk: bool,
    pub ZstdCompression: bool,
}
impl_inherits!(IncrementalPatchBuilder<I>, Instance<I>);
impl<I: Default> Default for IncrementalPatchBuilder<I> {
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
pub struct InputAction<I> {
    superclass: Instance<I>,
    pub Enabled: bool,
    pub Type: enums::InputActionType,
}
impl_inherits!(InputAction<I>, Instance<I>);
impl<I: Default> Default for InputAction<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct InputBinding<I> {
    superclass: Instance<I>,
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
impl_inherits!(InputBinding<I>, Instance<I>);
impl<I: Default> Default for InputBinding<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct InputContext<I> {
    superclass: Instance<I>,
    pub Enabled: bool,
    pub Priority: i32,
    pub Sink: bool,
}
impl_inherits!(InputContext<I>, Instance<I>);
impl<I: Default> Default for InputContext<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct InputObject<I> {
    superclass: Instance<I>,
}
impl_inherits!(InputObject<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InsertService<I> {
    superclass: Instance<I>,
    pub AllowClientInsertModels: bool,
    pub AllowInsertFreeModels: bool,
}
impl_inherits!(InsertService<I>, Instance<I>);
impl<I: Default> Default for InsertService<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Instance<I> {
    superclass: Object<I>,
    pub Capabilities: SecurityCapabilities,
    pub HistoryId: UniqueId,
    pub Name: String,
    pub SourceAssetId: i64,
    pub Tags: Tags,
    pub UniqueId: UniqueId,
}
impl_inherits!(Instance<I>, Object<I>);
impl<I: Default> Default for Instance<I> {
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
pub struct InstanceAdornment<I> {
    superclass: GuiBase3d<I>,
    pub Adornee: Ref,
}
impl_inherits!(InstanceAdornment<I>, GuiBase3d<I>);
impl<I: Default> Default for InstanceAdornment<I> {
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
pub struct InstanceExtensionsService<I> {
    superclass: Instance<I>,
}
impl_inherits!(InstanceExtensionsService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct IntConstrainedValue<I> {
    superclass: ValueBase<I>,
    pub MaxValue: i64,
    pub MinValue: i64,
    pub Value: i64,
}
impl_inherits!(IntConstrainedValue<I>, ValueBase<I>);
impl<I: Default> Default for IntConstrainedValue<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct IntValue<I> {
    superclass: ValueBase<I>,
    pub Value: i64,
}
impl_inherits!(IntValue<I>, ValueBase<I>);
impl<I: Default> Default for IntValue<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct InternalSyncItem<I> {
    superclass: Instance<I>,
    pub AutoSync: bool,
    pub Enabled: bool,
    pub Path: String,
}
impl_inherits!(InternalSyncItem<I>, Instance<I>);
impl<I: Default> Default for InternalSyncItem<I> {
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
pub struct InternalSyncService<I> {
    superclass: Instance<I>,
}
impl_inherits!(InternalSyncService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct IntersectOperation<I> {
    superclass: PartOperation<I>,
}
impl_inherits!(IntersectOperation<I>, PartOperation<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct InventoryPages<I> {
    superclass: Pages<I>,
}
impl_inherits!(InventoryPages<I>, Pages<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct JointImportData<I> {
    superclass: BaseImportData<I>,
}
impl_inherits!(JointImportData<I>, BaseImportData<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct JointInstance<I> {
    superclass: Instance<I>,
    pub C0: CFrame,
    pub C1: CFrame,
    pub Enabled: bool,
    pub Part0: Ref,
    pub Part1: Ref,
}
impl_inherits!(JointInstance<I>, Instance<I>);
impl<I: Default> Default for JointInstance<I> {
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
pub struct JointsService<I> {
    superclass: Instance<I>,
}
impl_inherits!(JointsService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct KeyboardService<I> {
    superclass: Instance<I>,
}
impl_inherits!(KeyboardService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Keyframe<I> {
    superclass: Instance<I>,
    pub Time: f32,
}
impl_inherits!(Keyframe<I>, Instance<I>);
impl<I: Default> Default for Keyframe<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct KeyframeMarker<I> {
    superclass: Instance<I>,
    pub Value: String,
}
impl_inherits!(KeyframeMarker<I>, Instance<I>);
impl<I: Default> Default for KeyframeMarker<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct KeyframeSequence<I> {
    superclass: AnimationClip<I>,
    pub AuthoredHipHeight: f32,
}
impl_inherits!(KeyframeSequence<I>, AnimationClip<I>);
impl<I: Default> Default for KeyframeSequence<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct KeyframeSequenceProvider<I> {
    superclass: Instance<I>,
}
impl_inherits!(KeyframeSequenceProvider<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LSPFileSyncService<I> {
    superclass: Instance<I>,
}
impl_inherits!(LSPFileSyncService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LanguageService<I> {
    superclass: Instance<I>,
}
impl_inherits!(LanguageService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LayerCollector<I> {
    superclass: GuiBase2d<I>,
    pub Enabled: bool,
    pub ResetOnSpawn: bool,
    pub ZIndexBehavior: enums::ZIndexBehavior,
}
impl_inherits!(LayerCollector<I>, GuiBase2d<I>);
impl<I: Default> Default for LayerCollector<I> {
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
pub struct LegacyStudioBridge<I> {
    superclass: ILegacyStudioBridge<I>,
}
impl_inherits!(LegacyStudioBridge<I>, ILegacyStudioBridge<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Light<I> {
    superclass: Instance<I>,
    pub Brightness: f32,
    pub Color: Color3,
    pub Enabled: bool,
    pub Shadows: bool,
}
impl_inherits!(Light<I>, Instance<I>);
impl<I: Default> Default for Light<I> {
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
pub struct Lighting<I> {
    superclass: Instance<I>,
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
impl_inherits!(Lighting<I>, Instance<I>);
impl<I: Default> Default for Lighting<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct LineForce<I> {
    superclass: Constraint<I>,
    pub ApplyAtCenterOfMass: bool,
    pub InverseSquareLaw: bool,
    pub Magnitude: f32,
    pub MaxForce: f32,
    pub ReactionForceEnabled: bool,
}
impl_inherits!(LineForce<I>, Constraint<I>);
impl<I: Default> Default for LineForce<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct LineHandleAdornment<I> {
    superclass: HandleAdornment<I>,
    pub Length: f32,
    pub Thickness: f32,
}
impl_inherits!(LineHandleAdornment<I>, HandleAdornment<I>);
impl<I: Default> Default for LineHandleAdornment<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct LinearVelocity<I> {
    superclass: Constraint<I>,
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
impl_inherits!(LinearVelocity<I>, Constraint<I>);
impl<I: Default> Default for LinearVelocity<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct LinkingService<I> {
    superclass: Instance<I>,
}
impl_inherits!(LinkingService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LiveScriptingService<I> {
    superclass: Instance<I>,
}
impl_inherits!(LiveScriptingService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LiveSyncService<I> {
    superclass: Instance<I>,
}
impl_inherits!(LiveSyncService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LocalDebuggerConnection<I> {
    superclass: DebuggerConnection<I>,
}
impl_inherits!(LocalDebuggerConnection<I>, DebuggerConnection<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LocalScript<I> {
    superclass: Script<I>,
}
impl_inherits!(LocalScript<I>, Script<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LocalStorageService<I> {
    superclass: Instance<I>,
}
impl_inherits!(LocalStorageService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LocalizationService<I> {
    superclass: Instance<I>,
}
impl_inherits!(LocalizationService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LocalizationTable<I> {
    superclass: Instance<I>,
    pub Contents: String,
    pub SourceLocaleId: String,
}
impl_inherits!(LocalizationTable<I>, Instance<I>);
impl<I: Default> Default for LocalizationTable<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct LodDataEntity<I> {
    superclass: Instance<I>,
}
impl_inherits!(LodDataEntity<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LodDataService<I> {
    superclass: Instance<I>,
}
impl_inherits!(LodDataService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LogReporterService<I> {
    superclass: Instance<I>,
}
impl_inherits!(LogReporterService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LogService<I> {
    superclass: Instance<I>,
}
impl_inherits!(LogService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LoginService<I> {
    superclass: Instance<I>,
}
impl_inherits!(LoginService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LuaSettings<I> {
    superclass: Instance<I>,
}
impl_inherits!(LuaSettings<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LuaSourceContainer<I> {
    superclass: Instance<I>,
    pub ScriptGuid: String,
}
impl_inherits!(LuaSourceContainer<I>, Instance<I>);
impl<I: Default> Default for LuaSourceContainer<I> {
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
pub struct LuaWebService<I> {
    superclass: Instance<I>,
}
impl_inherits!(LuaWebService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LuauScriptAnalyzerService<I> {
    superclass: Instance<I>,
}
impl_inherits!(LuauScriptAnalyzerService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MLModelDeliveryService<I> {
    superclass: Instance<I>,
}
impl_inherits!(MLModelDeliveryService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MLService<I> {
    superclass: Instance<I>,
}
impl_inherits!(MLService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MLSession<I> {
    superclass: Object<I>,
}
impl_inherits!(MLSession<I>, Object<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ManualGlue<I> {
    superclass: ManualSurfaceJointInstance<I>,
}
impl_inherits!(ManualGlue<I>, ManualSurfaceJointInstance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ManualSurfaceJointInstance<I> {
    superclass: JointInstance<I>,
}
impl_inherits!(ManualSurfaceJointInstance<I>, JointInstance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ManualWeld<I> {
    superclass: ManualSurfaceJointInstance<I>,
}
impl_inherits!(ManualWeld<I>, ManualSurfaceJointInstance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MarkerCurve<I> {
    superclass: Instance<I>,
    pub ValuesAndTimes: BinaryString,
}
impl_inherits!(MarkerCurve<I>, Instance<I>);
impl<I: Default> Default for MarkerCurve<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct MarketplaceService<I> {
    superclass: Instance<I>,
}
impl_inherits!(MarketplaceService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MatchmakingService<I> {
    superclass: Instance<I>,
}
impl_inherits!(MatchmakingService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MaterialGenerationService<I> {
    superclass: Instance<I>,
}
impl_inherits!(MaterialGenerationService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MaterialImportData<I> {
    superclass: BaseImportData<I>,
    pub DiffuseFilePath: String,
    pub EmissiveFilePath: String,
    pub MetalnessFilePath: String,
    pub NormalFilePath: String,
    pub RoughnessFilePath: String,
}
impl_inherits!(MaterialImportData<I>, BaseImportData<I>);
impl<I: Default> Default for MaterialImportData<I> {
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
pub struct MaterialService<I> {
    superclass: Instance<I>,
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
impl_inherits!(MaterialService<I>, Instance<I>);
impl<I: Default> Default for MaterialService<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct MaterialVariant<I> {
    superclass: Instance<I>,
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
impl_inherits!(MaterialVariant<I>, Instance<I>);
impl<I: Default> Default for MaterialVariant<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct MemStorageConnection<I> {
    superclass: Instance<I>,
}
impl_inherits!(MemStorageConnection<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MemStorageService<I> {
    superclass: Instance<I>,
}
impl_inherits!(MemStorageService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MemoryStoreHashMap<I> {
    superclass: Instance<I>,
}
impl_inherits!(MemoryStoreHashMap<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MemoryStoreHashMapPages<I> {
    superclass: Pages<I>,
}
impl_inherits!(MemoryStoreHashMapPages<I>, Pages<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MemoryStoreQueue<I> {
    superclass: Instance<I>,
}
impl_inherits!(MemoryStoreQueue<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MemoryStoreService<I> {
    superclass: Instance<I>,
}
impl_inherits!(MemoryStoreService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MemoryStoreSortedMap<I> {
    superclass: Instance<I>,
}
impl_inherits!(MemoryStoreSortedMap<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MeshContentProvider<I> {
    superclass: CacheableContentProvider<I>,
}
impl_inherits!(MeshContentProvider<I>, CacheableContentProvider<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MeshImportData<I> {
    superclass: BaseImportData<I>,
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
impl_inherits!(MeshImportData<I>, BaseImportData<I>);
impl<I: Default> Default for MeshImportData<I> {
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
pub struct MeshPart<I> {
    superclass: TriangleMeshPart<I>,
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
impl_inherits!(MeshPart<I>, TriangleMeshPart<I>);
impl<I: Default> Default for MeshPart<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Message<I> {
    superclass: Instance<I>,
    pub Text: String,
}
impl_inherits!(Message<I>, Instance<I>);
impl<I: Default> Default for Message<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct MessageBusConnection<I> {
    superclass: Instance<I>,
}
impl_inherits!(MessageBusConnection<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MessageBusService<I> {
    superclass: Instance<I>,
}
impl_inherits!(MessageBusService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MessagingService<I> {
    superclass: Instance<I>,
}
impl_inherits!(MessagingService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MetaBreakpoint<I> {
    superclass: Instance<I>,
    pub Condition: String,
    pub ContinueExecution: bool,
    pub Enabled: bool,
    pub Line: i32,
    pub LogMessage: String,
    pub RemoveOnHit: bool,
    pub Script: String,
}
impl_inherits!(MetaBreakpoint<I>, Instance<I>);
impl<I: Default> Default for MetaBreakpoint<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct MetaBreakpointContext<I> {
    superclass: Instance<I>,
    pub ContextDataInternal: String,
}
impl_inherits!(MetaBreakpointContext<I>, Instance<I>);
impl<I: Default> Default for MetaBreakpointContext<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct MetaBreakpointManager<I> {
    superclass: Instance<I>,
}
impl_inherits!(MetaBreakpointManager<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MicroProfilerService<I> {
    superclass: Instance<I>,
    pub ContextLabel: String,
}
impl_inherits!(MicroProfilerService<I>, Instance<I>);
impl<I: Default> Default for MicroProfilerService<I> {
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
pub struct Model<I> {
    superclass: PVInstance<I>,
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
impl_inherits!(Model<I>, PVInstance<I>);
impl<I: Default> Default for Model<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ModerationService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ModerationService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ModuleScript<I> {
    superclass: LuaSourceContainer<I>,
    pub LinkedSource: ContentId,
    pub Source: String,
}
impl_inherits!(ModuleScript<I>, LuaSourceContainer<I>);
impl<I: Default> Default for ModuleScript<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Motor<I> {
    superclass: JointInstance<I>,
    pub DesiredAngle: f32,
    pub MaxVelocity: f32,
}
impl_inherits!(Motor<I>, JointInstance<I>);
impl<I: Default> Default for Motor<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Motor6D<I> {
    superclass: Motor<I>,
}
impl_inherits!(Motor6D<I>, Motor<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MotorFeature<I> {
    superclass: Feature<I>,
}
impl_inherits!(MotorFeature<I>, Feature<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Mouse<I> {
    superclass: Instance<I>,
    pub IconContent: Content,
    pub TargetFilter: Ref,
}
impl_inherits!(Mouse<I>, Instance<I>);
impl<I: Default> Default for Mouse<I> {
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
pub struct MouseService<I> {
    superclass: Instance<I>,
}
impl_inherits!(MouseService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MultipleDocumentInterfaceInstance<I> {
    superclass: Instance<I>,
}
impl_inherits!(MultipleDocumentInterfaceInstance<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NegateOperation<I> {
    superclass: PartOperation<I>,
    pub PreviousOperation: enums::NegateOperationHiddenHistory,
}
impl_inherits!(NegateOperation<I>, PartOperation<I>);
impl<I: Default> Default for NegateOperation<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct NetworkClient<I> {
    superclass: NetworkPeer<I>,
}
impl_inherits!(NetworkClient<I>, NetworkPeer<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct NetworkMarker<I> {
    superclass: Instance<I>,
}
impl_inherits!(NetworkMarker<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct NetworkPeer<I> {
    superclass: Instance<I>,
}
impl_inherits!(NetworkPeer<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct NetworkReplicator<I> {
    superclass: Instance<I>,
}
impl_inherits!(NetworkReplicator<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct NetworkServer<I> {
    superclass: NetworkPeer<I>,
}
impl_inherits!(NetworkServer<I>, NetworkPeer<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NetworkSettings<I> {
    superclass: Instance<I>,
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
impl_inherits!(NetworkSettings<I>, Instance<I>);
impl<I: Default> Default for NetworkSettings<I> {
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
pub struct NoCollisionConstraint<I> {
    superclass: Instance<I>,
    pub Enabled: bool,
    pub Part0: Ref,
    pub Part1: Ref,
}
impl_inherits!(NoCollisionConstraint<I>, Instance<I>);
impl<I: Default> Default for NoCollisionConstraint<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Noise<I> {
    superclass: Instance<I>,
    pub NoiseType: enums::NoiseType,
    pub Seed: i32,
}
impl_inherits!(Noise<I>, Instance<I>);
impl<I: Default> Default for Noise<I> {
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
pub struct NonReplicatedCSGDictionaryService<I> {
    superclass: FlyweightService<I>,
}
impl_inherits!(NonReplicatedCSGDictionaryService<I>, FlyweightService<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct NotificationService<I> {
    superclass: Instance<I>,
}
impl_inherits!(NotificationService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NumberPose<I> {
    superclass: PoseBase<I>,
    pub Value: f64,
}
impl_inherits!(NumberPose<I>, PoseBase<I>);
impl<I: Default> Default for NumberPose<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct NumberValue<I> {
    superclass: ValueBase<I>,
    pub Value: f64,
}
impl_inherits!(NumberValue<I>, ValueBase<I>);
impl<I: Default> Default for NumberValue<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Object<I> {
    superclass: I,
}
impl_inherits!(Object<I>, I);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ObjectValue<I> {
    superclass: ValueBase<I>,
    pub Value: Ref,
}
impl_inherits!(ObjectValue<I>, ValueBase<I>);
impl<I: Default> Default for ObjectValue<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct OmniRecommendationsService<I> {
    superclass: Instance<I>,
}
impl_inherits!(OmniRecommendationsService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct OpenCloudApiV1<I> {
    superclass: Instance<I>,
}
impl_inherits!(OpenCloudApiV1<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct OpenCloudService<I> {
    superclass: Instance<I>,
}
impl_inherits!(OpenCloudService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct OperationGraph<I> {
    superclass: Instance<I>,
}
impl_inherits!(OperationGraph<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct OrderedDataStore<I> {
    superclass: GlobalDataStore<I>,
}
impl_inherits!(OrderedDataStore<I>, GlobalDataStore<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct OutfitPages<I> {
    superclass: Pages<I>,
}
impl_inherits!(OutfitPages<I>, Pages<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PVAdornment<I> {
    superclass: GuiBase3d<I>,
    pub Adornee: Ref,
}
impl_inherits!(PVAdornment<I>, GuiBase3d<I>);
impl<I: Default> Default for PVAdornment<I> {
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
pub struct PVInstance<I> {
    superclass: Instance<I>,
}
impl_inherits!(PVInstance<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PackageLink<I> {
    superclass: Instance<I>,
    pub AutoUpdate: bool,
    pub DefaultName: String,
    pub ModifiedState: i32,
    pub SerializedDefaultAttributes: BinaryString,
    pub VersionIdSerialize: i64,
}
impl_inherits!(PackageLink<I>, Instance<I>);
impl<I: Default> Default for PackageLink<I> {
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
pub struct PackageService<I> {
    superclass: Instance<I>,
}
impl_inherits!(PackageService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PackageUIService<I> {
    superclass: Instance<I>,
}
impl_inherits!(PackageUIService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Pages<I> {
    superclass: Instance<I>,
}
impl_inherits!(Pages<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Pants<I> {
    superclass: Clothing<I>,
    pub PantsTemplate: ContentId,
}
impl_inherits!(Pants<I>, Clothing<I>);
impl<I: Default> Default for Pants<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ParabolaAdornment<I> {
    superclass: PVAdornment<I>,
}
impl_inherits!(ParabolaAdornment<I>, PVAdornment<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Part<I> {
    superclass: FormFactorPart<I>,
}
impl_inherits!(Part<I>, FormFactorPart<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PartAdornment<I> {
    superclass: GuiBase3d<I>,
    pub Adornee: Ref,
}
impl_inherits!(PartAdornment<I>, GuiBase3d<I>);
impl<I: Default> Default for PartAdornment<I> {
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
pub struct PartOperation<I> {
    superclass: TriangleMeshPart<I>,
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
impl_inherits!(PartOperation<I>, TriangleMeshPart<I>);
impl<I: Default> Default for PartOperation<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct PartOperationAsset<I> {
    superclass: Instance<I>,
    pub ChildData: BinaryString,
    pub MeshData: BinaryString,
}
impl_inherits!(PartOperationAsset<I>, Instance<I>);
impl<I: Default> Default for PartOperationAsset<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ParticleEmitter<I> {
    superclass: Instance<I>,
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
impl_inherits!(ParticleEmitter<I>, Instance<I>);
impl<I: Default> Default for ParticleEmitter<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct PartyEmulatorService<I> {
    superclass: Instance<I>,
}
impl_inherits!(PartyEmulatorService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PatchBundlerFileWatch<I> {
    superclass: Instance<I>,
}
impl_inherits!(PatchBundlerFileWatch<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PatchMapping<I> {
    superclass: Instance<I>,
    pub FlattenTree: bool,
    pub PatchId: String,
    pub TargetPath: String,
}
impl_inherits!(PatchMapping<I>, Instance<I>);
impl<I: Default> Default for PatchMapping<I> {
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
pub struct Path<I> {
    superclass: Instance<I>,
}
impl_inherits!(Path<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Path2D<I> {
    superclass: GuiBase<I>,
    pub Closed: bool,
    pub Color3: Color3,
    pub PropertiesSerialize: BinaryString,
    pub Thickness: f32,
    pub Transparency: f32,
    pub Visible: bool,
    pub ZIndex: i32,
}
impl_inherits!(Path2D<I>, GuiBase<I>);
impl<I: Default> Default for Path2D<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct PathfindingLink<I> {
    superclass: Instance<I>,
    pub Attachment0: Ref,
    pub Attachment1: Ref,
    pub IsBidirectional: bool,
    pub Label: String,
}
impl_inherits!(PathfindingLink<I>, Instance<I>);
impl<I: Default> Default for PathfindingLink<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct PathfindingModifier<I> {
    superclass: Instance<I>,
    pub Label: String,
    pub PassThrough: bool,
}
impl_inherits!(PathfindingModifier<I>, Instance<I>);
impl<I: Default> Default for PathfindingModifier<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct PathfindingService<I> {
    superclass: Instance<I>,
}
impl_inherits!(PathfindingService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PausedState<I> {
    superclass: Instance<I>,
}
impl_inherits!(PausedState<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PausedStateBreakpoint<I> {
    superclass: PausedState<I>,
}
impl_inherits!(PausedStateBreakpoint<I>, PausedState<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PausedStateException<I> {
    superclass: PausedState<I>,
}
impl_inherits!(PausedStateException<I>, PausedState<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PerformanceControlService<I> {
    superclass: Instance<I>,
}
impl_inherits!(PerformanceControlService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PermissionsService<I> {
    superclass: Instance<I>,
}
impl_inherits!(PermissionsService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PhysicsService<I> {
    superclass: Instance<I>,
}
impl_inherits!(PhysicsService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PhysicsSettings<I> {
    superclass: Instance<I>,
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
impl_inherits!(PhysicsSettings<I>, Instance<I>);
impl<I: Default> Default for PhysicsSettings<I> {
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
pub struct PitchShiftSoundEffect<I> {
    superclass: SoundEffect<I>,
    pub Octave: f32,
}
impl_inherits!(PitchShiftSoundEffect<I>, SoundEffect<I>);
impl<I: Default> Default for PitchShiftSoundEffect<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct PlaceAssetIdsService<I> {
    superclass: Instance<I>,
}
impl_inherits!(PlaceAssetIdsService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlaceStatsService<I> {
    superclass: Instance<I>,
}
impl_inherits!(PlaceStatsService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlacesService<I> {
    superclass: Instance<I>,
}
impl_inherits!(PlacesService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Plane<I> {
    superclass: PlaneConstraint<I>,
}
impl_inherits!(Plane<I>, PlaneConstraint<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlaneConstraint<I> {
    superclass: Constraint<I>,
}
impl_inherits!(PlaneConstraint<I>, Constraint<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Platform<I> {
    superclass: Part<I>,
}
impl_inherits!(Platform<I>, Part<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlatformCloudStorageService<I> {
    superclass: Instance<I>,
}
impl_inherits!(PlatformCloudStorageService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlatformFriendsService<I> {
    superclass: Instance<I>,
}
impl_inherits!(PlatformFriendsService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Player<I> {
    superclass: Instance<I>,
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
impl_inherits!(Player<I>, Instance<I>);
impl<I: Default> Default for Player<I> {
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
pub struct PlayerData<I> {
    superclass: Instance<I>,
}
impl_inherits!(PlayerData<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlayerDataRecord<I> {
    superclass: Instance<I>,
}
impl_inherits!(PlayerDataRecord<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlayerDataRecordConfig<I> {
    superclass: Instance<I>,
}
impl_inherits!(PlayerDataRecordConfig<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerDataService<I> {
    superclass: Instance<I>,
    pub LoadFailureBehavior: enums::PlayerDataLoadFailureBehavior,
}
impl_inherits!(PlayerDataService<I>, Instance<I>);
impl<I: Default> Default for PlayerDataService<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct PlayerEmulatorService<I> {
    superclass: Instance<I>,
    pub CustomPoliciesEnabled: bool,
    pub EmulatedCountryCode: String,
    pub EmulatedGameLocale: String,
    pub PlayerEmulationEnabled: bool,
    pub PseudolocalizationEnabled: bool,
    pub SerializedEmulatedPolicyInfo: BinaryString,
    pub TextElongationFactor: i32,
}
impl_inherits!(PlayerEmulatorService<I>, Instance<I>);
impl<I: Default> Default for PlayerEmulatorService<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct PlayerGui<I> {
    superclass: BasePlayerGui<I>,
    pub ScreenOrientation: enums::ScreenOrientation,
    pub SelectionImageObject: Ref,
}
impl_inherits!(PlayerGui<I>, BasePlayerGui<I>);
impl<I: Default> Default for PlayerGui<I> {
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
pub struct PlayerHydrationService<I> {
    superclass: Instance<I>,
}
impl_inherits!(PlayerHydrationService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlayerMouse<I> {
    superclass: Mouse<I>,
}
impl_inherits!(PlayerMouse<I>, Mouse<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlayerScripts<I> {
    superclass: Instance<I>,
}
impl_inherits!(PlayerScripts<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlayerViewService<I> {
    superclass: Instance<I>,
}
impl_inherits!(PlayerViewService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Players<I> {
    superclass: Instance<I>,
    pub BanningEnabled: bool,
    pub CharacterAutoLoads: bool,
    pub RespawnTime: f32,
    pub UseStrafingAnimations: bool,
}
impl_inherits!(Players<I>, Instance<I>);
impl<I: Default> Default for Players<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Plugin<I> {
    superclass: Instance<I>,
    pub DisableUiDragDetectorDrags: bool,
    pub IsDebuggable: bool,
}
impl_inherits!(Plugin<I>, Instance<I>);
impl<I: Default> Default for Plugin<I> {
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
pub struct PluginAction<I> {
    superclass: Instance<I>,
}
impl_inherits!(PluginAction<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginCapabilities<I> {
    superclass: Instance<I>,
    pub Manifest: String,
}
impl_inherits!(PluginCapabilities<I>, Instance<I>);
impl<I: Default> Default for PluginCapabilities<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct PluginDebugService<I> {
    superclass: Instance<I>,
}
impl_inherits!(PluginDebugService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginDragEvent<I> {
    superclass: Instance<I>,
}
impl_inherits!(PluginDragEvent<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginGui<I> {
    superclass: LayerCollector<I>,
    pub Title: String,
}
impl_inherits!(PluginGui<I>, LayerCollector<I>);
impl<I: Default> Default for PluginGui<I> {
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
pub struct PluginGuiService<I> {
    superclass: Instance<I>,
}
impl_inherits!(PluginGuiService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginManagementService<I> {
    superclass: Instance<I>,
}
impl_inherits!(PluginManagementService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginManager<I> {
    superclass: Instance<I>,
}
impl_inherits!(PluginManager<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginManagerInterface<I> {
    superclass: Instance<I>,
}
impl_inherits!(PluginManagerInterface<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginMenu<I> {
    superclass: Instance<I>,
}
impl_inherits!(PluginMenu<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginMouse<I> {
    superclass: Mouse<I>,
}
impl_inherits!(PluginMouse<I>, Mouse<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginPolicyService<I> {
    superclass: Instance<I>,
}
impl_inherits!(PluginPolicyService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginToolbar<I> {
    superclass: Instance<I>,
}
impl_inherits!(PluginToolbar<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginToolbarButton<I> {
    superclass: Instance<I>,
}
impl_inherits!(PluginToolbarButton<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PointLight<I> {
    superclass: Light<I>,
    pub Range: f32,
}
impl_inherits!(PointLight<I>, Light<I>);
impl<I: Default> Default for PointLight<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct PointsService<I> {
    superclass: Instance<I>,
}
impl_inherits!(PointsService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PolicyService<I> {
    superclass: Instance<I>,
    pub IsLuobuServer: enums::TriStateBoolean,
    pub LuobuWhitelisted: enums::TriStateBoolean,
}
impl_inherits!(PolicyService<I>, Instance<I>);
impl<I: Default> Default for PolicyService<I> {
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
pub struct Pose<I> {
    superclass: PoseBase<I>,
    pub CFrame: CFrame,
}
impl_inherits!(Pose<I>, PoseBase<I>);
impl<I: Default> Default for Pose<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct PoseBase<I> {
    superclass: Instance<I>,
    pub EasingDirection: enums::PoseEasingDirection,
    pub EasingStyle: enums::PoseEasingStyle,
    pub Weight: f32,
}
impl_inherits!(PoseBase<I>, Instance<I>);
impl<I: Default> Default for PoseBase<I> {
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
pub struct PostEffect<I> {
    superclass: Instance<I>,
    pub Enabled: bool,
}
impl_inherits!(PostEffect<I>, Instance<I>);
impl<I: Default> Default for PostEffect<I> {
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
pub struct PrismaticConstraint<I> {
    superclass: SlidingBallConstraint<I>,
}
impl_inherits!(PrismaticConstraint<I>, SlidingBallConstraint<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ProcessInstancePhysicsService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ProcessInstancePhysicsService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ProximityPrompt<I> {
    superclass: Instance<I>,
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
impl_inherits!(ProximityPrompt<I>, Instance<I>);
impl<I: Default> Default for ProximityPrompt<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ProximityPromptService<I> {
    superclass: Instance<I>,
    pub Enabled: bool,
    pub MaxIndicatorsVisible: i32,
    pub MaxPromptsVisible: i32,
}
impl_inherits!(ProximityPromptService<I>, Instance<I>);
impl<I: Default> Default for ProximityPromptService<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct PublishService<I> {
    superclass: Instance<I>,
}
impl_inherits!(PublishService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PyramidHandleAdornment<I> {
    superclass: HandleAdornment<I>,
    pub Height: f32,
    pub Shading: enums::AdornShading,
    pub Sides: i32,
    pub Size: f32,
}
impl_inherits!(PyramidHandleAdornment<I>, HandleAdornment<I>);
impl<I: Default> Default for PyramidHandleAdornment<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct QWidgetPluginGui<I> {
    superclass: PluginGui<I>,
}
impl_inherits!(QWidgetPluginGui<I>, PluginGui<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RTAnimationTracker<I> {
    superclass: Instance<I>,
}
impl_inherits!(RTAnimationTracker<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RayValue<I> {
    superclass: ValueBase<I>,
    pub Value: Ray,
}
impl_inherits!(RayValue<I>, ValueBase<I>);
impl<I: Default> Default for RayValue<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct RbxAnalyticsService<I> {
    superclass: Instance<I>,
}
impl_inherits!(RbxAnalyticsService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RecommendationPages<I> {
    superclass: Pages<I>,
}
impl_inherits!(RecommendationPages<I>, Pages<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RecommendationService<I> {
    superclass: Instance<I>,
}
impl_inherits!(RecommendationService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadata<I> {
    superclass: Instance<I>,
}
impl_inherits!(ReflectionMetadata<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataCallbacks<I> {
    superclass: Instance<I>,
}
impl_inherits!(ReflectionMetadataCallbacks<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataClass<I> {
    superclass: ReflectionMetadataItem<I>,
    pub ExplorerImageIndex: i32,
    pub ExplorerOrder: i32,
    pub Insertable: bool,
    pub PreferredParent: String,
    pub ServiceVisibility: enums::ServiceVisibility,
}
impl_inherits!(ReflectionMetadataClass<I>, ReflectionMetadataItem<I>);
impl<I: Default> Default for ReflectionMetadataClass<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ReflectionMetadataClasses<I> {
    superclass: Instance<I>,
}
impl_inherits!(ReflectionMetadataClasses<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataEnum<I> {
    superclass: ReflectionMetadataItem<I>,
}
impl_inherits!(ReflectionMetadataEnum<I>, ReflectionMetadataItem<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataEnumItem<I> {
    superclass: ReflectionMetadataItem<I>,
}
impl_inherits!(ReflectionMetadataEnumItem<I>, ReflectionMetadataItem<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataEnums<I> {
    superclass: Instance<I>,
}
impl_inherits!(ReflectionMetadataEnums<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataEvents<I> {
    superclass: Instance<I>,
}
impl_inherits!(ReflectionMetadataEvents<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataFunctions<I> {
    superclass: Instance<I>,
}
impl_inherits!(ReflectionMetadataFunctions<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataItem<I> {
    superclass: Instance<I>,
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
impl_inherits!(ReflectionMetadataItem<I>, Instance<I>);
impl<I: Default> Default for ReflectionMetadataItem<I> {
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
pub struct ReflectionMetadataMember<I> {
    superclass: ReflectionMetadataItem<I>,
}
impl_inherits!(ReflectionMetadataMember<I>, ReflectionMetadataItem<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataProperties<I> {
    superclass: Instance<I>,
}
impl_inherits!(ReflectionMetadataProperties<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataYieldFunctions<I> {
    superclass: Instance<I>,
}
impl_inherits!(ReflectionMetadataYieldFunctions<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ReflectionService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RelativeGui<I> {
    superclass: GuiObject<I>,
}
impl_inherits!(RelativeGui<I>, GuiObject<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RemoteCommandService<I> {
    superclass: Instance<I>,
}
impl_inherits!(RemoteCommandService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RemoteCursorService<I> {
    superclass: Instance<I>,
}
impl_inherits!(RemoteCursorService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RemoteDebuggerServer<I> {
    superclass: Instance<I>,
}
impl_inherits!(RemoteDebuggerServer<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RemoteEvent<I> {
    superclass: BaseRemoteEvent<I>,
}
impl_inherits!(RemoteEvent<I>, BaseRemoteEvent<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RemoteFunction<I> {
    superclass: Instance<I>,
}
impl_inherits!(RemoteFunction<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RenderSettings<I> {
    superclass: Instance<I>,
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
impl_inherits!(RenderSettings<I>, Instance<I>);
impl<I: Default> Default for RenderSettings<I> {
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
pub struct RenderingTest<I> {
    superclass: Instance<I>,
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
impl_inherits!(RenderingTest<I>, Instance<I>);
impl<I: Default> Default for RenderingTest<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ReplicatedFirst<I> {
    superclass: Instance<I>,
}
impl_inherits!(ReplicatedFirst<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReplicatedStorage<I> {
    superclass: Instance<I>,
}
impl_inherits!(ReplicatedStorage<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReverbSoundEffect<I> {
    superclass: SoundEffect<I>,
    pub DecayTime: f32,
    pub Density: f32,
    pub Diffusion: f32,
    pub DryLevel: f32,
    pub WetLevel: f32,
}
impl_inherits!(ReverbSoundEffect<I>, SoundEffect<I>);
impl<I: Default> Default for ReverbSoundEffect<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct RibbonNotificationService<I> {
    superclass: Instance<I>,
}
impl_inherits!(RibbonNotificationService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RigidConstraint<I> {
    superclass: Constraint<I>,
}
impl_inherits!(RigidConstraint<I>, Constraint<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RobloxPluginGuiService<I> {
    superclass: Instance<I>,
}
impl_inherits!(RobloxPluginGuiService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RobloxReplicatedStorage<I> {
    superclass: Instance<I>,
}
impl_inherits!(RobloxReplicatedStorage<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RobloxSerializableInstance<I> {
    superclass: Instance<I>,
    pub Data: BinaryString,
}
impl_inherits!(RobloxSerializableInstance<I>, Instance<I>);
impl<I: Default> Default for RobloxSerializableInstance<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct RobloxServerStorage<I> {
    superclass: Instance<I>,
}
impl_inherits!(RobloxServerStorage<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RocketPropulsion<I> {
    superclass: BodyMover<I>,
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
impl_inherits!(RocketPropulsion<I>, BodyMover<I>);
impl<I: Default> Default for RocketPropulsion<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct RodConstraint<I> {
    superclass: Constraint<I>,
    pub Length: f32,
    pub LimitAngle0: f32,
    pub LimitAngle1: f32,
    pub LimitsEnabled: bool,
    pub Thickness: f32,
}
impl_inherits!(RodConstraint<I>, Constraint<I>);
impl<I: Default> Default for RodConstraint<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct RomarkRbxAnalyticsService<I> {
    superclass: Instance<I>,
}
impl_inherits!(RomarkRbxAnalyticsService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RomarkService<I> {
    superclass: Instance<I>,
}
impl_inherits!(RomarkService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RootImportData<I> {
    superclass: BaseImportData<I>,
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
impl_inherits!(RootImportData<I>, BaseImportData<I>);
impl<I: Default> Default for RootImportData<I> {
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
pub struct RopeConstraint<I> {
    superclass: Constraint<I>,
    pub Length: f32,
    pub Restitution: f32,
    pub Thickness: f32,
    pub WinchEnabled: bool,
    pub WinchForce: f32,
    pub WinchResponsiveness: f32,
    pub WinchSpeed: f32,
    pub WinchTarget: f32,
}
impl_inherits!(RopeConstraint<I>, Constraint<I>);
impl<I: Default> Default for RopeConstraint<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Rotate<I> {
    superclass: JointInstance<I>,
}
impl_inherits!(Rotate<I>, JointInstance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RotateP<I> {
    superclass: DynamicRotate<I>,
}
impl_inherits!(RotateP<I>, DynamicRotate<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RotateV<I> {
    superclass: DynamicRotate<I>,
}
impl_inherits!(RotateV<I>, DynamicRotate<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RotationCurve<I> {
    superclass: Instance<I>,
    pub ValuesAndTimes: BinaryString,
}
impl_inherits!(RotationCurve<I>, Instance<I>);
impl<I: Default> Default for RotationCurve<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct RtMessagingService<I> {
    superclass: Instance<I>,
}
impl_inherits!(RtMessagingService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RunService<I> {
    superclass: Instance<I>,
}
impl_inherits!(RunService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RunningAverageItemDouble<I> {
    superclass: StatsItem<I>,
}
impl_inherits!(RunningAverageItemDouble<I>, StatsItem<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RunningAverageItemInt<I> {
    superclass: StatsItem<I>,
}
impl_inherits!(RunningAverageItemInt<I>, StatsItem<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RunningAverageTimeIntervalItem<I> {
    superclass: StatsItem<I>,
}
impl_inherits!(RunningAverageTimeIntervalItem<I>, StatsItem<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RuntimeContentService<I> {
    superclass: Instance<I>,
}
impl_inherits!(RuntimeContentService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RuntimeScriptService<I> {
    superclass: Instance<I>,
}
impl_inherits!(RuntimeScriptService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SafetyService<I> {
    superclass: Instance<I>,
    pub IsCaptureModeForReport: bool,
}
impl_inherits!(SafetyService<I>, Instance<I>);
impl<I: Default> Default for SafetyService<I> {
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
pub struct ScreenGui<I> {
    superclass: LayerCollector<I>,
    pub ClipToDeviceSafeArea: bool,
    pub DisplayOrder: i32,
    pub SafeAreaCompatibility: enums::SafeAreaCompatibility,
    pub ScreenInsets: enums::ScreenInsets,
}
impl_inherits!(ScreenGui<I>, LayerCollector<I>);
impl<I: Default> Default for ScreenGui<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ScreenshotCapture<I> {
    superclass: Capture<I>,
}
impl_inherits!(ScreenshotCapture<I>, Capture<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScreenshotHud<I> {
    superclass: Instance<I>,
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
impl_inherits!(ScreenshotHud<I>, Instance<I>);
impl<I: Default> Default for ScreenshotHud<I> {
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
pub struct Script<I> {
    superclass: BaseScript<I>,
    pub Source: String,
}
impl_inherits!(Script<I>, BaseScript<I>);
impl<I: Default> Default for Script<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ScriptBuilder<I> {
    superclass: Instance<I>,
}
impl_inherits!(ScriptBuilder<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptChangeService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ScriptChangeService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptCloneWatcher<I> {
    superclass: Instance<I>,
}
impl_inherits!(ScriptCloneWatcher<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptCloneWatcherHelper<I> {
    superclass: Instance<I>,
}
impl_inherits!(ScriptCloneWatcherHelper<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptCommitService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ScriptCommitService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptContext<I> {
    superclass: Instance<I>,
}
impl_inherits!(ScriptContext<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptDebugger<I> {
    superclass: Instance<I>,
    pub CoreScriptIdentifier: String,
    pub ScriptGuid: String,
}
impl_inherits!(ScriptDebugger<I>, Instance<I>);
impl<I: Default> Default for ScriptDebugger<I> {
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
pub struct ScriptDocument<I> {
    superclass: Instance<I>,
}
impl_inherits!(ScriptDocument<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptEditorService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ScriptEditorService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptProfilerService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ScriptProfilerService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptRegistrationService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ScriptRegistrationService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptRuntime<I> {
    superclass: Instance<I>,
}
impl_inherits!(ScriptRuntime<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ScriptService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScrollingFrame<I> {
    superclass: GuiObject<I>,
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
impl_inherits!(ScrollingFrame<I>, GuiObject<I>);
impl<I: Default> Default for ScrollingFrame<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Seat<I> {
    superclass: Part<I>,
    pub Disabled: bool,
}
impl_inherits!(Seat<I>, Part<I>);
impl<I: Default> Default for Seat<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Selection<I> {
    superclass: Instance<I>,
}
impl_inherits!(Selection<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionBox<I> {
    superclass: InstanceAdornment<I>,
    pub LineThickness: f32,
    pub StudioSelectionBox: bool,
    pub SurfaceColor3: Color3,
    pub SurfaceTransparency: f32,
}
impl_inherits!(SelectionBox<I>, InstanceAdornment<I>);
impl<I: Default> Default for SelectionBox<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SelectionHighlightManager<I> {
    superclass: Instance<I>,
}
impl_inherits!(SelectionHighlightManager<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionLasso<I> {
    superclass: GuiBase3d<I>,
    pub Humanoid: Ref,
}
impl_inherits!(SelectionLasso<I>, GuiBase3d<I>);
impl<I: Default> Default for SelectionLasso<I> {
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
pub struct SelectionPartLasso<I> {
    superclass: SelectionLasso<I>,
    pub Part: Ref,
}
impl_inherits!(SelectionPartLasso<I>, SelectionLasso<I>);
impl<I: Default> Default for SelectionPartLasso<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SelectionPointLasso<I> {
    superclass: SelectionLasso<I>,
    pub Point: Vector3,
}
impl_inherits!(SelectionPointLasso<I>, SelectionLasso<I>);
impl<I: Default> Default for SelectionPointLasso<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SelectionSphere<I> {
    superclass: PVAdornment<I>,
    pub SurfaceColor3: Color3,
    pub SurfaceTransparency: f32,
}
impl_inherits!(SelectionSphere<I>, PVAdornment<I>);
impl<I: Default> Default for SelectionSphere<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SensorBase<I> {
    superclass: Instance<I>,
    pub UpdateType: enums::SensorUpdateType,
}
impl_inherits!(SensorBase<I>, Instance<I>);
impl<I: Default> Default for SensorBase<I> {
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
pub struct SerializationService<I> {
    superclass: Instance<I>,
}
impl_inherits!(SerializationService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ServerReplicator<I> {
    superclass: NetworkReplicator<I>,
}
impl_inherits!(ServerReplicator<I>, NetworkReplicator<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ServerScriptService<I> {
    superclass: Instance<I>,
    pub LoadStringEnabled: bool,
}
impl_inherits!(ServerScriptService<I>, Instance<I>);
impl<I: Default> Default for ServerScriptService<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ServerStorage<I> {
    superclass: Instance<I>,
}
impl_inherits!(ServerStorage<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ServiceProvider<I> {
    superclass: Instance<I>,
}
impl_inherits!(ServiceProvider<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ServiceVisibilityService<I> {
    superclass: Instance<I>,
    pub HiddenServices: BinaryString,
    pub VisibleServices: BinaryString,
}
impl_inherits!(ServiceVisibilityService<I>, Instance<I>);
impl<I: Default> Default for ServiceVisibilityService<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SessionCheckService<I> {
    superclass: Instance<I>,
}
impl_inherits!(SessionCheckService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SessionService<I> {
    superclass: Instance<I>,
}
impl_inherits!(SessionService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SharedTableRegistry<I> {
    superclass: Instance<I>,
}
impl_inherits!(SharedTableRegistry<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Shirt<I> {
    superclass: Clothing<I>,
    pub ShirtTemplate: ContentId,
}
impl_inherits!(Shirt<I>, Clothing<I>);
impl<I: Default> Default for Shirt<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct ShirtGraphic<I> {
    superclass: CharacterAppearance<I>,
    pub Color3: Color3,
    pub Graphic: ContentId,
}
impl_inherits!(ShirtGraphic<I>, CharacterAppearance<I>);
impl<I: Default> Default for ShirtGraphic<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SkateboardController<I> {
    superclass: Controller<I>,
}
impl_inherits!(SkateboardController<I>, Controller<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SkateboardPlatform<I> {
    superclass: Part<I>,
    pub Steer: i32,
    pub StickyWheels: bool,
    pub Throttle: i32,
}
impl_inherits!(SkateboardPlatform<I>, Part<I>);
impl<I: Default> Default for SkateboardPlatform<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Skin<I> {
    superclass: CharacterAppearance<I>,
    pub SkinColor: BrickColor,
}
impl_inherits!(Skin<I>, CharacterAppearance<I>);
impl<I: Default> Default for Skin<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Sky<I> {
    superclass: Instance<I>,
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
impl_inherits!(Sky<I>, Instance<I>);
impl<I: Default> Default for Sky<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SlidingBallConstraint<I> {
    superclass: Constraint<I>,
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
impl_inherits!(SlidingBallConstraint<I>, Constraint<I>);
impl<I: Default> Default for SlidingBallConstraint<I> {
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
pub struct SlimContentProvider<I> {
    superclass: CacheableContentProvider<I>,
}
impl_inherits!(SlimContentProvider<I>, CacheableContentProvider<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SlimService<I> {
    superclass: Instance<I>,
}
impl_inherits!(SlimService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Smoke<I> {
    superclass: Instance<I>,
    pub Color: Color3,
    pub Enabled: bool,
    pub TimeScale: f32,
}
impl_inherits!(Smoke<I>, Instance<I>);
impl<I: Default> Default for Smoke<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SmoothVoxelsUpgraderService<I> {
    superclass: Instance<I>,
}
impl_inherits!(SmoothVoxelsUpgraderService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Snap<I> {
    superclass: JointInstance<I>,
}
impl_inherits!(Snap<I>, JointInstance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SnippetService<I> {
    superclass: Instance<I>,
}
impl_inherits!(SnippetService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SocialService<I> {
    superclass: Instance<I>,
}
impl_inherits!(SocialService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SolidModelContentProvider<I> {
    superclass: CacheableContentProvider<I>,
}
impl_inherits!(SolidModelContentProvider<I>, CacheableContentProvider<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Sound<I> {
    superclass: Instance<I>,
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
impl_inherits!(Sound<I>, Instance<I>);
impl<I: Default> Default for Sound<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SoundEffect<I> {
    superclass: Instance<I>,
    pub Enabled: bool,
    pub Priority: i32,
}
impl_inherits!(SoundEffect<I>, Instance<I>);
impl<I: Default> Default for SoundEffect<I> {
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
pub struct SoundGroup<I> {
    superclass: Instance<I>,
    pub Volume: f32,
}
impl_inherits!(SoundGroup<I>, Instance<I>);
impl<I: Default> Default for SoundGroup<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SoundService<I> {
    superclass: Instance<I>,
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
impl_inherits!(SoundService<I>, Instance<I>);
impl<I: Default> Default for SoundService<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SoundShimService<I> {
    superclass: Instance<I>,
}
impl_inherits!(SoundShimService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Sparkles<I> {
    superclass: Instance<I>,
    pub Enabled: bool,
    pub SparkleColor: Color3,
    pub TimeScale: f32,
}
impl_inherits!(Sparkles<I>, Instance<I>);
impl<I: Default> Default for Sparkles<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SpawnLocation<I> {
    superclass: Part<I>,
    pub AllowTeamChangeOnTouch: bool,
    pub Duration: i32,
    pub Enabled: bool,
    pub Neutral: bool,
    pub TeamColor: BrickColor,
}
impl_inherits!(SpawnLocation<I>, Part<I>);
impl<I: Default> Default for SpawnLocation<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SpawnerService<I> {
    superclass: Instance<I>,
}
impl_inherits!(SpawnerService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SpecialMesh<I> {
    superclass: FileMesh<I>,
    pub MeshType: enums::MeshType,
}
impl_inherits!(SpecialMesh<I>, FileMesh<I>);
impl<I: Default> Default for SpecialMesh<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SphereHandleAdornment<I> {
    superclass: HandleAdornment<I>,
    pub Radius: f32,
    pub Shading: enums::AdornShading,
}
impl_inherits!(SphereHandleAdornment<I>, HandleAdornment<I>);
impl<I: Default> Default for SphereHandleAdornment<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SpotLight<I> {
    superclass: Light<I>,
    pub Angle: f32,
    pub Face: enums::NormalId,
    pub Range: f32,
}
impl_inherits!(SpotLight<I>, Light<I>);
impl<I: Default> Default for SpotLight<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SpringConstraint<I> {
    superclass: Constraint<I>,
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
impl_inherits!(SpringConstraint<I>, Constraint<I>);
impl<I: Default> Default for SpringConstraint<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct StackFrame<I> {
    superclass: Instance<I>,
}
impl_inherits!(StackFrame<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StandalonePluginScripts<I> {
    superclass: Instance<I>,
}
impl_inherits!(StandalonePluginScripts<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StandardPages<I> {
    superclass: Pages<I>,
}
impl_inherits!(StandardPages<I>, Pages<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StartPageService<I> {
    superclass: Instance<I>,
}
impl_inherits!(StartPageService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StarterCharacterScripts<I> {
    superclass: StarterPlayerScripts<I>,
}
impl_inherits!(StarterCharacterScripts<I>, StarterPlayerScripts<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StarterGear<I> {
    superclass: Instance<I>,
}
impl_inherits!(StarterGear<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StarterGui<I> {
    superclass: BasePlayerGui<I>,
    pub ResetPlayerGuiOnSpawn: bool,
    pub RtlTextSupport: enums::RtlTextSupport,
    pub ScreenOrientation: enums::ScreenOrientation,
    pub ShowDevelopmentGui: bool,
    pub StudioDefaultStyleSheet: Ref,
    pub StudioInsertWidgetLayerCollectorAutoLinkStyleSheet: Ref,
    pub VirtualCursorMode: enums::VirtualCursorMode,
}
impl_inherits!(StarterGui<I>, BasePlayerGui<I>);
impl<I: Default> Default for StarterGui<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct StarterPack<I> {
    superclass: Instance<I>,
}
impl_inherits!(StarterPack<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StarterPlayer<I> {
    superclass: Instance<I>,
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
impl_inherits!(StarterPlayer<I>, Instance<I>);
impl<I: Default> Default for StarterPlayer<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct StarterPlayerScripts<I> {
    superclass: Instance<I>,
}
impl_inherits!(StarterPlayerScripts<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StartupMessageService<I> {
    superclass: Instance<I>,
}
impl_inherits!(StartupMessageService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Stats<I> {
    superclass: Instance<I>,
}
impl_inherits!(Stats<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StatsItem<I> {
    superclass: Instance<I>,
}
impl_inherits!(StatsItem<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Status<I> {
    superclass: Model<I>,
}
impl_inherits!(Status<I>, Model<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StopWatchReporter<I> {
    superclass: Instance<I>,
}
impl_inherits!(StopWatchReporter<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StreamingService<I> {
    superclass: Instance<I>,
}
impl_inherits!(StreamingService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StringValue<I> {
    superclass: ValueBase<I>,
    pub Value: String,
}
impl_inherits!(StringValue<I>, ValueBase<I>);
impl<I: Default> Default for StringValue<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Studio<I> {
    superclass: Instance<I>,
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
impl_inherits!(Studio<I>, Instance<I>);
impl<I: Default> Default for Studio<I> {
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
pub struct StudioAssetService<I> {
    superclass: Instance<I>,
}
impl_inherits!(StudioAssetService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioAttachment<I> {
    superclass: Instance<I>,
    pub AutoHideParent: bool,
    pub IsArrowVisible: bool,
    pub Offset: Vector2,
    pub SourceAnchorPoint: Vector2,
    pub TargetAnchorPoint: Vector2,
}
impl_inherits!(StudioAttachment<I>, Instance<I>);
impl<I: Default> Default for StudioAttachment<I> {
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
pub struct StudioCallout<I> {
    superclass: Instance<I>,
}
impl_inherits!(StudioCallout<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioCameraService<I> {
    superclass: Instance<I>,
    pub LockCameraSpeed: bool,
    pub LoggingEnabled: bool,
}
impl_inherits!(StudioCameraService<I>, Instance<I>);
impl<I: Default> Default for StudioCameraService<I> {
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
pub struct StudioData<I> {
    superclass: Instance<I>,
    pub EnableScriptCollabByDefaultOnLoad: bool,
}
impl_inherits!(StudioData<I>, Instance<I>);
impl<I: Default> Default for StudioData<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct StudioDeviceEmulatorService<I> {
    superclass: Instance<I>,
}
impl_inherits!(StudioDeviceEmulatorService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioObjectBase<I> {
    superclass: Instance<I>,
}
impl_inherits!(StudioObjectBase<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioPublishService<I> {
    superclass: Instance<I>,
    pub PublishLocked: bool,
}
impl_inherits!(StudioPublishService<I>, Instance<I>);
impl<I: Default> Default for StudioPublishService<I> {
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
pub struct StudioScriptDebugEventListener<I> {
    superclass: Instance<I>,
}
impl_inherits!(StudioScriptDebugEventListener<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioSdkService<I> {
    superclass: Instance<I>,
}
impl_inherits!(StudioSdkService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioService<I> {
    superclass: Instance<I>,
    pub Secrets: String,
}
impl_inherits!(StudioService<I>, Instance<I>);
impl<I: Default> Default for StudioService<I> {
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
pub struct StudioTestService<I> {
    superclass: Instance<I>,
}
impl_inherits!(StudioTestService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioTheme<I> {
    superclass: Instance<I>,
}
impl_inherits!(StudioTheme<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioUserService<I> {
    superclass: Instance<I>,
}
impl_inherits!(StudioUserService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioWidget<I> {
    superclass: StudioObjectBase<I>,
}
impl_inherits!(StudioWidget<I>, StudioObjectBase<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioWidgetsService<I> {
    superclass: Instance<I>,
}
impl_inherits!(StudioWidgetsService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StyleBase<I> {
    superclass: Instance<I>,
}
impl_inherits!(StyleBase<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StyleDerive<I> {
    superclass: Instance<I>,
    pub Priority: i32,
    pub StyleSheet: Ref,
}
impl_inherits!(StyleDerive<I>, Instance<I>);
impl<I: Default> Default for StyleDerive<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct StyleLink<I> {
    superclass: Instance<I>,
    pub StyleSheet: Ref,
}
impl_inherits!(StyleLink<I>, Instance<I>);
impl<I: Default> Default for StyleLink<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct StyleQuery<I> {
    superclass: Instance<I>,
    pub AspectRatioRange: NumberRange,
    pub ConditionsSerialize: BinaryString,
    pub MaxSize: Vector2,
    pub MinSize: Vector2,
}
impl_inherits!(StyleQuery<I>, Instance<I>);
impl<I: Default> Default for StyleQuery<I> {
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
pub struct StyleRule<I> {
    superclass: StyleBase<I>,
    pub Priority: i32,
    pub Selector: String,
}
impl_inherits!(StyleRule<I>, StyleBase<I>);
impl<I: Default> Default for StyleRule<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct StyleSheet<I> {
    superclass: StyleBase<I>,
}
impl_inherits!(StyleSheet<I>, StyleBase<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StylingService<I> {
    superclass: Instance<I>,
}
impl_inherits!(StylingService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SunRaysEffect<I> {
    superclass: PostEffect<I>,
    pub Intensity: f32,
    pub Spread: f32,
}
impl_inherits!(SunRaysEffect<I>, PostEffect<I>);
impl<I: Default> Default for SunRaysEffect<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SurfaceAppearance<I> {
    superclass: Instance<I>,
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
impl_inherits!(SurfaceAppearance<I>, Instance<I>);
impl<I: Default> Default for SurfaceAppearance<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SurfaceGui<I> {
    superclass: SurfaceGuiBase<I>,
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
impl_inherits!(SurfaceGui<I>, SurfaceGuiBase<I>);
impl<I: Default> Default for SurfaceGui<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SurfaceGuiBase<I> {
    superclass: LayerCollector<I>,
    pub Active: bool,
    pub Adornee: Ref,
    pub Face: enums::NormalId,
}
impl_inherits!(SurfaceGuiBase<I>, LayerCollector<I>);
impl<I: Default> Default for SurfaceGuiBase<I> {
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
pub struct SurfaceLight<I> {
    superclass: Light<I>,
    pub Angle: f32,
    pub Face: enums::NormalId,
    pub Range: f32,
}
impl_inherits!(SurfaceLight<I>, Light<I>);
impl<I: Default> Default for SurfaceLight<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SurfaceSelection<I> {
    superclass: PartAdornment<I>,
    pub TargetSurface: enums::NormalId,
}
impl_inherits!(SurfaceSelection<I>, PartAdornment<I>);
impl<I: Default> Default for SurfaceSelection<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SwimController<I> {
    superclass: ControllerBase<I>,
    pub AccelerationTime: f32,
    pub PitchMaxTorque: f32,
    pub PitchSpeedFactor: f32,
    pub RollMaxTorque: f32,
    pub RollSpeedFactor: f32,
}
impl_inherits!(SwimController<I>, ControllerBase<I>);
impl<I: Default> Default for SwimController<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct SyncScriptBuilder<I> {
    superclass: ScriptBuilder<I>,
    pub CompileTarget: enums::CompileTarget,
    pub CoverageInfo: bool,
    pub DebugInfo: bool,
    pub PackAsSource: bool,
    pub RawBytecode: bool,
}
impl_inherits!(SyncScriptBuilder<I>, ScriptBuilder<I>);
impl<I: Default> Default for SyncScriptBuilder<I> {
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
pub struct SystemThemeService<I> {
    superclass: Instance<I>,
}
impl_inherits!(SystemThemeService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TaskScheduler<I> {
    superclass: Instance<I>,
    pub ThreadPoolConfig: enums::ThreadPoolConfig,
}
impl_inherits!(TaskScheduler<I>, Instance<I>);
impl<I: Default> Default for TaskScheduler<I> {
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
pub struct Team<I> {
    superclass: Instance<I>,
    pub AutoAssignable: bool,
    pub TeamColor: BrickColor,
}
impl_inherits!(Team<I>, Instance<I>);
impl<I: Default> Default for Team<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct TeamCreateData<I> {
    superclass: Instance<I>,
}
impl_inherits!(TeamCreateData<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TeamCreatePublishService<I> {
    superclass: Instance<I>,
}
impl_inherits!(TeamCreatePublishService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TeamCreateService<I> {
    superclass: Instance<I>,
}
impl_inherits!(TeamCreateService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Teams<I> {
    superclass: Instance<I>,
}
impl_inherits!(Teams<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TelemetryService<I> {
    superclass: Instance<I>,
}
impl_inherits!(TelemetryService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TeleportAsyncResult<I> {
    superclass: Instance<I>,
}
impl_inherits!(TeleportAsyncResult<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TeleportOptions<I> {
    superclass: Instance<I>,
    pub ReservedServerAccessCode: String,
    pub ServerInstanceId: String,
    pub ShouldReserveServer: bool,
}
impl_inherits!(TeleportOptions<I>, Instance<I>);
impl<I: Default> Default for TeleportOptions<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct TeleportService<I> {
    superclass: Instance<I>,
}
impl_inherits!(TeleportService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TemporaryCageMeshProvider<I> {
    superclass: Instance<I>,
}
impl_inherits!(TemporaryCageMeshProvider<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TemporaryScriptService<I> {
    superclass: Instance<I>,
}
impl_inherits!(TemporaryScriptService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Terrain<I> {
    superclass: BasePart<I>,
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
impl_inherits!(Terrain<I>, BasePart<I>);
impl<I: Default> Default for Terrain<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct TerrainDetail<I> {
    superclass: Instance<I>,
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
impl_inherits!(TerrainDetail<I>, Instance<I>);
impl<I: Default> Default for TerrainDetail<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct TerrainIterateOperation<I> {
    superclass: Object<I>,
}
impl_inherits!(TerrainIterateOperation<I>, Object<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TerrainModifyOperation<I> {
    superclass: Object<I>,
}
impl_inherits!(TerrainModifyOperation<I>, Object<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TerrainReadOperation<I> {
    superclass: Object<I>,
}
impl_inherits!(TerrainReadOperation<I>, Object<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TerrainRegion<I> {
    superclass: Instance<I>,
    pub ExtentsMax: Vector3int16,
    pub ExtentsMin: Vector3int16,
    pub SmoothGrid: BinaryString,
}
impl_inherits!(TerrainRegion<I>, Instance<I>);
impl<I: Default> Default for TerrainRegion<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct TerrainWriteOperation<I> {
    superclass: Object<I>,
}
impl_inherits!(TerrainWriteOperation<I>, Object<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TestService<I> {
    superclass: Instance<I>,
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
impl_inherits!(TestService<I>, Instance<I>);
impl<I: Default> Default for TestService<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct TextBox<I> {
    superclass: GuiObject<I>,
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
impl_inherits!(TextBox<I>, GuiObject<I>);
impl<I: Default> Default for TextBox<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct TextBoxService<I> {
    superclass: Instance<I>,
}
impl_inherits!(TextBoxService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextButton<I> {
    superclass: GuiButton<I>,
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
impl_inherits!(TextButton<I>, GuiButton<I>);
impl<I: Default> Default for TextButton<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct TextChannel<I> {
    superclass: Instance<I>,
}
impl_inherits!(TextChannel<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextChatCommand<I> {
    superclass: Instance<I>,
    pub AutocompleteVisible: bool,
    pub Enabled: bool,
    pub PrimaryAlias: String,
    pub SecondaryAlias: String,
}
impl_inherits!(TextChatCommand<I>, Instance<I>);
impl<I: Default> Default for TextChatCommand<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct TextChatConfigurations<I> {
    superclass: Instance<I>,
}
impl_inherits!(TextChatConfigurations<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextChatMessage<I> {
    superclass: Instance<I>,
    pub BubbleChatMessageProperties: Ref,
    pub ChatWindowMessageProperties: Ref,
    pub TextChannel: Ref,
    pub TextSource: Ref,
}
impl_inherits!(TextChatMessage<I>, Instance<I>);
impl<I: Default> Default for TextChatMessage<I> {
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
pub struct TextChatMessageProperties<I> {
    superclass: Instance<I>,
}
impl_inherits!(TextChatMessageProperties<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextChatService<I> {
    superclass: Instance<I>,
    pub ChatTranslationFtuxShown: bool,
    pub ChatTranslationToggleEnabled: bool,
    pub ChatVersion: enums::ChatVersion,
    pub CreateDefaultCommands: bool,
    pub CreateDefaultTextChannels: bool,
    pub HasSeenDeprecationDialog: bool,
    pub IsLegacyChatDisabled: bool,
}
impl_inherits!(TextChatService<I>, Instance<I>);
impl<I: Default> Default for TextChatService<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct TextFilterResult<I> {
    superclass: Instance<I>,
}
impl_inherits!(TextFilterResult<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextFilterTranslatedResult<I> {
    superclass: Instance<I>,
}
impl_inherits!(TextFilterTranslatedResult<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextGenerator<I> {
    superclass: Instance<I>,
    pub Seed: i32,
    pub SystemPrompt: String,
    pub Temperature: f32,
    pub TopP: f32,
}
impl_inherits!(TextGenerator<I>, Instance<I>);
impl<I: Default> Default for TextGenerator<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct TextLabel<I> {
    superclass: GuiLabel<I>,
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
impl_inherits!(TextLabel<I>, GuiLabel<I>);
impl<I: Default> Default for TextLabel<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct TextService<I> {
    superclass: Instance<I>,
}
impl_inherits!(TextService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextSource<I> {
    superclass: Instance<I>,
    pub CanSend: bool,
}
impl_inherits!(TextSource<I>, Instance<I>);
impl<I: Default> Default for TextSource<I> {
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
pub struct Texture<I> {
    superclass: Decal<I>,
    pub OffsetStudsU: f32,
    pub OffsetStudsV: f32,
    pub StudsPerTileU: f32,
    pub StudsPerTileV: f32,
}
impl_inherits!(Texture<I>, Decal<I>);
impl<I: Default> Default for Texture<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct TextureGenerationPartGroup<I> {
    superclass: Instance<I>,
}
impl_inherits!(TextureGenerationPartGroup<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextureGenerationService<I> {
    superclass: Instance<I>,
}
impl_inherits!(TextureGenerationService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextureGenerationUnwrappingRequest<I> {
    superclass: Instance<I>,
}
impl_inherits!(TextureGenerationUnwrappingRequest<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ThirdPartyUserService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ThirdPartyUserService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ThreadState<I> {
    superclass: Instance<I>,
}
impl_inherits!(ThreadState<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TimerService<I> {
    superclass: Instance<I>,
}
impl_inherits!(TimerService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ToastNotificationService<I> {
    superclass: Instance<I>,
}
impl_inherits!(ToastNotificationService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Tool<I> {
    superclass: BackpackItem<I>,
    pub CanBeDropped: bool,
    pub Enabled: bool,
    pub Grip: CFrame,
    pub ManualActivationOnly: bool,
    pub RequiresHandle: bool,
    pub ToolTip: String,
}
impl_inherits!(Tool<I>, BackpackItem<I>);
impl<I: Default> Default for Tool<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Torque<I> {
    superclass: Constraint<I>,
    pub RelativeTo: enums::ActuatorRelativeTo,
    pub Torque: Vector3,
}
impl_inherits!(Torque<I>, Constraint<I>);
impl<I: Default> Default for Torque<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct TorsionSpringConstraint<I> {
    superclass: Constraint<I>,
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
impl_inherits!(TorsionSpringConstraint<I>, Constraint<I>);
impl<I: Default> Default for TorsionSpringConstraint<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct TotalCountTimeIntervalItem<I> {
    superclass: StatsItem<I>,
}
impl_inherits!(TotalCountTimeIntervalItem<I>, StatsItem<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TouchInputService<I> {
    superclass: Instance<I>,
}
impl_inherits!(TouchInputService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TouchTransmitter<I> {
    superclass: Instance<I>,
}
impl_inherits!(TouchTransmitter<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TracerService<I> {
    superclass: Instance<I>,
}
impl_inherits!(TracerService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TrackerLodController<I> {
    superclass: Instance<I>,
    pub AudioMode: enums::TrackerLodFlagMode,
    pub VideoExtrapolationMode: enums::TrackerExtrapolationFlagMode,
    pub VideoLodMode: enums::TrackerLodValueMode,
    pub VideoMode: enums::TrackerLodFlagMode,
}
impl_inherits!(TrackerLodController<I>, Instance<I>);
impl<I: Default> Default for TrackerLodController<I> {
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
pub struct TrackerStreamAnimation<I> {
    superclass: Instance<I>,
}
impl_inherits!(TrackerStreamAnimation<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Trail<I> {
    superclass: Instance<I>,
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
impl_inherits!(Trail<I>, Instance<I>);
impl<I: Default> Default for Trail<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Translator<I> {
    superclass: Instance<I>,
}
impl_inherits!(Translator<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TremoloSoundEffect<I> {
    superclass: SoundEffect<I>,
    pub Depth: f32,
    pub Duty: f32,
    pub Frequency: f32,
}
impl_inherits!(TremoloSoundEffect<I>, SoundEffect<I>);
impl<I: Default> Default for TremoloSoundEffect<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct TriangleMeshPart<I> {
    superclass: BasePart<I>,
    pub AeroMeshData: SharedString,
    pub FluidFidelityInternal: enums::FluidFidelity,
    pub InertiaMigrated: bool,
    pub PhysicalConfigData: SharedString,
    pub UnscaledCofm: Vector3,
    pub UnscaledVolInertiaDiags: Vector3,
    pub UnscaledVolInertiaOffDiags: Vector3,
    pub UnscaledVolume: f32,
}
impl_inherits!(TriangleMeshPart<I>, BasePart<I>);
impl<I: Default> Default for TriangleMeshPart<I> {
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
pub struct TrussPart<I> {
    superclass: BasePart<I>,
    pub Style: enums::Style,
}
impl_inherits!(TrussPart<I>, BasePart<I>);
impl<I: Default> Default for TrussPart<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct TutorialService<I> {
    superclass: Instance<I>,
}
impl_inherits!(TutorialService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Tween<I> {
    superclass: TweenBase<I>,
}
impl_inherits!(Tween<I>, TweenBase<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TweenBase<I> {
    superclass: Instance<I>,
}
impl_inherits!(TweenBase<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TweenService<I> {
    superclass: Instance<I>,
}
impl_inherits!(TweenService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UGCAvatarService<I> {
    superclass: Instance<I>,
}
impl_inherits!(UGCAvatarService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UGCValidationService<I> {
    superclass: Instance<I>,
}
impl_inherits!(UGCValidationService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIAspectRatioConstraint<I> {
    superclass: UIConstraint<I>,
    pub AspectRatio: f32,
    pub AspectType: enums::AspectType,
    pub DominantAxis: enums::DominantAxis,
}
impl_inherits!(UIAspectRatioConstraint<I>, UIConstraint<I>);
impl<I: Default> Default for UIAspectRatioConstraint<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct UIBase<I> {
    superclass: Instance<I>,
}
impl_inherits!(UIBase<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UIComponent<I> {
    superclass: UIBase<I>,
}
impl_inherits!(UIComponent<I>, UIBase<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UIConstraint<I> {
    superclass: UIComponent<I>,
}
impl_inherits!(UIConstraint<I>, UIComponent<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UICorner<I> {
    superclass: UIComponent<I>,
    pub CornerRadius: UDim,
}
impl_inherits!(UICorner<I>, UIComponent<I>);
impl<I: Default> Default for UICorner<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct UIDragDetector<I> {
    superclass: UIComponent<I>,
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
impl_inherits!(UIDragDetector<I>, UIComponent<I>);
impl<I: Default> Default for UIDragDetector<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct UIDragDetectorService<I> {
    superclass: Instance<I>,
}
impl_inherits!(UIDragDetectorService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIFlexItem<I> {
    superclass: UIComponent<I>,
    pub FlexMode: enums::UIFlexMode,
    pub GrowRatio: f32,
    pub ItemLineAlignment: enums::ItemLineAlignment,
    pub ShrinkRatio: f32,
}
impl_inherits!(UIFlexItem<I>, UIComponent<I>);
impl<I: Default> Default for UIFlexItem<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct UIGradient<I> {
    superclass: UIComponent<I>,
    pub Color: ColorSequence,
    pub Enabled: bool,
    pub Offset: Vector2,
    pub Rotation: f32,
    pub Transparency: NumberSequence,
}
impl_inherits!(UIGradient<I>, UIComponent<I>);
impl<I: Default> Default for UIGradient<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct UIGridLayout<I> {
    superclass: UIGridStyleLayout<I>,
    pub CellPadding: UDim2,
    pub CellSize: UDim2,
    pub FillDirectionMaxCells: i32,
    pub StartCorner: enums::StartCorner,
}
impl_inherits!(UIGridLayout<I>, UIGridStyleLayout<I>);
impl<I: Default> Default for UIGridLayout<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct UIGridStyleLayout<I> {
    superclass: UILayout<I>,
    pub FillDirection: enums::FillDirection,
    pub HorizontalAlignment: enums::HorizontalAlignment,
    pub SortOrder: enums::SortOrder,
    pub VerticalAlignment: enums::VerticalAlignment,
}
impl_inherits!(UIGridStyleLayout<I>, UILayout<I>);
impl<I: Default> Default for UIGridStyleLayout<I> {
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
pub struct UILayout<I> {
    superclass: UIComponent<I>,
}
impl_inherits!(UILayout<I>, UIComponent<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIListLayout<I> {
    superclass: UIGridStyleLayout<I>,
    pub HorizontalFlex: enums::UIFlexAlignment,
    pub ItemLineAlignment: enums::ItemLineAlignment,
    pub Padding: UDim,
    pub VerticalFlex: enums::UIFlexAlignment,
    pub Wraps: bool,
}
impl_inherits!(UIListLayout<I>, UIGridStyleLayout<I>);
impl<I: Default> Default for UIListLayout<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct UIPadding<I> {
    superclass: UIComponent<I>,
    pub PaddingBottom: UDim,
    pub PaddingLeft: UDim,
    pub PaddingRight: UDim,
    pub PaddingTop: UDim,
}
impl_inherits!(UIPadding<I>, UIComponent<I>);
impl<I: Default> Default for UIPadding<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct UIPageLayout<I> {
    superclass: UIGridStyleLayout<I>,
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
impl_inherits!(UIPageLayout<I>, UIGridStyleLayout<I>);
impl<I: Default> Default for UIPageLayout<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct UIScale<I> {
    superclass: UIComponent<I>,
    pub Scale: f32,
}
impl_inherits!(UIScale<I>, UIComponent<I>);
impl<I: Default> Default for UIScale<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct UISizeConstraint<I> {
    superclass: UIConstraint<I>,
    pub MaxSize: Vector2,
    pub MinSize: Vector2,
}
impl_inherits!(UISizeConstraint<I>, UIConstraint<I>);
impl<I: Default> Default for UISizeConstraint<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct UIStroke<I> {
    superclass: UIComponent<I>,
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
impl_inherits!(UIStroke<I>, UIComponent<I>);
impl<I: Default> Default for UIStroke<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct UITableLayout<I> {
    superclass: UIGridStyleLayout<I>,
    pub FillEmptySpaceColumns: bool,
    pub FillEmptySpaceRows: bool,
    pub MajorAxis: enums::TableMajorAxis,
    pub Padding: UDim2,
}
impl_inherits!(UITableLayout<I>, UIGridStyleLayout<I>);
impl<I: Default> Default for UITableLayout<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct UITextSizeConstraint<I> {
    superclass: UIConstraint<I>,
    pub MaxTextSize: i32,
    pub MinTextSize: i32,
}
impl_inherits!(UITextSizeConstraint<I>, UIConstraint<I>);
impl<I: Default> Default for UITextSizeConstraint<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct UnionOperation<I> {
    superclass: PartOperation<I>,
}
impl_inherits!(UnionOperation<I>, PartOperation<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UniqueIdLookupService<I> {
    superclass: Instance<I>,
}
impl_inherits!(UniqueIdLookupService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UniversalConstraint<I> {
    superclass: Constraint<I>,
    pub LimitsEnabled: bool,
    pub MaxAngle: f32,
    pub Radius: f32,
    pub Restitution: f32,
}
impl_inherits!(UniversalConstraint<I>, Constraint<I>);
impl<I: Default> Default for UniversalConstraint<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct UnreliableRemoteEvent<I> {
    superclass: BaseRemoteEvent<I>,
}
impl_inherits!(UnreliableRemoteEvent<I>, BaseRemoteEvent<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UnvalidatedAssetService<I> {
    superclass: Instance<I>,
    pub CachedData: String,
}
impl_inherits!(UnvalidatedAssetService<I>, Instance<I>);
impl<I: Default> Default for UnvalidatedAssetService<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct UserGameSettings<I> {
    superclass: Instance<I>,
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
impl_inherits!(UserGameSettings<I>, Instance<I>);
impl<I: Default> Default for UserGameSettings<I> {
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
pub struct UserInputService<I> {
    superclass: Instance<I>,
    pub LegacyInputEventsEnabled: bool,
    pub MouseBehavior: enums::MouseBehavior,
    pub MouseIconContent: Content,
    pub MouseIconEnabled: bool,
}
impl_inherits!(UserInputService<I>, Instance<I>);
impl<I: Default> Default for UserInputService<I> {
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
pub struct UserService<I> {
    superclass: Instance<I>,
}
impl_inherits!(UserService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UserSettings<I> {
    superclass: GenericSettings<I>,
}
impl_inherits!(UserSettings<I>, GenericSettings<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UserStorageService<I> {
    superclass: LocalStorageService<I>,
}
impl_inherits!(UserStorageService<I>, LocalStorageService<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VRService<I> {
    superclass: Instance<I>,
    pub AutomaticScaling: enums::VRScaling,
    pub AvatarGestures: bool,
    pub ControllerModels: enums::VRControllerModelMode,
    pub FadeOutViewOnCollision: bool,
    pub LaserPointer: enums::VRLaserPointerMode,
}
impl_inherits!(VRService<I>, Instance<I>);
impl<I: Default> Default for VRService<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct VRStatusService<I> {
    superclass: Instance<I>,
}
impl_inherits!(VRStatusService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ValueBase<I> {
    superclass: Instance<I>,
}
impl_inherits!(ValueBase<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ValueCurve<I> {
    superclass: Instance<I>,
    pub ValuesAndTimes: BinaryString,
}
impl_inherits!(ValueCurve<I>, Instance<I>);
impl<I: Default> Default for ValueCurve<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Vector3Curve<I> {
    superclass: Instance<I>,
}
impl_inherits!(Vector3Curve<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Vector3Value<I> {
    superclass: ValueBase<I>,
    pub Value: Vector3,
}
impl_inherits!(Vector3Value<I>, ValueBase<I>);
impl<I: Default> Default for Vector3Value<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct VectorForce<I> {
    superclass: Constraint<I>,
    pub ApplyAtCenterOfMass: bool,
    pub Force: Vector3,
    pub RelativeTo: enums::ActuatorRelativeTo,
}
impl_inherits!(VectorForce<I>, Constraint<I>);
impl<I: Default> Default for VectorForce<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct VehicleController<I> {
    superclass: Controller<I>,
}
impl_inherits!(VehicleController<I>, Controller<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VehicleSeat<I> {
    superclass: BasePart<I>,
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
impl_inherits!(VehicleSeat<I>, BasePart<I>);
impl<I: Default> Default for VehicleSeat<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct VelocityMotor<I> {
    superclass: JointInstance<I>,
    pub CurrentAngle: f32,
    pub DesiredAngle: f32,
    pub Hole: Ref,
    pub MaxVelocity: f32,
}
impl_inherits!(VelocityMotor<I>, JointInstance<I>);
impl<I: Default> Default for VelocityMotor<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct VersionControlService<I> {
    superclass: Instance<I>,
}
impl_inherits!(VersionControlService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VideoCapture<I> {
    superclass: Capture<I>,
}
impl_inherits!(VideoCapture<I>, Capture<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VideoCaptureService<I> {
    superclass: Instance<I>,
}
impl_inherits!(VideoCaptureService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VideoDeviceInput<I> {
    superclass: Instance<I>,
    pub Active: bool,
    pub CameraId: String,
    pub CaptureQuality: enums::VideoDeviceCaptureQuality,
}
impl_inherits!(VideoDeviceInput<I>, Instance<I>);
impl<I: Default> Default for VideoDeviceInput<I> {
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
pub struct VideoDisplay<I> {
    superclass: GuiObject<I>,
    pub ResampleMode: enums::ResamplerMode,
    pub ScaleType: enums::ScaleType,
    pub TileSize: UDim2,
    pub VideoColor3: Color3,
    pub VideoRectOffset: Vector2,
    pub VideoRectSize: Vector2,
    pub VideoTransparency: f32,
}
impl_inherits!(VideoDisplay<I>, GuiObject<I>);
impl<I: Default> Default for VideoDisplay<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct VideoFrame<I> {
    superclass: GuiObject<I>,
    pub Looped: bool,
    pub Playing: bool,
    pub TimePosition: f64,
    pub VideoContent: Content,
    pub Volume: f32,
}
impl_inherits!(VideoFrame<I>, GuiObject<I>);
impl<I: Default> Default for VideoFrame<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct VideoPlayer<I> {
    superclass: Instance<I>,
    pub Looping: bool,
    pub PlaybackSpeed: f32,
    pub TimePosition: f64,
    pub VideoContent: Content,
    pub Volume: f32,
}
impl_inherits!(VideoPlayer<I>, Instance<I>);
impl<I: Default> Default for VideoPlayer<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct VideoSampler<I> {
    superclass: Object<I>,
}
impl_inherits!(VideoSampler<I>, Object<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VideoScreenCaptureService<I> {
    superclass: Instance<I>,
}
impl_inherits!(VideoScreenCaptureService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VideoService<I> {
    superclass: Instance<I>,
}
impl_inherits!(VideoService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ViewportFrame<I> {
    superclass: GuiObject<I>,
    pub Ambient: Color3,
    pub CameraCFrame: CFrame,
    pub CameraFieldOfView: f32,
    pub ImageColor3: Color3,
    pub ImageTransparency: f32,
    pub LightColor: Color3,
    pub LightDirection: Vector3,
}
impl_inherits!(ViewportFrame<I>, GuiObject<I>);
impl<I: Default> Default for ViewportFrame<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct VirtualInputManager<I> {
    superclass: Instance<I>,
}
impl_inherits!(VirtualInputManager<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VirtualUser<I> {
    superclass: Instance<I>,
}
impl_inherits!(VirtualUser<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VisibilityCheckDispatcher<I> {
    superclass: Instance<I>,
}
impl_inherits!(VisibilityCheckDispatcher<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Visit<I> {
    superclass: Instance<I>,
}
impl_inherits!(Visit<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VisualizationMode<I> {
    superclass: Instance<I>,
    pub Enabled: bool,
    pub Title: String,
    pub ToolTip: String,
}
impl_inherits!(VisualizationMode<I>, Instance<I>);
impl<I: Default> Default for VisualizationMode<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct VisualizationModeCategory<I> {
    superclass: Instance<I>,
    pub Enabled: bool,
    pub Title: String,
}
impl_inherits!(VisualizationModeCategory<I>, Instance<I>);
impl<I: Default> Default for VisualizationModeCategory<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct VisualizationModeService<I> {
    superclass: Instance<I>,
}
impl_inherits!(VisualizationModeService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VoiceChatInternal<I> {
    superclass: Instance<I>,
}
impl_inherits!(VoiceChatInternal<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VoiceChatService<I> {
    superclass: Instance<I>,
    pub DefaultDistanceAttenuation: enums::VoiceChatDistanceAttenuationType,
    pub EnableDefaultVoice: bool,
    pub UseAudioApi: enums::AudioApiRollout,
}
impl_inherits!(VoiceChatService<I>, Instance<I>);
impl<I: Default> Default for VoiceChatService<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct WebSocketClient<I> {
    superclass: Instance<I>,
}
impl_inherits!(WebSocketClient<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WebSocketService<I> {
    superclass: Instance<I>,
}
impl_inherits!(WebSocketService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WebStreamClient<I> {
    superclass: Object<I>,
}
impl_inherits!(WebStreamClient<I>, Object<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WebViewService<I> {
    superclass: Instance<I>,
}
impl_inherits!(WebViewService<I>, Instance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WedgePart<I> {
    superclass: FormFactorPart<I>,
}
impl_inherits!(WedgePart<I>, FormFactorPart<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Weld<I> {
    superclass: JointInstance<I>,
}
impl_inherits!(Weld<I>, JointInstance<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WeldConstraint<I> {
    superclass: Instance<I>,
    pub CFrame0: CFrame,
    pub State: i32,
}
impl_inherits!(WeldConstraint<I>, Instance<I>);
impl<I: Default> Default for WeldConstraint<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Wire<I> {
    superclass: Instance<I>,
    pub SourceInstance: Ref,
    pub SourceName: String,
    pub TargetInstance: Ref,
    pub TargetName: String,
}
impl_inherits!(Wire<I>, Instance<I>);
impl<I: Default> Default for Wire<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct WireframeHandleAdornment<I> {
    superclass: HandleAdornment<I>,
    pub Scale: Vector3,
    pub Thickness: f32,
}
impl_inherits!(WireframeHandleAdornment<I>, HandleAdornment<I>);
impl<I: Default> Default for WireframeHandleAdornment<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct Workspace<I> {
    superclass: WorldRoot<I>,
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
impl_inherits!(Workspace<I>, WorldRoot<I>);
impl<I: Default> Default for Workspace<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct WorkspaceAnnotation<I> {
    superclass: Annotation<I>,
}
impl_inherits!(WorkspaceAnnotation<I>, Annotation<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WorldModel<I> {
    superclass: WorldRoot<I>,
}
impl_inherits!(WorldModel<I>, WorldRoot<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WorldRoot<I> {
    superclass: Model<I>,
}
impl_inherits!(WorldRoot<I>, Model<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WrapDeformer<I> {
    superclass: BaseWrap<I>,
}
impl_inherits!(WrapDeformer<I>, BaseWrap<I>);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WrapLayer<I> {
    superclass: BaseWrap<I>,
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
impl_inherits!(WrapLayer<I>, BaseWrap<I>);
impl<I: Default> Default for WrapLayer<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct WrapTarget<I> {
    superclass: BaseWrap<I>,
    pub Stiffness: f32,
}
impl_inherits!(WrapTarget<I>, BaseWrap<I>);
impl<I: Default> Default for WrapTarget<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
pub struct WrapTextureTransfer<I> {
    superclass: Instance<I>,
    pub ReferenceCageMeshContent: Content,
    pub UvMaxBound: Vector2,
    pub UvMinBound: Vector2,
}
impl_inherits!(WrapTextureTransfer<I>, Instance<I>);
impl<I: Default> Default for WrapTextureTransfer<I> {
    fn default() -> Self {
        let superclass = Object::default();
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
