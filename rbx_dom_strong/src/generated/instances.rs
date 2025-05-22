use super::enums;
use crate::impl_inherits;
use core::ops::{Deref, DerefMut};
use rbx_types::*;
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum StrongInstance {
    Accessory(Box<Accessory>),
    AccessoryDescription(Box<AccessoryDescription>),
    AccountService(Box<AccountService>),
    Accoutrement(Box<Accoutrement>),
    AchievementService(Box<AchievementService>),
    ActivityHistoryEventService(Box<ActivityHistoryEventService>),
    Actor(Box<Actor>),
    AdGui(Box<AdGui>),
    AdPortal(Box<AdPortal>),
    AdService(Box<AdService>),
    AdvancedDragger(Box<AdvancedDragger>),
    AirController(Box<AirController>),
    AlignOrientation(Box<AlignOrientation>),
    AlignPosition(Box<AlignPosition>),
    AnalysticsSettings(Box<AnalysticsSettings>),
    AnalyticsService(Box<AnalyticsService>),
    AngularVelocity(Box<AngularVelocity>),
    Animation(Box<Animation>),
    AnimationClip(Box<AnimationClip>),
    AnimationClipProvider(Box<AnimationClipProvider>),
    AnimationConstraint(Box<AnimationConstraint>),
    AnimationController(Box<AnimationController>),
    AnimationFromVideoCreatorService(Box<AnimationFromVideoCreatorService>),
    AnimationFromVideoCreatorStudioService(Box<AnimationFromVideoCreatorStudioService>),
    AnimationImportData(Box<AnimationImportData>),
    AnimationRigData(Box<AnimationRigData>),
    AnimationStreamTrack(Box<AnimationStreamTrack>),
    AnimationTrack(Box<AnimationTrack>),
    Animator(Box<Animator>),
    Annotation(Box<Annotation>),
    AnnotationsService(Box<AnnotationsService>),
    AppLifecycleObserverService(Box<AppLifecycleObserverService>),
    AppStorageService(Box<AppStorageService>),
    AppUpdateService(Box<AppUpdateService>),
    ArcHandles(Box<ArcHandles>),
    AssetCounterService(Box<AssetCounterService>),
    AssetDeliveryProxy(Box<AssetDeliveryProxy>),
    AssetImportService(Box<AssetImportService>),
    AssetImportSession(Box<AssetImportSession>),
    AssetManagerService(Box<AssetManagerService>),
    AssetPatchSettings(Box<AssetPatchSettings>),
    AssetService(Box<AssetService>),
    AssetSoundEffect(Box<AssetSoundEffect>),
    Atmosphere(Box<Atmosphere>),
    AtmosphereSensor(Box<AtmosphereSensor>),
    Attachment(Box<Attachment>),
    AudioAnalyzer(Box<AudioAnalyzer>),
    AudioChannelMixer(Box<AudioChannelMixer>),
    AudioChannelSplitter(Box<AudioChannelSplitter>),
    AudioChorus(Box<AudioChorus>),
    AudioCompressor(Box<AudioCompressor>),
    AudioDeviceInput(Box<AudioDeviceInput>),
    AudioDeviceOutput(Box<AudioDeviceOutput>),
    AudioDistortion(Box<AudioDistortion>),
    AudioEcho(Box<AudioEcho>),
    AudioEmitter(Box<AudioEmitter>),
    AudioEqualizer(Box<AudioEqualizer>),
    AudioFader(Box<AudioFader>),
    AudioFilter(Box<AudioFilter>),
    AudioFlanger(Box<AudioFlanger>),
    AudioFocusService(Box<AudioFocusService>),
    AudioLimiter(Box<AudioLimiter>),
    AudioListener(Box<AudioListener>),
    AudioPages(Box<AudioPages>),
    AudioPitchShifter(Box<AudioPitchShifter>),
    AudioPlayer(Box<AudioPlayer>),
    AudioReverb(Box<AudioReverb>),
    AudioSearchParams(Box<AudioSearchParams>),
    AudioTextToSpeech(Box<AudioTextToSpeech>),
    AuroraScript(Box<AuroraScript>),
    AuroraScriptService(Box<AuroraScriptService>),
    AuroraService(Box<AuroraService>),
    AvatarChatService(Box<AvatarChatService>),
    AvatarCreationService(Box<AvatarCreationService>),
    AvatarEditorService(Box<AvatarEditorService>),
    AvatarImportService(Box<AvatarImportService>),
    AvatarPreloader(Box<AvatarPreloader>),
    Backpack(Box<Backpack>),
    BackpackItem(Box<BackpackItem>),
    BadgeService(Box<BadgeService>),
    BallSocketConstraint(Box<BallSocketConstraint>),
    BanHistoryPages(Box<BanHistoryPages>),
    BaseImportData(Box<BaseImportData>),
    BasePart(Box<BasePart>),
    BasePlayerGui(Box<BasePlayerGui>),
    BaseRemoteEvent(Box<BaseRemoteEvent>),
    BaseScript(Box<BaseScript>),
    BaseWrap(Box<BaseWrap>),
    Beam(Box<Beam>),
    BevelMesh(Box<BevelMesh>),
    BillboardGui(Box<BillboardGui>),
    BinaryStringValue(Box<BinaryStringValue>),
    BindableEvent(Box<BindableEvent>),
    BindableFunction(Box<BindableFunction>),
    BlockMesh(Box<BlockMesh>),
    BloomEffect(Box<BloomEffect>),
    BlurEffect(Box<BlurEffect>),
    BodyAngularVelocity(Box<BodyAngularVelocity>),
    BodyColors(Box<BodyColors>),
    BodyForce(Box<BodyForce>),
    BodyGyro(Box<BodyGyro>),
    BodyMover(Box<BodyMover>),
    BodyPartDescription(Box<BodyPartDescription>),
    BodyPosition(Box<BodyPosition>),
    BodyThrust(Box<BodyThrust>),
    BodyVelocity(Box<BodyVelocity>),
    Bone(Box<Bone>),
    BoolValue(Box<BoolValue>),
    BoxHandleAdornment(Box<BoxHandleAdornment>),
    Breakpoint(Box<Breakpoint>),
    BrickColorValue(Box<BrickColorValue>),
    BrowserService(Box<BrowserService>),
    BubbleChatConfiguration(Box<BubbleChatConfiguration>),
    BubbleChatMessageProperties(Box<BubbleChatMessageProperties>),
    BugReporterService(Box<BugReporterService>),
    BulkImportService(Box<BulkImportService>),
    BuoyancySensor(Box<BuoyancySensor>),
    CFrameValue(Box<CFrameValue>),
    CSGDictionaryService(Box<CSGDictionaryService>),
    CacheableContentProvider(Box<CacheableContentProvider>),
    CalloutService(Box<CalloutService>),
    Camera(Box<Camera>),
    CanvasGroup(Box<CanvasGroup>),
    Capture(Box<Capture>),
    CaptureService(Box<CaptureService>),
    CatalogPages(Box<CatalogPages>),
    ChangeHistoryService(Box<ChangeHistoryService>),
    ChannelSelectorSoundEffect(Box<ChannelSelectorSoundEffect>),
    ChannelTabsConfiguration(Box<ChannelTabsConfiguration>),
    CharacterAppearance(Box<CharacterAppearance>),
    CharacterMesh(Box<CharacterMesh>),
    Chat(Box<Chat>),
    ChatInputBarConfiguration(Box<ChatInputBarConfiguration>),
    ChatWindowConfiguration(Box<ChatWindowConfiguration>),
    ChatWindowMessageProperties(Box<ChatWindowMessageProperties>),
    ChatbotUIService(Box<ChatbotUIService>),
    ChorusSoundEffect(Box<ChorusSoundEffect>),
    ClickDetector(Box<ClickDetector>),
    ClientReplicator(Box<ClientReplicator>),
    ClimbController(Box<ClimbController>),
    Clothing(Box<Clothing>),
    CloudCRUDService(Box<CloudCRUDService>),
    CloudLocalizationTable(Box<CloudLocalizationTable>),
    Clouds(Box<Clouds>),
    ClusterPacketCache(Box<ClusterPacketCache>),
    Collaborator(Box<Collaborator>),
    CollaboratorsService(Box<CollaboratorsService>),
    CollectionService(Box<CollectionService>),
    Color3Value(Box<Color3Value>),
    ColorCorrectionEffect(Box<ColorCorrectionEffect>),
    ColorGradingEffect(Box<ColorGradingEffect>),
    CommandInstance(Box<CommandInstance>),
    CommandService(Box<CommandService>),
    CommerceService(Box<CommerceService>),
    CompressorSoundEffect(Box<CompressorSoundEffect>),
    ConeHandleAdornment(Box<ConeHandleAdornment>),
    ConfigService(Box<ConfigService>),
    ConfigSnapshot(Box<ConfigSnapshot>),
    Configuration(Box<Configuration>),
    ConfigureServerService(Box<ConfigureServerService>),
    ConnectivityService(Box<ConnectivityService>),
    Constraint(Box<Constraint>),
    ContentProvider(Box<ContentProvider>),
    ContextActionService(Box<ContextActionService>),
    Controller(Box<Controller>),
    ControllerBase(Box<ControllerBase>),
    ControllerManager(Box<ControllerManager>),
    ControllerPartSensor(Box<ControllerPartSensor>),
    ControllerSensor(Box<ControllerSensor>),
    ControllerService(Box<ControllerService>),
    ConversationalAIAcceptanceService(Box<ConversationalAIAcceptanceService>),
    CookiesService(Box<CookiesService>),
    CoreGui(Box<CoreGui>),
    CorePackages(Box<CorePackages>),
    CoreScript(Box<CoreScript>),
    CoreScriptDebuggingManagerHelper(Box<CoreScriptDebuggingManagerHelper>),
    CoreScriptSyncService(Box<CoreScriptSyncService>),
    CornerWedgePart(Box<CornerWedgePart>),
    CreationDBService(Box<CreationDBService>),
    CreatorStoreService(Box<CreatorStoreService>),
    CrossDMScriptChangeListener(Box<CrossDMScriptChangeListener>),
    CurveAnimation(Box<CurveAnimation>),
    CustomEvent(Box<CustomEvent>),
    CustomEventReceiver(Box<CustomEventReceiver>),
    CustomLog(Box<CustomLog>),
    CustomSoundEffect(Box<CustomSoundEffect>),
    CylinderHandleAdornment(Box<CylinderHandleAdornment>),
    CylinderMesh(Box<CylinderMesh>),
    CylindricalConstraint(Box<CylindricalConstraint>),
    DataModel(Box<DataModel>),
    DataModelMesh(Box<DataModelMesh>),
    DataModelPatchService(Box<DataModelPatchService>),
    DataModelSession(Box<DataModelSession>),
    DataStore(Box<DataStore>),
    DataStoreGetOptions(Box<DataStoreGetOptions>),
    DataStoreIncrementOptions(Box<DataStoreIncrementOptions>),
    DataStoreInfo(Box<DataStoreInfo>),
    DataStoreKey(Box<DataStoreKey>),
    DataStoreKeyInfo(Box<DataStoreKeyInfo>),
    DataStoreKeyPages(Box<DataStoreKeyPages>),
    DataStoreListingPages(Box<DataStoreListingPages>),
    DataStoreObjectVersionInfo(Box<DataStoreObjectVersionInfo>),
    DataStoreOptions(Box<DataStoreOptions>),
    DataStorePages(Box<DataStorePages>),
    DataStoreService(Box<DataStoreService>),
    DataStoreSetOptions(Box<DataStoreSetOptions>),
    DataStoreVersionPages(Box<DataStoreVersionPages>),
    Debris(Box<Debris>),
    DebugSettings(Box<DebugSettings>),
    DebuggablePluginWatcher(Box<DebuggablePluginWatcher>),
    DebuggerBreakpoint(Box<DebuggerBreakpoint>),
    DebuggerConnection(Box<DebuggerConnection>),
    DebuggerConnectionManager(Box<DebuggerConnectionManager>),
    DebuggerLuaResponse(Box<DebuggerLuaResponse>),
    DebuggerManager(Box<DebuggerManager>),
    DebuggerUIService(Box<DebuggerUIService>),
    DebuggerVariable(Box<DebuggerVariable>),
    DebuggerWatch(Box<DebuggerWatch>),
    Decal(Box<Decal>),
    DepthOfFieldEffect(Box<DepthOfFieldEffect>),
    DeviceIdService(Box<DeviceIdService>),
    Dialog(Box<Dialog>),
    DialogChoice(Box<DialogChoice>),
    DistortionSoundEffect(Box<DistortionSoundEffect>),
    DockWidgetPluginGui(Box<DockWidgetPluginGui>),
    DoubleConstrainedValue(Box<DoubleConstrainedValue>),
    DraftsService(Box<DraftsService>),
    DragDetector(Box<DragDetector>),
    Dragger(Box<Dragger>),
    DraggerService(Box<DraggerService>),
    DynamicRotate(Box<DynamicRotate>),
    EchoSoundEffect(Box<EchoSoundEffect>),
    EditableImage(Box<EditableImage>),
    EditableMesh(Box<EditableMesh>),
    EditableService(Box<EditableService>),
    EmotesPages(Box<EmotesPages>),
    EqualizerSoundEffect(Box<EqualizerSoundEffect>),
    EulerRotationCurve(Box<EulerRotationCurve>),
    EventIngestService(Box<EventIngestService>),
    ExampleService(Box<ExampleService>),
    ExperienceAuthService(Box<ExperienceAuthService>),
    ExperienceInviteOptions(Box<ExperienceInviteOptions>),
    ExperienceNotificationService(Box<ExperienceNotificationService>),
    ExperienceService(Box<ExperienceService>),
    ExperienceStateCaptureService(Box<ExperienceStateCaptureService>),
    ExplorerFilter(Box<ExplorerFilter>),
    ExplorerFilterAutocompleter(Box<ExplorerFilterAutocompleter>),
    ExplorerServiceVisibilityService(Box<ExplorerServiceVisibilityService>),
    Explosion(Box<Explosion>),
    FaceAnimatorService(Box<FaceAnimatorService>),
    FaceControls(Box<FaceControls>),
    FaceInstance(Box<FaceInstance>),
    FacialAgeEstimationService(Box<FacialAgeEstimationService>),
    FacialAnimationRecordingService(Box<FacialAnimationRecordingService>),
    FacialAnimationStreamingServiceStats(Box<FacialAnimationStreamingServiceStats>),
    FacialAnimationStreamingServiceV2(Box<FacialAnimationStreamingServiceV2>),
    FacialAnimationStreamingSubsessionStats(Box<FacialAnimationStreamingSubsessionStats>),
    FacsImportData(Box<FacsImportData>),
    Feature(Box<Feature>),
    FeatureRestrictionManager(Box<FeatureRestrictionManager>),
    FeedPages(Box<FeedPages>),
    FeedService(Box<FeedService>),
    File(Box<File>),
    FileMesh(Box<FileMesh>),
    Fire(Box<Fire>),
    Flag(Box<Flag>),
    FlagStand(Box<FlagStand>),
    FlagStandService(Box<FlagStandService>),
    FlangeSoundEffect(Box<FlangeSoundEffect>),
    FloatCurve(Box<FloatCurve>),
    FloorWire(Box<FloorWire>),
    FluidForceSensor(Box<FluidForceSensor>),
    FlyweightService(Box<FlyweightService>),
    Folder(Box<Folder>),
    ForceField(Box<ForceField>),
    FormFactorPart(Box<FormFactorPart>),
    Frame(Box<Frame>),
    FriendPages(Box<FriendPages>),
    FriendService(Box<FriendService>),
    FunctionalTest(Box<FunctionalTest>),
    GamePassService(Box<GamePassService>),
    GameSettings(Box<GameSettings>),
    GamepadService(Box<GamepadService>),
    GenerationService(Box<GenerationService>),
    GenericChallengeService(Box<GenericChallengeService>),
    GenericSettings(Box<GenericSettings>),
    Geometry(Box<Geometry>),
    GeometryService(Box<GeometryService>),
    GetTextBoundsParams(Box<GetTextBoundsParams>),
    GlobalDataStore(Box<GlobalDataStore>),
    GlobalSettings(Box<GlobalSettings>),
    Glue(Box<Glue>),
    GoogleAnalyticsConfiguration(Box<GoogleAnalyticsConfiguration>),
    GroundController(Box<GroundController>),
    GroupImportData(Box<GroupImportData>),
    GroupService(Box<GroupService>),
    GuiBase(Box<GuiBase>),
    GuiBase2d(Box<GuiBase2d>),
    GuiBase3d(Box<GuiBase3d>),
    GuiButton(Box<GuiButton>),
    GuiLabel(Box<GuiLabel>),
    GuiMain(Box<GuiMain>),
    GuiObject(Box<GuiObject>),
    GuiService(Box<GuiService>),
    GuidRegistryService(Box<GuidRegistryService>),
    HSRDataContentProvider(Box<HSRDataContentProvider>),
    HandleAdornment(Box<HandleAdornment>),
    Handles(Box<Handles>),
    HandlesBase(Box<HandlesBase>),
    HapticEffect(Box<HapticEffect>),
    HapticService(Box<HapticService>),
    Hat(Box<Hat>),
    HeapProfilerService(Box<HeapProfilerService>),
    HeatmapService(Box<HeatmapService>),
    HeightmapImporterService(Box<HeightmapImporterService>),
    HiddenSurfaceRemovalAsset(Box<HiddenSurfaceRemovalAsset>),
    Highlight(Box<Highlight>),
    HingeConstraint(Box<HingeConstraint>),
    Hint(Box<Hint>),
    Hole(Box<Hole>),
    Hopper(Box<Hopper>),
    HopperBin(Box<HopperBin>),
    HttpRbxApiService(Box<HttpRbxApiService>),
    HttpRequest(Box<HttpRequest>),
    HttpService(Box<HttpService>),
    Humanoid(Box<Humanoid>),
    HumanoidController(Box<HumanoidController>),
    HumanoidDescription(Box<HumanoidDescription>),
    HumanoidRigDescription(Box<HumanoidRigDescription>),
    IKControl(Box<IKControl>),
    ILegacyStudioBridge(Box<ILegacyStudioBridge>),
    IXPService(Box<IXPService>),
    ImageButton(Box<ImageButton>),
    ImageHandleAdornment(Box<ImageHandleAdornment>),
    ImageLabel(Box<ImageLabel>),
    ImportSession(Box<ImportSession>),
    IncrementalPatchBuilder(Box<IncrementalPatchBuilder>),
    InputAction(Box<InputAction>),
    InputBinding(Box<InputBinding>),
    InputContext(Box<InputContext>),
    InputObject(Box<InputObject>),
    InsertService(Box<InsertService>),
    Instance(Box<Instance>),
    InstanceAdornment(Box<InstanceAdornment>),
    IntConstrainedValue(Box<IntConstrainedValue>),
    IntValue(Box<IntValue>),
    InternalSyncItem(Box<InternalSyncItem>),
    InternalSyncService(Box<InternalSyncService>),
    IntersectOperation(Box<IntersectOperation>),
    InventoryPages(Box<InventoryPages>),
    JointImportData(Box<JointImportData>),
    JointInstance(Box<JointInstance>),
    JointsService(Box<JointsService>),
    KeyboardService(Box<KeyboardService>),
    Keyframe(Box<Keyframe>),
    KeyframeMarker(Box<KeyframeMarker>),
    KeyframeSequence(Box<KeyframeSequence>),
    KeyframeSequenceProvider(Box<KeyframeSequenceProvider>),
    LSPFileSyncService(Box<LSPFileSyncService>),
    LanguageService(Box<LanguageService>),
    LayerCollector(Box<LayerCollector>),
    LegacyStudioBridge(Box<LegacyStudioBridge>),
    Light(Box<Light>),
    Lighting(Box<Lighting>),
    LineForce(Box<LineForce>),
    LineHandleAdornment(Box<LineHandleAdornment>),
    LinearVelocity(Box<LinearVelocity>),
    LinkingService(Box<LinkingService>),
    LiveScriptingService(Box<LiveScriptingService>),
    LiveSyncService(Box<LiveSyncService>),
    LocalDebuggerConnection(Box<LocalDebuggerConnection>),
    LocalScript(Box<LocalScript>),
    LocalStorageService(Box<LocalStorageService>),
    LocalizationService(Box<LocalizationService>),
    LocalizationTable(Box<LocalizationTable>),
    LodDataEntity(Box<LodDataEntity>),
    LodDataService(Box<LodDataService>),
    LogReporterService(Box<LogReporterService>),
    LogService(Box<LogService>),
    LoginService(Box<LoginService>),
    LuaSettings(Box<LuaSettings>),
    LuaSourceContainer(Box<LuaSourceContainer>),
    LuaWebService(Box<LuaWebService>),
    LuauScriptAnalyzerService(Box<LuauScriptAnalyzerService>),
    MLModelDeliveryService(Box<MLModelDeliveryService>),
    ManualGlue(Box<ManualGlue>),
    ManualSurfaceJointInstance(Box<ManualSurfaceJointInstance>),
    ManualWeld(Box<ManualWeld>),
    MarkerCurve(Box<MarkerCurve>),
    MarketplaceService(Box<MarketplaceService>),
    MatchmakingService(Box<MatchmakingService>),
    MaterialGenerationService(Box<MaterialGenerationService>),
    MaterialGenerationSession(Box<MaterialGenerationSession>),
    MaterialImportData(Box<MaterialImportData>),
    MaterialService(Box<MaterialService>),
    MaterialVariant(Box<MaterialVariant>),
    MemStorageConnection(Box<MemStorageConnection>),
    MemStorageService(Box<MemStorageService>),
    MemoryStoreHashMap(Box<MemoryStoreHashMap>),
    MemoryStoreHashMapPages(Box<MemoryStoreHashMapPages>),
    MemoryStoreQueue(Box<MemoryStoreQueue>),
    MemoryStoreService(Box<MemoryStoreService>),
    MemoryStoreSortedMap(Box<MemoryStoreSortedMap>),
    MeshContentProvider(Box<MeshContentProvider>),
    MeshImportData(Box<MeshImportData>),
    MeshPart(Box<MeshPart>),
    Message(Box<Message>),
    MessageBusConnection(Box<MessageBusConnection>),
    MessageBusService(Box<MessageBusService>),
    MessagingService(Box<MessagingService>),
    MetaBreakpoint(Box<MetaBreakpoint>),
    MetaBreakpointContext(Box<MetaBreakpointContext>),
    MetaBreakpointManager(Box<MetaBreakpointManager>),
    Model(Box<Model>),
    ModuleScript(Box<ModuleScript>),
    Motor(Box<Motor>),
    Motor6D(Box<Motor6D>),
    MotorFeature(Box<MotorFeature>),
    Mouse(Box<Mouse>),
    MouseService(Box<MouseService>),
    MultipleDocumentInterfaceInstance(Box<MultipleDocumentInterfaceInstance>),
    NegateOperation(Box<NegateOperation>),
    NetworkClient(Box<NetworkClient>),
    NetworkMarker(Box<NetworkMarker>),
    NetworkPeer(Box<NetworkPeer>),
    NetworkReplicator(Box<NetworkReplicator>),
    NetworkServer(Box<NetworkServer>),
    NetworkSettings(Box<NetworkSettings>),
    NoCollisionConstraint(Box<NoCollisionConstraint>),
    Noise(Box<Noise>),
    NonReplicatedCSGDictionaryService(Box<NonReplicatedCSGDictionaryService>),
    NotificationService(Box<NotificationService>),
    NumberPose(Box<NumberPose>),
    NumberValue(Box<NumberValue>),
    Object(Box<Object>),
    ObjectValue(Box<ObjectValue>),
    OmniRecommendationsService(Box<OmniRecommendationsService>),
    OpenCloudApiV1(Box<OpenCloudApiV1>),
    OpenCloudService(Box<OpenCloudService>),
    OperationGraph(Box<OperationGraph>),
    OrderedDataStore(Box<OrderedDataStore>),
    OutfitPages(Box<OutfitPages>),
    PVAdornment(Box<PVAdornment>),
    PVInstance(Box<PVInstance>),
    PackageLink(Box<PackageLink>),
    PackageService(Box<PackageService>),
    PackageUIService(Box<PackageUIService>),
    Pages(Box<Pages>),
    Pants(Box<Pants>),
    ParabolaAdornment(Box<ParabolaAdornment>),
    Part(Box<Part>),
    PartAdornment(Box<PartAdornment>),
    PartOperation(Box<PartOperation>),
    PartOperationAsset(Box<PartOperationAsset>),
    ParticleEmitter(Box<ParticleEmitter>),
    PatchBundlerFileWatch(Box<PatchBundlerFileWatch>),
    PatchMapping(Box<PatchMapping>),
    Path(Box<Path>),
    Path2D(Box<Path2D>),
    PathfindingLink(Box<PathfindingLink>),
    PathfindingModifier(Box<PathfindingModifier>),
    PathfindingService(Box<PathfindingService>),
    PausedState(Box<PausedState>),
    PausedStateBreakpoint(Box<PausedStateBreakpoint>),
    PausedStateException(Box<PausedStateException>),
    PerformanceControlService(Box<PerformanceControlService>),
    PermissionsService(Box<PermissionsService>),
    PhysicsService(Box<PhysicsService>),
    PhysicsSettings(Box<PhysicsSettings>),
    PitchShiftSoundEffect(Box<PitchShiftSoundEffect>),
    PlaceAssetIdsService(Box<PlaceAssetIdsService>),
    PlaceStatsService(Box<PlaceStatsService>),
    PlacesService(Box<PlacesService>),
    Plane(Box<Plane>),
    PlaneConstraint(Box<PlaneConstraint>),
    Platform(Box<Platform>),
    PlatformCloudStorageService(Box<PlatformCloudStorageService>),
    PlatformFriendsService(Box<PlatformFriendsService>),
    Player(Box<Player>),
    PlayerData(Box<PlayerData>),
    PlayerDataRecord(Box<PlayerDataRecord>),
    PlayerDataRecordConfig(Box<PlayerDataRecordConfig>),
    PlayerDataService(Box<PlayerDataService>),
    PlayerEmulatorService(Box<PlayerEmulatorService>),
    PlayerGui(Box<PlayerGui>),
    PlayerHydrationService(Box<PlayerHydrationService>),
    PlayerMouse(Box<PlayerMouse>),
    PlayerScripts(Box<PlayerScripts>),
    PlayerViewService(Box<PlayerViewService>),
    Players(Box<Players>),
    Plugin(Box<Plugin>),
    PluginAction(Box<PluginAction>),
    PluginCapabilities(Box<PluginCapabilities>),
    PluginDebugService(Box<PluginDebugService>),
    PluginDragEvent(Box<PluginDragEvent>),
    PluginGui(Box<PluginGui>),
    PluginGuiService(Box<PluginGuiService>),
    PluginManagementService(Box<PluginManagementService>),
    PluginManager(Box<PluginManager>),
    PluginManagerInterface(Box<PluginManagerInterface>),
    PluginMenu(Box<PluginMenu>),
    PluginMouse(Box<PluginMouse>),
    PluginPolicyService(Box<PluginPolicyService>),
    PluginToolbar(Box<PluginToolbar>),
    PluginToolbarButton(Box<PluginToolbarButton>),
    PointLight(Box<PointLight>),
    PointsService(Box<PointsService>),
    PolicyService(Box<PolicyService>),
    Pose(Box<Pose>),
    PoseBase(Box<PoseBase>),
    PostEffect(Box<PostEffect>),
    PrismaticConstraint(Box<PrismaticConstraint>),
    ProcessInstancePhysicsService(Box<ProcessInstancePhysicsService>),
    ProximityPrompt(Box<ProximityPrompt>),
    ProximityPromptService(Box<ProximityPromptService>),
    PublishService(Box<PublishService>),
    QWidgetPluginGui(Box<QWidgetPluginGui>),
    RTAnimationTracker(Box<RTAnimationTracker>),
    RayValue(Box<RayValue>),
    RbxAnalyticsService(Box<RbxAnalyticsService>),
    ReflectionMetadata(Box<ReflectionMetadata>),
    ReflectionMetadataCallbacks(Box<ReflectionMetadataCallbacks>),
    ReflectionMetadataClass(Box<ReflectionMetadataClass>),
    ReflectionMetadataClasses(Box<ReflectionMetadataClasses>),
    ReflectionMetadataEnum(Box<ReflectionMetadataEnum>),
    ReflectionMetadataEnumItem(Box<ReflectionMetadataEnumItem>),
    ReflectionMetadataEnums(Box<ReflectionMetadataEnums>),
    ReflectionMetadataEvents(Box<ReflectionMetadataEvents>),
    ReflectionMetadataFunctions(Box<ReflectionMetadataFunctions>),
    ReflectionMetadataItem(Box<ReflectionMetadataItem>),
    ReflectionMetadataMember(Box<ReflectionMetadataMember>),
    ReflectionMetadataProperties(Box<ReflectionMetadataProperties>),
    ReflectionMetadataYieldFunctions(Box<ReflectionMetadataYieldFunctions>),
    ReflectionService(Box<ReflectionService>),
    RelativeGui(Box<RelativeGui>),
    RemoteCursorService(Box<RemoteCursorService>),
    RemoteDebuggerServer(Box<RemoteDebuggerServer>),
    RemoteEvent(Box<RemoteEvent>),
    RemoteFunction(Box<RemoteFunction>),
    RenderSettings(Box<RenderSettings>),
    RenderingTest(Box<RenderingTest>),
    ReplicatedFirst(Box<ReplicatedFirst>),
    ReplicatedStorage(Box<ReplicatedStorage>),
    ReverbSoundEffect(Box<ReverbSoundEffect>),
    RibbonNotificationService(Box<RibbonNotificationService>),
    RigidConstraint(Box<RigidConstraint>),
    RobloxPluginGuiService(Box<RobloxPluginGuiService>),
    RobloxReplicatedStorage(Box<RobloxReplicatedStorage>),
    RobloxSerializableInstance(Box<RobloxSerializableInstance>),
    RobloxServerStorage(Box<RobloxServerStorage>),
    RocketPropulsion(Box<RocketPropulsion>),
    RodConstraint(Box<RodConstraint>),
    RomarkRbxAnalyticsService(Box<RomarkRbxAnalyticsService>),
    RomarkService(Box<RomarkService>),
    RootImportData(Box<RootImportData>),
    RopeConstraint(Box<RopeConstraint>),
    Rotate(Box<Rotate>),
    RotateP(Box<RotateP>),
    RotateV(Box<RotateV>),
    RotationCurve(Box<RotationCurve>),
    RtMessagingService(Box<RtMessagingService>),
    RunService(Box<RunService>),
    RunningAverageItemDouble(Box<RunningAverageItemDouble>),
    RunningAverageItemInt(Box<RunningAverageItemInt>),
    RunningAverageTimeIntervalItem(Box<RunningAverageTimeIntervalItem>),
    RuntimeScriptService(Box<RuntimeScriptService>),
    SafetyService(Box<SafetyService>),
    ScreenGui(Box<ScreenGui>),
    ScreenshotCapture(Box<ScreenshotCapture>),
    ScreenshotHud(Box<ScreenshotHud>),
    Script(Box<Script>),
    ScriptBuilder(Box<ScriptBuilder>),
    ScriptChangeService(Box<ScriptChangeService>),
    ScriptCloneWatcher(Box<ScriptCloneWatcher>),
    ScriptCloneWatcherHelper(Box<ScriptCloneWatcherHelper>),
    ScriptCommitService(Box<ScriptCommitService>),
    ScriptContext(Box<ScriptContext>),
    ScriptDebugger(Box<ScriptDebugger>),
    ScriptDocument(Box<ScriptDocument>),
    ScriptEditorService(Box<ScriptEditorService>),
    ScriptProfilerService(Box<ScriptProfilerService>),
    ScriptRegistrationService(Box<ScriptRegistrationService>),
    ScriptRuntime(Box<ScriptRuntime>),
    ScriptService(Box<ScriptService>),
    ScrollingFrame(Box<ScrollingFrame>),
    Seat(Box<Seat>),
    Selection(Box<Selection>),
    SelectionBox(Box<SelectionBox>),
    SelectionHighlightManager(Box<SelectionHighlightManager>),
    SelectionLasso(Box<SelectionLasso>),
    SelectionPartLasso(Box<SelectionPartLasso>),
    SelectionPointLasso(Box<SelectionPointLasso>),
    SelectionSphere(Box<SelectionSphere>),
    SensorBase(Box<SensorBase>),
    SerializationService(Box<SerializationService>),
    ServerReplicator(Box<ServerReplicator>),
    ServerScriptService(Box<ServerScriptService>),
    ServerStorage(Box<ServerStorage>),
    ServiceProvider(Box<ServiceProvider>),
    ServiceVisibilityService(Box<ServiceVisibilityService>),
    SessionService(Box<SessionService>),
    SharedTableRegistry(Box<SharedTableRegistry>),
    Shirt(Box<Shirt>),
    ShirtGraphic(Box<ShirtGraphic>),
    SkateboardController(Box<SkateboardController>),
    SkateboardPlatform(Box<SkateboardPlatform>),
    Skin(Box<Skin>),
    Sky(Box<Sky>),
    SlidingBallConstraint(Box<SlidingBallConstraint>),
    Smoke(Box<Smoke>),
    SmoothVoxelsUpgraderService(Box<SmoothVoxelsUpgraderService>),
    Snap(Box<Snap>),
    SnippetService(Box<SnippetService>),
    SocialService(Box<SocialService>),
    SolidModelContentProvider(Box<SolidModelContentProvider>),
    Sound(Box<Sound>),
    SoundEffect(Box<SoundEffect>),
    SoundGroup(Box<SoundGroup>),
    SoundService(Box<SoundService>),
    Sparkles(Box<Sparkles>),
    SpawnLocation(Box<SpawnLocation>),
    SpawnerService(Box<SpawnerService>),
    SpecialMesh(Box<SpecialMesh>),
    SphereHandleAdornment(Box<SphereHandleAdornment>),
    SpotLight(Box<SpotLight>),
    SpringConstraint(Box<SpringConstraint>),
    StackFrame(Box<StackFrame>),
    StandalonePluginScripts(Box<StandalonePluginScripts>),
    StandardPages(Box<StandardPages>),
    StartPageService(Box<StartPageService>),
    StarterCharacterScripts(Box<StarterCharacterScripts>),
    StarterGear(Box<StarterGear>),
    StarterGui(Box<StarterGui>),
    StarterPack(Box<StarterPack>),
    StarterPlayer(Box<StarterPlayer>),
    StarterPlayerScripts(Box<StarterPlayerScripts>),
    StartupMessageService(Box<StartupMessageService>),
    Stats(Box<Stats>),
    StatsItem(Box<StatsItem>),
    Status(Box<Status>),
    StopWatchReporter(Box<StopWatchReporter>),
    StreamingService(Box<StreamingService>),
    StringValue(Box<StringValue>),
    Studio(Box<Studio>),
    StudioAssetService(Box<StudioAssetService>),
    StudioAttachment(Box<StudioAttachment>),
    StudioCallout(Box<StudioCallout>),
    StudioCameraService(Box<StudioCameraService>),
    StudioData(Box<StudioData>),
    StudioDeviceEmulatorService(Box<StudioDeviceEmulatorService>),
    StudioObjectBase(Box<StudioObjectBase>),
    StudioPublishService(Box<StudioPublishService>),
    StudioScriptDebugEventListener(Box<StudioScriptDebugEventListener>),
    StudioSdkService(Box<StudioSdkService>),
    StudioService(Box<StudioService>),
    StudioTheme(Box<StudioTheme>),
    StudioUserService(Box<StudioUserService>),
    StudioWidget(Box<StudioWidget>),
    StudioWidgetsService(Box<StudioWidgetsService>),
    StyleBase(Box<StyleBase>),
    StyleDerive(Box<StyleDerive>),
    StyleLink(Box<StyleLink>),
    StyleRule(Box<StyleRule>),
    StyleSheet(Box<StyleSheet>),
    StylingService(Box<StylingService>),
    SunRaysEffect(Box<SunRaysEffect>),
    SurfaceAppearance(Box<SurfaceAppearance>),
    SurfaceGui(Box<SurfaceGui>),
    SurfaceGuiBase(Box<SurfaceGuiBase>),
    SurfaceLight(Box<SurfaceLight>),
    SurfaceSelection(Box<SurfaceSelection>),
    SwimController(Box<SwimController>),
    SyncScriptBuilder(Box<SyncScriptBuilder>),
    SystemThemeService(Box<SystemThemeService>),
    TaskScheduler(Box<TaskScheduler>),
    Team(Box<Team>),
    TeamCreateData(Box<TeamCreateData>),
    TeamCreatePublishService(Box<TeamCreatePublishService>),
    TeamCreateService(Box<TeamCreateService>),
    Teams(Box<Teams>),
    TelemetryService(Box<TelemetryService>),
    TeleportAsyncResult(Box<TeleportAsyncResult>),
    TeleportOptions(Box<TeleportOptions>),
    TeleportService(Box<TeleportService>),
    TemporaryCageMeshProvider(Box<TemporaryCageMeshProvider>),
    TemporaryScriptService(Box<TemporaryScriptService>),
    Terrain(Box<Terrain>),
    TerrainDetail(Box<TerrainDetail>),
    TerrainRegion(Box<TerrainRegion>),
    TestService(Box<TestService>),
    TextBox(Box<TextBox>),
    TextBoxService(Box<TextBoxService>),
    TextButton(Box<TextButton>),
    TextChannel(Box<TextChannel>),
    TextChatCommand(Box<TextChatCommand>),
    TextChatConfigurations(Box<TextChatConfigurations>),
    TextChatMessage(Box<TextChatMessage>),
    TextChatMessageProperties(Box<TextChatMessageProperties>),
    TextChatService(Box<TextChatService>),
    TextFilterResult(Box<TextFilterResult>),
    TextFilterTranslatedResult(Box<TextFilterTranslatedResult>),
    TextLabel(Box<TextLabel>),
    TextService(Box<TextService>),
    TextSource(Box<TextSource>),
    Texture(Box<Texture>),
    TextureGenerationPartGroup(Box<TextureGenerationPartGroup>),
    TextureGenerationService(Box<TextureGenerationService>),
    TextureGenerationUnwrappingRequest(Box<TextureGenerationUnwrappingRequest>),
    ThirdPartyUserService(Box<ThirdPartyUserService>),
    ThreadState(Box<ThreadState>),
    TimerService(Box<TimerService>),
    ToastNotificationService(Box<ToastNotificationService>),
    Tool(Box<Tool>),
    Torque(Box<Torque>),
    TorsionSpringConstraint(Box<TorsionSpringConstraint>),
    TotalCountTimeIntervalItem(Box<TotalCountTimeIntervalItem>),
    TouchInputService(Box<TouchInputService>),
    TouchTransmitter(Box<TouchTransmitter>),
    TracerService(Box<TracerService>),
    TrackerLodController(Box<TrackerLodController>),
    TrackerStreamAnimation(Box<TrackerStreamAnimation>),
    Trail(Box<Trail>),
    Translator(Box<Translator>),
    TremoloSoundEffect(Box<TremoloSoundEffect>),
    TriangleMeshPart(Box<TriangleMeshPart>),
    TrussPart(Box<TrussPart>),
    TutorialService(Box<TutorialService>),
    Tween(Box<Tween>),
    TweenBase(Box<TweenBase>),
    TweenService(Box<TweenService>),
    UGCAvatarService(Box<UGCAvatarService>),
    UGCValidationService(Box<UGCValidationService>),
    UIAspectRatioConstraint(Box<UIAspectRatioConstraint>),
    UIBase(Box<UIBase>),
    UIComponent(Box<UIComponent>),
    UIConstraint(Box<UIConstraint>),
    UICorner(Box<UICorner>),
    UIDragDetector(Box<UIDragDetector>),
    UIDragDetectorService(Box<UIDragDetectorService>),
    UIFlexItem(Box<UIFlexItem>),
    UIGradient(Box<UIGradient>),
    UIGridLayout(Box<UIGridLayout>),
    UIGridStyleLayout(Box<UIGridStyleLayout>),
    UILayout(Box<UILayout>),
    UIListLayout(Box<UIListLayout>),
    UIPadding(Box<UIPadding>),
    UIPageLayout(Box<UIPageLayout>),
    UIScale(Box<UIScale>),
    UISizeConstraint(Box<UISizeConstraint>),
    UIStroke(Box<UIStroke>),
    UITableLayout(Box<UITableLayout>),
    UITextSizeConstraint(Box<UITextSizeConstraint>),
    UnionOperation(Box<UnionOperation>),
    UniqueIdLookupService(Box<UniqueIdLookupService>),
    UniversalConstraint(Box<UniversalConstraint>),
    UnreliableRemoteEvent(Box<UnreliableRemoteEvent>),
    UnvalidatedAssetService(Box<UnvalidatedAssetService>),
    UserGameSettings(Box<UserGameSettings>),
    UserInputService(Box<UserInputService>),
    UserService(Box<UserService>),
    UserSettings(Box<UserSettings>),
    UserStorageService(Box<UserStorageService>),
    VRService(Box<VRService>),
    VRStatusService(Box<VRStatusService>),
    ValueBase(Box<ValueBase>),
    Vector3Curve(Box<Vector3Curve>),
    Vector3Value(Box<Vector3Value>),
    VectorForce(Box<VectorForce>),
    VehicleController(Box<VehicleController>),
    VehicleSeat(Box<VehicleSeat>),
    VelocityMotor(Box<VelocityMotor>),
    VersionControlService(Box<VersionControlService>),
    VideoCaptureService(Box<VideoCaptureService>),
    VideoDeviceInput(Box<VideoDeviceInput>),
    VideoDisplay(Box<VideoDisplay>),
    VideoFrame(Box<VideoFrame>),
    VideoPlayer(Box<VideoPlayer>),
    VideoService(Box<VideoService>),
    ViewportFrame(Box<ViewportFrame>),
    VirtualInputManager(Box<VirtualInputManager>),
    VirtualUser(Box<VirtualUser>),
    VisibilityCheckDispatcher(Box<VisibilityCheckDispatcher>),
    Visit(Box<Visit>),
    VisualizationMode(Box<VisualizationMode>),
    VisualizationModeCategory(Box<VisualizationModeCategory>),
    VisualizationModeService(Box<VisualizationModeService>),
    VoiceChatInternal(Box<VoiceChatInternal>),
    VoiceChatService(Box<VoiceChatService>),
    WebSocketClient(Box<WebSocketClient>),
    WebSocketService(Box<WebSocketService>),
    WebViewService(Box<WebViewService>),
    WedgePart(Box<WedgePart>),
    Weld(Box<Weld>),
    WeldConstraint(Box<WeldConstraint>),
    Wire(Box<Wire>),
    WireframeHandleAdornment(Box<WireframeHandleAdornment>),
    Workspace(Box<Workspace>),
    WorkspaceAnnotation(Box<WorkspaceAnnotation>),
    WorldModel(Box<WorldModel>),
    WorldRoot(Box<WorldRoot>),
    WrapDeformer(Box<WrapDeformer>),
    WrapLayer(Box<WrapLayer>),
    WrapTarget(Box<WrapTarget>),
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Accessory {
    superclass: Accoutrement,
    pub AccessoryType: enums::AccessoryType,
}
impl_inherits!(Accessory, Accoutrement);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AccessoryDescription {
    superclass: Instance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AccountService {
    superclass: Instance,
}
impl_inherits!(AccountService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Accoutrement {
    superclass: Instance,
    pub AttachmentPoint: CFrame,
}
impl_inherits!(Accoutrement, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AchievementService {
    superclass: Instance,
}
impl_inherits!(AchievementService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ActivityHistoryEventService {
    superclass: Instance,
}
impl_inherits!(ActivityHistoryEventService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Actor {
    superclass: Model,
}
impl_inherits!(Actor, Model);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AdGui {
    superclass: SurfaceGuiBase,
    pub AdShape: enums::AdShape,
    pub EnableVideoAds: bool,
    pub FallbackImage: ContentId,
}
impl_inherits!(AdGui, SurfaceGuiBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AdPortal {
    superclass: Instance,
}
impl_inherits!(AdPortal, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AdService {
    superclass: Instance,
}
impl_inherits!(AdService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AdvancedDragger {
    superclass: Instance,
}
impl_inherits!(AdvancedDragger, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AirController {
    superclass: ControllerBase,
    pub BalanceMaxTorque: f32,
    pub BalanceSpeed: f32,
    pub MaintainAngularMomentum: bool,
    pub MaintainLinearMomentum: bool,
    pub MoveMaxForce: f32,
    pub TurnMaxTorque: f32,
    pub TurnSpeedFactor: f32,
}
impl_inherits!(AirController, ControllerBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AlignOrientation {
    superclass: Constraint,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AlignPosition {
    superclass: Constraint,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnalysticsSettings {
    superclass: GenericSettings,
}
impl_inherits!(AnalysticsSettings, GenericSettings);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnalyticsService {
    superclass: Instance,
    pub ApiKey: String,
}
impl_inherits!(AnalyticsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AngularVelocity {
    superclass: Constraint,
    pub AngularVelocity: Vector3,
    pub MaxTorque: f32,
    pub ReactionTorqueEnabled: bool,
    pub RelativeTo: enums::ActuatorRelativeTo,
}
impl_inherits!(AngularVelocity, Constraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Animation {
    superclass: Instance,
    pub AnimationId: ContentId,
}
impl_inherits!(Animation, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationClip {
    superclass: Instance,
    pub GuidBinaryString: BinaryString,
    pub Loop: bool,
    pub Priority: enums::AnimationPriority,
}
impl_inherits!(AnimationClip, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationClipProvider {
    superclass: Instance,
}
impl_inherits!(AnimationClipProvider, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationConstraint {
    superclass: Constraint,
    pub IsKinematic: bool,
    pub MaxForce: f32,
    pub MaxTorque: f32,
    pub Transform: CFrame,
}
impl_inherits!(AnimationConstraint, Constraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationController {
    superclass: Instance,
}
impl_inherits!(AnimationController, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationFromVideoCreatorService {
    superclass: Instance,
}
impl_inherits!(AnimationFromVideoCreatorService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationFromVideoCreatorStudioService {
    superclass: Instance,
}
impl_inherits!(AnimationFromVideoCreatorStudioService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationImportData {
    superclass: BaseImportData,
}
impl_inherits!(AnimationImportData, BaseImportData);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationRigData {
    superclass: Instance,
    pub Label: BinaryString,
    pub Name: BinaryString,
    pub Parent: BinaryString,
    pub PostTransform: BinaryString,
    pub PreTransform: BinaryString,
    pub Transform: BinaryString,
}
impl_inherits!(AnimationRigData, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationStreamTrack {
    superclass: Instance,
}
impl_inherits!(AnimationStreamTrack, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationTrack {
    superclass: Instance,
    pub Priority: enums::AnimationPriority,
}
impl_inherits!(AnimationTrack, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Animator {
    superclass: Instance,
    pub PreferLodEnabled: bool,
}
impl_inherits!(Animator, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Annotation {
    superclass: Instance,
}
impl_inherits!(Annotation, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnnotationsService {
    superclass: Instance,
}
impl_inherits!(AnnotationsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AppLifecycleObserverService {
    superclass: Instance,
}
impl_inherits!(AppLifecycleObserverService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AppStorageService {
    superclass: LocalStorageService,
}
impl_inherits!(AppStorageService, LocalStorageService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AppUpdateService {
    superclass: Instance,
}
impl_inherits!(AppUpdateService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ArcHandles {
    superclass: HandlesBase,
    pub Axes: Axes,
}
impl_inherits!(ArcHandles, HandlesBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetCounterService {
    superclass: Instance,
}
impl_inherits!(AssetCounterService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetDeliveryProxy {
    superclass: Instance,
    pub Interface: String,
    pub Port: i32,
    pub StartServer: bool,
}
impl_inherits!(AssetDeliveryProxy, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetImportService {
    superclass: Instance,
}
impl_inherits!(AssetImportService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetImportSession {
    superclass: ImportSession,
}
impl_inherits!(AssetImportSession, ImportSession);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetManagerService {
    superclass: Instance,
}
impl_inherits!(AssetManagerService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetPatchSettings {
    superclass: Instance,
    pub ContentId: String,
    pub OutputPath: String,
    pub PatchId: String,
}
impl_inherits!(AssetPatchSettings, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetService {
    superclass: Instance,
}
impl_inherits!(AssetService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetSoundEffect {
    superclass: CustomSoundEffect,
}
impl_inherits!(AssetSoundEffect, CustomSoundEffect);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Atmosphere {
    superclass: Instance,
    pub Color: Color3,
    pub Decay: Color3,
    pub Density: f32,
    pub Glare: f32,
    pub Haze: f32,
    pub Offset: f32,
}
impl_inherits!(Atmosphere, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AtmosphereSensor {
    superclass: SensorBase,
}
impl_inherits!(AtmosphereSensor, SensorBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Attachment {
    superclass: Instance,
    pub CFrame: CFrame,
    pub Visible: bool,
}
impl_inherits!(Attachment, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioAnalyzer {
    superclass: Instance,
    pub SpectrumEnabled: bool,
    pub WindowSize: enums::AudioWindowSize,
}
impl_inherits!(AudioAnalyzer, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioChannelMixer {
    superclass: Instance,
    pub Layout: enums::AudioChannelLayout,
}
impl_inherits!(AudioChannelMixer, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioChannelSplitter {
    superclass: Instance,
    pub Layout: enums::AudioChannelLayout,
}
impl_inherits!(AudioChannelSplitter, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioChorus {
    superclass: Instance,
    pub Bypass: bool,
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
impl_inherits!(AudioChorus, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioCompressor {
    superclass: Instance,
    pub Attack: f32,
    pub Bypass: bool,
    pub MakeupGain: f32,
    pub Ratio: f32,
    pub Release: f32,
    pub Threshold: f32,
}
impl_inherits!(AudioCompressor, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioDeviceInput {
    superclass: Instance,
    pub AccessType: enums::AccessModifierType,
    pub Active: bool,
    pub Muted: bool,
    pub Player: Ref,
    pub Volume: f32,
}
impl_inherits!(AudioDeviceInput, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioDeviceOutput {
    superclass: Instance,
    pub Player: Ref,
}
impl_inherits!(AudioDeviceOutput, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioDistortion {
    superclass: Instance,
    pub Bypass: bool,
    pub Level: f32,
}
impl_inherits!(AudioDistortion, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioEcho {
    superclass: Instance,
    pub Bypass: bool,
    pub DelayTime: f32,
    pub DryLevel: f32,
    pub Feedback: f32,
    pub RampTime: f32,
    pub WetLevel: f32,
}
impl_inherits!(AudioEcho, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioEmitter {
    superclass: Instance,
    pub AngleAttenuation: BinaryString,
    pub AudioInteractionGroup: String,
    pub DistanceAttenuation: BinaryString,
    pub SimulationFidelity: enums::AudioSimulationFidelity,
}
impl_inherits!(AudioEmitter, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioEqualizer {
    superclass: Instance,
    pub Bypass: bool,
    pub HighGain: f32,
    pub LowGain: f32,
    pub MidGain: f32,
    pub MidRange: NumberRange,
}
impl_inherits!(AudioEqualizer, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioFader {
    superclass: Instance,
    pub Bypass: bool,
    pub Volume: f32,
}
impl_inherits!(AudioFader, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioFilter {
    superclass: Instance,
    pub Bypass: bool,
    pub FilterType: enums::AudioFilterType,
    pub Frequency: f32,
    pub Gain: f32,
    pub Q: f32,
}
impl_inherits!(AudioFilter, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioFlanger {
    superclass: Instance,
    pub Bypass: bool,
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
impl_inherits!(AudioFlanger, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioFocusService {
    superclass: Instance,
}
impl_inherits!(AudioFocusService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioLimiter {
    superclass: Instance,
    pub Bypass: bool,
    pub MaxLevel: f32,
    pub Release: f32,
}
impl_inherits!(AudioLimiter, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioListener {
    superclass: Instance,
    pub AngleAttenuation: BinaryString,
    pub AudioInteractionGroup: String,
    pub DistanceAttenuation: BinaryString,
    pub SimulationFidelity: enums::AudioSimulationFidelity,
}
impl_inherits!(AudioListener, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioPages {
    superclass: Pages,
}
impl_inherits!(AudioPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioPitchShifter {
    superclass: Instance,
    pub Bypass: bool,
    pub Pitch: f32,
    pub WindowSize: enums::AudioWindowSize,
}
impl_inherits!(AudioPitchShifter, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioPlayer {
    superclass: Instance,
    pub Asset: ContentId,
    pub AutoLoad: bool,
    pub LoopRegion: NumberRange,
    pub Looping: bool,
    pub PlaybackRegion: NumberRange,
    pub PlaybackSpeed: f64,
    pub TimePosition: f64,
    pub Volume: f32,
}
impl_inherits!(AudioPlayer, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioReverb {
    superclass: Instance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioSearchParams {
    superclass: Instance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioTextToSpeech {
    superclass: Instance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AuroraScript {
    superclass: LuaSourceContainer,
    pub EnableCulling: bool,
    pub EnableLod: bool,
    pub LodCriticality: i32,
    pub Priority: i32,
    pub Source: String,
    pub Tag: String,
}
impl_inherits!(AuroraScript, LuaSourceContainer);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AuroraScriptService {
    superclass: Instance,
    pub BufferSize: i32,
}
impl_inherits!(AuroraScriptService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AuroraService {
    superclass: Instance,
}
impl_inherits!(AuroraService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarChatService {
    superclass: Instance,
}
impl_inherits!(AvatarChatService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarCreationService {
    superclass: Instance,
}
impl_inherits!(AvatarCreationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarEditorService {
    superclass: Instance,
}
impl_inherits!(AvatarEditorService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarImportService {
    superclass: Instance,
}
impl_inherits!(AvatarImportService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarPreloader {
    superclass: Instance,
}
impl_inherits!(AvatarPreloader, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Backpack {
    superclass: Instance,
}
impl_inherits!(Backpack, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BackpackItem {
    superclass: Model,
    pub TextureId: ContentId,
}
impl_inherits!(BackpackItem, Model);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BadgeService {
    superclass: Instance,
}
impl_inherits!(BadgeService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BallSocketConstraint {
    superclass: Constraint,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BanHistoryPages {
    superclass: Pages,
}
impl_inherits!(BanHistoryPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BaseImportData {
    superclass: Instance,
    pub ImportName: String,
    pub ShouldImport: bool,
}
impl_inherits!(BaseImportData, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BasePart {
    superclass: PVInstance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BasePlayerGui {
    superclass: Instance,
}
impl_inherits!(BasePlayerGui, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BaseRemoteEvent {
    superclass: Instance,
}
impl_inherits!(BaseRemoteEvent, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BaseScript {
    superclass: LuaSourceContainer,
    pub Disabled: bool,
    pub LinkedSource: ContentId,
    pub RunContext: enums::RunContext,
}
impl_inherits!(BaseScript, LuaSourceContainer);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BaseWrap {
    superclass: Instance,
    pub CageMeshContent: Content,
    pub CageOrigin: CFrame,
    pub HsrAssetId: ContentId,
    pub HsrData: SharedString,
    pub HsrMeshIdData: SharedString,
    pub ImportOrigin: CFrame,
    pub TemporaryCageMeshId: ContentId,
}
impl_inherits!(BaseWrap, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Beam {
    superclass: Instance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BevelMesh {
    superclass: DataModelMesh,
    pub Bevel: f32,
    pub BevelRoundness: f32,
    pub Bulge: f32,
}
impl_inherits!(BevelMesh, DataModelMesh);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BillboardGui {
    superclass: LayerCollector,
    pub Active: bool,
    pub Adornee: Ref,
    pub AlwaysOnTop: bool,
    pub Brightness: f32,
    pub ClipsDescendants: bool,
    pub DistanceLowerLimit: f32,
    pub DistanceStep: f32,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BinaryStringValue {
    superclass: ValueBase,
    pub Value: BinaryString,
}
impl_inherits!(BinaryStringValue, ValueBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BindableEvent {
    superclass: Instance,
}
impl_inherits!(BindableEvent, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BindableFunction {
    superclass: Instance,
}
impl_inherits!(BindableFunction, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BlockMesh {
    superclass: BevelMesh,
}
impl_inherits!(BlockMesh, BevelMesh);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BloomEffect {
    superclass: PostEffect,
    pub Intensity: f32,
    pub Size: f32,
    pub Threshold: f32,
}
impl_inherits!(BloomEffect, PostEffect);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BlurEffect {
    superclass: PostEffect,
    pub Size: f32,
}
impl_inherits!(BlurEffect, PostEffect);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyAngularVelocity {
    superclass: BodyMover,
    pub AngularVelocity: Vector3,
    pub MaxTorque: Vector3,
    pub P: f32,
}
impl_inherits!(BodyAngularVelocity, BodyMover);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyColors {
    superclass: CharacterAppearance,
    pub HeadColor3: Color3,
    pub LeftArmColor3: Color3,
    pub LeftLegColor3: Color3,
    pub RightArmColor3: Color3,
    pub RightLegColor3: Color3,
    pub TorsoColor3: Color3,
}
impl_inherits!(BodyColors, CharacterAppearance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyForce {
    superclass: BodyMover,
    pub Force: Vector3,
}
impl_inherits!(BodyForce, BodyMover);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyGyro {
    superclass: BodyMover,
    pub CFrame: CFrame,
    pub D: f32,
    pub MaxTorque: Vector3,
    pub P: f32,
}
impl_inherits!(BodyGyro, BodyMover);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyMover {
    superclass: Instance,
}
impl_inherits!(BodyMover, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyPartDescription {
    superclass: Instance,
    pub AssetId: i64,
    pub BodyPart: enums::BodyPart,
    pub Color: Color3,
    pub Instance: Ref,
}
impl_inherits!(BodyPartDescription, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyPosition {
    superclass: BodyMover,
    pub D: f32,
    pub MaxForce: Vector3,
    pub P: f32,
    pub Position: Vector3,
}
impl_inherits!(BodyPosition, BodyMover);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyThrust {
    superclass: BodyMover,
    pub Force: Vector3,
    pub Location: Vector3,
}
impl_inherits!(BodyThrust, BodyMover);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyVelocity {
    superclass: BodyMover,
    pub MaxForce: Vector3,
    pub P: f32,
    pub Velocity: Vector3,
}
impl_inherits!(BodyVelocity, BodyMover);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Bone {
    superclass: Attachment,
}
impl_inherits!(Bone, Attachment);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BoolValue {
    superclass: ValueBase,
    pub Value: bool,
}
impl_inherits!(BoolValue, ValueBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BoxHandleAdornment {
    superclass: HandleAdornment,
    pub Size: Vector3,
}
impl_inherits!(BoxHandleAdornment, HandleAdornment);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Breakpoint {
    superclass: Instance,
}
impl_inherits!(Breakpoint, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BrickColorValue {
    superclass: ValueBase,
    pub Value: BrickColor,
}
impl_inherits!(BrickColorValue, ValueBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BrowserService {
    superclass: Instance,
}
impl_inherits!(BrowserService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BubbleChatConfiguration {
    superclass: TextChatConfigurations,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BubbleChatMessageProperties {
    superclass: TextChatMessageProperties,
}
impl_inherits!(BubbleChatMessageProperties, TextChatMessageProperties);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BugReporterService {
    superclass: Instance,
}
impl_inherits!(BugReporterService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BulkImportService {
    superclass: Instance,
}
impl_inherits!(BulkImportService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BuoyancySensor {
    superclass: SensorBase,
    pub FullySubmerged: bool,
    pub TouchingSurface: bool,
}
impl_inherits!(BuoyancySensor, SensorBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CFrameValue {
    superclass: ValueBase,
    pub Value: CFrame,
}
impl_inherits!(CFrameValue, ValueBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CSGDictionaryService {
    superclass: FlyweightService,
}
impl_inherits!(CSGDictionaryService, FlyweightService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CacheableContentProvider {
    superclass: Instance,
}
impl_inherits!(CacheableContentProvider, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CalloutService {
    superclass: Instance,
}
impl_inherits!(CalloutService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Camera {
    superclass: PVInstance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CanvasGroup {
    superclass: GuiObject,
    pub GroupColor3: Color3,
    pub GroupTransparency: f32,
}
impl_inherits!(CanvasGroup, GuiObject);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Capture {
    superclass: Object,
}
impl_inherits!(Capture, Object);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CaptureService {
    superclass: Instance,
}
impl_inherits!(CaptureService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CatalogPages {
    superclass: Pages,
}
impl_inherits!(CatalogPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChangeHistoryService {
    superclass: Instance,
}
impl_inherits!(ChangeHistoryService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChannelSelectorSoundEffect {
    superclass: CustomSoundEffect,
    pub Channel: i32,
}
impl_inherits!(ChannelSelectorSoundEffect, CustomSoundEffect);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChannelTabsConfiguration {
    superclass: TextChatConfigurations,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CharacterAppearance {
    superclass: Instance,
}
impl_inherits!(CharacterAppearance, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CharacterMesh {
    superclass: CharacterAppearance,
    pub BaseTextureId: i64,
    pub BodyPart: enums::BodyPart,
    pub MeshId: i64,
    pub OverlayTextureId: i64,
}
impl_inherits!(CharacterMesh, CharacterAppearance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Chat {
    superclass: Instance,
    pub BubbleChatEnabled: bool,
    pub IsAutoMigrated: bool,
    pub LoadDefaultChat: bool,
}
impl_inherits!(Chat, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChatInputBarConfiguration {
    superclass: TextChatConfigurations,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChatWindowConfiguration {
    superclass: TextChatConfigurations,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChatWindowMessageProperties {
    superclass: TextChatMessageProperties,
}
impl_inherits!(ChatWindowMessageProperties, TextChatMessageProperties);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChatbotUIService {
    superclass: Instance,
}
impl_inherits!(ChatbotUIService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChorusSoundEffect {
    superclass: SoundEffect,
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
impl_inherits!(ChorusSoundEffect, SoundEffect);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ClickDetector {
    superclass: Instance,
    pub CursorIcon: ContentId,
    pub MaxActivationDistance: f32,
}
impl_inherits!(ClickDetector, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ClientReplicator {
    superclass: NetworkReplicator,
}
impl_inherits!(ClientReplicator, NetworkReplicator);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ClimbController {
    superclass: ControllerBase,
    pub AccelerationTime: f32,
    pub BalanceMaxTorque: f32,
    pub BalanceSpeed: f32,
    pub MoveMaxForce: f32,
}
impl_inherits!(ClimbController, ControllerBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Clothing {
    superclass: CharacterAppearance,
    pub Color3: Color3,
}
impl_inherits!(Clothing, CharacterAppearance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CloudCRUDService {
    superclass: Instance,
}
impl_inherits!(CloudCRUDService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CloudLocalizationTable {
    superclass: LocalizationTable,
}
impl_inherits!(CloudLocalizationTable, LocalizationTable);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Clouds {
    superclass: Instance,
    pub Color: Color3,
    pub Cover: f32,
    pub Density: f32,
    pub Enabled: bool,
}
impl_inherits!(Clouds, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ClusterPacketCache {
    superclass: Instance,
}
impl_inherits!(ClusterPacketCache, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Collaborator {
    superclass: Instance,
}
impl_inherits!(Collaborator, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CollaboratorsService {
    superclass: Instance,
}
impl_inherits!(CollaboratorsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CollectionService {
    superclass: Instance,
}
impl_inherits!(CollectionService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Color3Value {
    superclass: ValueBase,
    pub Value: Color3,
}
impl_inherits!(Color3Value, ValueBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ColorCorrectionEffect {
    superclass: PostEffect,
    pub Brightness: f32,
    pub Contrast: f32,
    pub Saturation: f32,
    pub TintColor: Color3,
}
impl_inherits!(ColorCorrectionEffect, PostEffect);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ColorGradingEffect {
    superclass: PostEffect,
    pub TonemapperPreset: enums::TonemapperPreset,
}
impl_inherits!(ColorGradingEffect, PostEffect);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CommandInstance {
    superclass: Instance,
}
impl_inherits!(CommandInstance, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CommandService {
    superclass: Instance,
}
impl_inherits!(CommandService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CommerceService {
    superclass: Instance,
}
impl_inherits!(CommerceService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CompressorSoundEffect {
    superclass: SoundEffect,
    pub Attack: f32,
    pub GainMakeup: f32,
    pub Ratio: f32,
    pub Release: f32,
    pub SideChain: Ref,
    pub Threshold: f32,
}
impl_inherits!(CompressorSoundEffect, SoundEffect);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ConeHandleAdornment {
    superclass: HandleAdornment,
    pub Height: f32,
    pub Radius: f32,
}
impl_inherits!(ConeHandleAdornment, HandleAdornment);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ConfigService {
    superclass: Instance,
}
impl_inherits!(ConfigService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ConfigSnapshot {
    superclass: Object,
}
impl_inherits!(ConfigSnapshot, Object);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Configuration {
    superclass: Instance,
}
impl_inherits!(Configuration, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ConfigureServerService {
    superclass: Instance,
}
impl_inherits!(ConfigureServerService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ConnectivityService {
    superclass: Instance,
}
impl_inherits!(ConnectivityService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Constraint {
    superclass: Instance,
    pub Attachment0: Ref,
    pub Attachment1: Ref,
    pub Color: BrickColor,
    pub Enabled: bool,
    pub Visible: bool,
}
impl_inherits!(Constraint, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ContentProvider {
    superclass: Instance,
}
impl_inherits!(ContentProvider, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ContextActionService {
    superclass: Instance,
}
impl_inherits!(ContextActionService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Controller {
    superclass: Instance,
}
impl_inherits!(Controller, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ControllerBase {
    superclass: Instance,
    pub BalanceRigidityEnabled: bool,
    pub MoveSpeedFactor: f32,
}
impl_inherits!(ControllerBase, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ControllerManager {
    superclass: Instance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ControllerPartSensor {
    superclass: ControllerSensor,
    pub HitFrame: CFrame,
    pub HitNormal: Vector3,
    pub SearchDistance: f32,
    pub SensedPart: Ref,
    pub SensorMode: enums::SensorMode,
}
impl_inherits!(ControllerPartSensor, ControllerSensor);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ControllerSensor {
    superclass: SensorBase,
}
impl_inherits!(ControllerSensor, SensorBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ControllerService {
    superclass: Instance,
}
impl_inherits!(ControllerService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ConversationalAIAcceptanceService {
    superclass: Instance,
}
impl_inherits!(ConversationalAIAcceptanceService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CookiesService {
    superclass: Instance,
}
impl_inherits!(CookiesService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CoreGui {
    superclass: BasePlayerGui,
    pub SelectionImageObject: Ref,
}
impl_inherits!(CoreGui, BasePlayerGui);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CorePackages {
    superclass: Instance,
}
impl_inherits!(CorePackages, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CoreScript {
    superclass: BaseScript,
}
impl_inherits!(CoreScript, BaseScript);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CoreScriptDebuggingManagerHelper {
    superclass: Instance,
}
impl_inherits!(CoreScriptDebuggingManagerHelper, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CoreScriptSyncService {
    superclass: Instance,
}
impl_inherits!(CoreScriptSyncService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CornerWedgePart {
    superclass: BasePart,
}
impl_inherits!(CornerWedgePart, BasePart);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CreationDBService {
    superclass: Instance,
}
impl_inherits!(CreationDBService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CreatorStoreService {
    superclass: Instance,
}
impl_inherits!(CreatorStoreService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CrossDMScriptChangeListener {
    superclass: Instance,
}
impl_inherits!(CrossDMScriptChangeListener, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CurveAnimation {
    superclass: AnimationClip,
}
impl_inherits!(CurveAnimation, AnimationClip);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CustomEvent {
    superclass: Instance,
    pub PersistedCurrentValue: f32,
}
impl_inherits!(CustomEvent, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CustomEventReceiver {
    superclass: Instance,
    pub Source: Ref,
}
impl_inherits!(CustomEventReceiver, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CustomLog {
    superclass: Instance,
}
impl_inherits!(CustomLog, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CustomSoundEffect {
    superclass: SoundEffect,
}
impl_inherits!(CustomSoundEffect, SoundEffect);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CylinderHandleAdornment {
    superclass: HandleAdornment,
    pub Angle: f32,
    pub Height: f32,
    pub InnerRadius: f32,
    pub Radius: f32,
}
impl_inherits!(CylinderHandleAdornment, HandleAdornment);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CylinderMesh {
    superclass: BevelMesh,
}
impl_inherits!(CylinderMesh, BevelMesh);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CylindricalConstraint {
    superclass: SlidingBallConstraint,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataModel {
    superclass: ServiceProvider,
}
impl_inherits!(DataModel, ServiceProvider);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataModelMesh {
    superclass: Instance,
    pub Offset: Vector3,
    pub Scale: Vector3,
    pub VertexColor: Vector3,
}
impl_inherits!(DataModelMesh, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataModelPatchService {
    superclass: Instance,
}
impl_inherits!(DataModelPatchService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataModelSession {
    superclass: Instance,
}
impl_inherits!(DataModelSession, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStore {
    superclass: GlobalDataStore,
}
impl_inherits!(DataStore, GlobalDataStore);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreGetOptions {
    superclass: Instance,
    pub UseCache: bool,
}
impl_inherits!(DataStoreGetOptions, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreIncrementOptions {
    superclass: Instance,
}
impl_inherits!(DataStoreIncrementOptions, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreInfo {
    superclass: Instance,
}
impl_inherits!(DataStoreInfo, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreKey {
    superclass: Instance,
}
impl_inherits!(DataStoreKey, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreKeyInfo {
    superclass: Instance,
}
impl_inherits!(DataStoreKeyInfo, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreKeyPages {
    superclass: Pages,
}
impl_inherits!(DataStoreKeyPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreListingPages {
    superclass: Pages,
}
impl_inherits!(DataStoreListingPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreObjectVersionInfo {
    superclass: Instance,
}
impl_inherits!(DataStoreObjectVersionInfo, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreOptions {
    superclass: Instance,
    pub AllScopes: bool,
}
impl_inherits!(DataStoreOptions, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStorePages {
    superclass: Pages,
}
impl_inherits!(DataStorePages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreService {
    superclass: Instance,
    pub AutomaticRetry: bool,
    pub LegacyNamingScheme: bool,
}
impl_inherits!(DataStoreService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreSetOptions {
    superclass: Instance,
}
impl_inherits!(DataStoreSetOptions, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreVersionPages {
    superclass: Pages,
}
impl_inherits!(DataStoreVersionPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Debris {
    superclass: Instance,
    pub MaxItems: i32,
}
impl_inherits!(Debris, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebugSettings {
    superclass: Instance,
    pub IsScriptStackTracingEnabled: bool,
    pub ReportSoundWarnings: bool,
    pub TickCountPreciseOverride: enums::TickCountSampleMethod,
}
impl_inherits!(DebugSettings, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggablePluginWatcher {
    superclass: Instance,
}
impl_inherits!(DebuggablePluginWatcher, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerBreakpoint {
    superclass: Instance,
    pub Condition: String,
    pub ContinueExecution: bool,
    pub IsContextDependentBreakpoint: bool,
    pub IsEnabled: bool,
    pub Line: i32,
    pub LogExpression: String,
}
impl_inherits!(DebuggerBreakpoint, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerConnection {
    superclass: Instance,
}
impl_inherits!(DebuggerConnection, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerConnectionManager {
    superclass: Instance,
    pub Timeout: f64,
}
impl_inherits!(DebuggerConnectionManager, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerLuaResponse {
    superclass: Instance,
}
impl_inherits!(DebuggerLuaResponse, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerManager {
    superclass: Instance,
}
impl_inherits!(DebuggerManager, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerUIService {
    superclass: Instance,
}
impl_inherits!(DebuggerUIService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerVariable {
    superclass: Instance,
}
impl_inherits!(DebuggerVariable, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerWatch {
    superclass: Instance,
    pub Expression: String,
}
impl_inherits!(DebuggerWatch, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Decal {
    superclass: FaceInstance,
    pub Color3: Color3,
    pub TextureContent: Content,
    pub Transparency: f32,
    pub ZIndex: i32,
}
impl_inherits!(Decal, FaceInstance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DepthOfFieldEffect {
    superclass: PostEffect,
    pub FarIntensity: f32,
    pub FocusDistance: f32,
    pub InFocusRadius: f32,
    pub NearIntensity: f32,
}
impl_inherits!(DepthOfFieldEffect, PostEffect);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DeviceIdService {
    superclass: Instance,
}
impl_inherits!(DeviceIdService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Dialog {
    superclass: Instance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DialogChoice {
    superclass: Instance,
    pub GoodbyeChoiceActive: bool,
    pub GoodbyeDialog: String,
    pub ResponseDialog: String,
    pub UserDialog: String,
}
impl_inherits!(DialogChoice, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DistortionSoundEffect {
    superclass: SoundEffect,
    pub Level: f32,
}
impl_inherits!(DistortionSoundEffect, SoundEffect);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DockWidgetPluginGui {
    superclass: PluginGui,
}
impl_inherits!(DockWidgetPluginGui, PluginGui);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DoubleConstrainedValue {
    superclass: ValueBase,
    pub MaxValue: f64,
    pub MinValue: f64,
    pub Value: f64,
}
impl_inherits!(DoubleConstrainedValue, ValueBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DraftsService {
    superclass: Instance,
}
impl_inherits!(DraftsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DragDetector {
    superclass: ClickDetector,
    pub ActivatedCursorIcon: ContentId,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Dragger {
    superclass: Instance,
}
impl_inherits!(Dragger, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DraggerService {
    superclass: Instance,
}
impl_inherits!(DraggerService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DynamicRotate {
    superclass: JointInstance,
    pub BaseAngle: f32,
}
impl_inherits!(DynamicRotate, JointInstance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EchoSoundEffect {
    superclass: SoundEffect,
    pub Delay: f32,
    pub DryLevel: f32,
    pub Feedback: f32,
    pub WetLevel: f32,
}
impl_inherits!(EchoSoundEffect, SoundEffect);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EditableImage {
    superclass: Object,
    pub ImageData: BinaryString,
}
impl_inherits!(EditableImage, Object);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EditableMesh {
    superclass: Object,
    pub MeshData: SharedString,
    pub SkinningEnabled: bool,
}
impl_inherits!(EditableMesh, Object);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EditableService {
    superclass: Instance,
}
impl_inherits!(EditableService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EmotesPages {
    superclass: InventoryPages,
}
impl_inherits!(EmotesPages, InventoryPages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EqualizerSoundEffect {
    superclass: SoundEffect,
    pub HighGain: f32,
    pub LowGain: f32,
    pub MidGain: f32,
}
impl_inherits!(EqualizerSoundEffect, SoundEffect);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EulerRotationCurve {
    superclass: Instance,
    pub RotationOrder: enums::RotationOrder,
}
impl_inherits!(EulerRotationCurve, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EventIngestService {
    superclass: Instance,
}
impl_inherits!(EventIngestService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ExampleService {
    superclass: Instance,
}
impl_inherits!(ExampleService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ExperienceAuthService {
    superclass: Instance,
}
impl_inherits!(ExperienceAuthService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ExperienceInviteOptions {
    superclass: Instance,
    pub InviteMessageId: String,
    pub InviteUser: i64,
    pub LaunchData: String,
    pub PromptMessage: String,
}
impl_inherits!(ExperienceInviteOptions, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ExperienceNotificationService {
    superclass: Instance,
}
impl_inherits!(ExperienceNotificationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ExperienceService {
    superclass: Instance,
}
impl_inherits!(ExperienceService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ExperienceStateCaptureService {
    superclass: Instance,
}
impl_inherits!(ExperienceStateCaptureService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ExplorerFilter {
    superclass: Instance,
}
impl_inherits!(ExplorerFilter, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ExplorerFilterAutocompleter {
    superclass: Instance,
}
impl_inherits!(ExplorerFilterAutocompleter, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ExplorerServiceVisibilityService {
    superclass: Instance,
}
impl_inherits!(ExplorerServiceVisibilityService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Explosion {
    superclass: Instance,
    pub BlastPressure: f32,
    pub BlastRadius: f32,
    pub DestroyJointRadiusPercent: f32,
    pub ExplosionType: enums::ExplosionType,
    pub Position: Vector3,
    pub TimeScale: f32,
    pub Visible: bool,
}
impl_inherits!(Explosion, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FaceAnimatorService {
    superclass: Instance,
}
impl_inherits!(FaceAnimatorService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FaceControls {
    superclass: Instance,
}
impl_inherits!(FaceControls, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FaceInstance {
    superclass: Instance,
    pub Face: enums::NormalId,
}
impl_inherits!(FaceInstance, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FacialAgeEstimationService {
    superclass: Instance,
}
impl_inherits!(FacialAgeEstimationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FacialAnimationRecordingService {
    superclass: Instance,
}
impl_inherits!(FacialAnimationRecordingService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FacialAnimationStreamingServiceStats {
    superclass: Instance,
}
impl_inherits!(FacialAnimationStreamingServiceStats, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FacialAnimationStreamingServiceV2 {
    superclass: Instance,
    pub ServiceState: i32,
}
impl_inherits!(FacialAnimationStreamingServiceV2, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FacialAnimationStreamingSubsessionStats {
    superclass: Instance,
}
impl_inherits!(FacialAnimationStreamingSubsessionStats, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FacsImportData {
    superclass: BaseImportData,
}
impl_inherits!(FacsImportData, BaseImportData);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Feature {
    superclass: Instance,
    pub FaceId: enums::NormalId,
    pub InOut: enums::InOut,
    pub LeftRight: enums::LeftRight,
    pub TopBottom: enums::TopBottom,
}
impl_inherits!(Feature, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FeatureRestrictionManager {
    superclass: Instance,
}
impl_inherits!(FeatureRestrictionManager, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FeedPages {
    superclass: Pages,
}
impl_inherits!(FeedPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FeedService {
    superclass: Instance,
}
impl_inherits!(FeedService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct File {
    superclass: Instance,
}
impl_inherits!(File, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FileMesh {
    superclass: DataModelMesh,
    pub MeshId: ContentId,
    pub TextureId: ContentId,
}
impl_inherits!(FileMesh, DataModelMesh);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Fire {
    superclass: Instance,
    pub Color: Color3,
    pub Enabled: bool,
    pub SecondaryColor: Color3,
    pub TimeScale: f32,
}
impl_inherits!(Fire, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Flag {
    superclass: Tool,
    pub TeamColor: BrickColor,
}
impl_inherits!(Flag, Tool);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FlagStand {
    superclass: Part,
    pub TeamColor: BrickColor,
}
impl_inherits!(FlagStand, Part);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FlagStandService {
    superclass: Instance,
}
impl_inherits!(FlagStandService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FlangeSoundEffect {
    superclass: SoundEffect,
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
impl_inherits!(FlangeSoundEffect, SoundEffect);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FloatCurve {
    superclass: Instance,
    pub ValuesAndTimes: BinaryString,
}
impl_inherits!(FloatCurve, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FloorWire {
    superclass: GuiBase3d,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FluidForceSensor {
    superclass: SensorBase,
}
impl_inherits!(FluidForceSensor, SensorBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FlyweightService {
    superclass: Instance,
}
impl_inherits!(FlyweightService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Folder {
    superclass: Instance,
}
impl_inherits!(Folder, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ForceField {
    superclass: Instance,
    pub Visible: bool,
}
impl_inherits!(ForceField, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FormFactorPart {
    superclass: BasePart,
}
impl_inherits!(FormFactorPart, BasePart);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Frame {
    superclass: GuiObject,
    pub Style: enums::FrameStyle,
}
impl_inherits!(Frame, GuiObject);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FriendPages {
    superclass: Pages,
}
impl_inherits!(FriendPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FriendService {
    superclass: Instance,
}
impl_inherits!(FriendService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FunctionalTest {
    superclass: Instance,
    pub Description: String,
    pub HasMigratedSettingsToTestService: bool,
}
impl_inherits!(FunctionalTest, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GamePassService {
    superclass: Instance,
}
impl_inherits!(GamePassService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GameSettings {
    superclass: Instance,
    pub VideoCaptureEnabled: bool,
}
impl_inherits!(GameSettings, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GamepadService {
    superclass: Instance,
    pub GamepadCursorEnabled: bool,
}
impl_inherits!(GamepadService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GenerationService {
    superclass: Instance,
}
impl_inherits!(GenerationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GenericChallengeService {
    superclass: Instance,
}
impl_inherits!(GenericChallengeService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GenericSettings {
    superclass: ServiceProvider,
}
impl_inherits!(GenericSettings, ServiceProvider);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Geometry {
    superclass: Instance,
}
impl_inherits!(Geometry, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GeometryService {
    superclass: Instance,
}
impl_inherits!(GeometryService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GetTextBoundsParams {
    superclass: Instance,
    pub Font: Font,
    pub RichText: bool,
    pub Size: f32,
    pub Text: String,
    pub Width: f32,
}
impl_inherits!(GetTextBoundsParams, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GlobalDataStore {
    superclass: Instance,
}
impl_inherits!(GlobalDataStore, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GlobalSettings {
    superclass: GenericSettings,
}
impl_inherits!(GlobalSettings, GenericSettings);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Glue {
    superclass: JointInstance,
    pub F0: Vector3,
    pub F1: Vector3,
    pub F2: Vector3,
    pub F3: Vector3,
}
impl_inherits!(Glue, JointInstance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GoogleAnalyticsConfiguration {
    superclass: Instance,
    pub GaId: String,
}
impl_inherits!(GoogleAnalyticsConfiguration, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GroundController {
    superclass: ControllerBase,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GroupImportData {
    superclass: BaseImportData,
    pub Anchored: bool,
    pub ImportAsModelAsset: bool,
    pub InsertInWorkspace: bool,
}
impl_inherits!(GroupImportData, BaseImportData);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GroupService {
    superclass: Instance,
}
impl_inherits!(GroupService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiBase {
    superclass: Instance,
}
impl_inherits!(GuiBase, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiBase2d {
    superclass: GuiBase,
    pub AutoLocalize: bool,
    pub RootLocalizationTable: Ref,
    pub SelectionBehaviorDown: enums::SelectionBehavior,
    pub SelectionBehaviorLeft: enums::SelectionBehavior,
    pub SelectionBehaviorRight: enums::SelectionBehavior,
    pub SelectionBehaviorUp: enums::SelectionBehavior,
    pub SelectionGroup: bool,
}
impl_inherits!(GuiBase2d, GuiBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiBase3d {
    superclass: GuiBase,
    pub Color3: Color3,
    pub Transparency: f32,
    pub Visible: bool,
}
impl_inherits!(GuiBase3d, GuiBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiButton {
    superclass: GuiObject,
    pub AutoButtonColor: bool,
    pub HoverHapticEffect: Ref,
    pub Modal: bool,
    pub PressHapticEffect: Ref,
    pub Selected: bool,
    pub Style: enums::ButtonStyle,
}
impl_inherits!(GuiButton, GuiObject);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiLabel {
    superclass: GuiObject,
}
impl_inherits!(GuiLabel, GuiObject);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiMain {
    superclass: ScreenGui,
}
impl_inherits!(GuiMain, ScreenGui);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiObject {
    superclass: GuiBase2d,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiService {
    superclass: Instance,
    pub AutoSelectGuiEnabled: bool,
    pub GuiNavigationEnabled: bool,
    pub SelectedObject: Ref,
}
impl_inherits!(GuiService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuidRegistryService {
    superclass: Instance,
}
impl_inherits!(GuidRegistryService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HSRDataContentProvider {
    superclass: CacheableContentProvider,
}
impl_inherits!(HSRDataContentProvider, CacheableContentProvider);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HandleAdornment {
    superclass: PVAdornment,
    pub AdornCullingMode: enums::AdornCullingMode,
    pub AlwaysOnTop: bool,
    pub CFrame: CFrame,
    pub SizeRelativeOffset: Vector3,
    pub ZIndex: i32,
}
impl_inherits!(HandleAdornment, PVAdornment);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Handles {
    superclass: HandlesBase,
    pub Faces: Faces,
    pub Style: enums::HandlesStyle,
}
impl_inherits!(Handles, HandlesBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HandlesBase {
    superclass: PartAdornment,
}
impl_inherits!(HandlesBase, PartAdornment);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HapticEffect {
    superclass: Instance,
    pub Looped: bool,
    pub Position: Vector3,
    pub Radius: f32,
    pub Type: enums::HapticEffectType,
    pub Waveform: Ref,
}
impl_inherits!(HapticEffect, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HapticService {
    superclass: Instance,
}
impl_inherits!(HapticService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Hat {
    superclass: Accoutrement,
}
impl_inherits!(Hat, Accoutrement);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HeapProfilerService {
    superclass: Instance,
}
impl_inherits!(HeapProfilerService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HeatmapService {
    superclass: Instance,
}
impl_inherits!(HeatmapService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HeightmapImporterService {
    superclass: Instance,
}
impl_inherits!(HeightmapImporterService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HiddenSurfaceRemovalAsset {
    superclass: Instance,
    pub HsrData: BinaryString,
    pub HsrMeshIdData: BinaryString,
}
impl_inherits!(HiddenSurfaceRemovalAsset, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Highlight {
    superclass: Instance,
    pub Adornee: Ref,
    pub DepthMode: enums::HighlightDepthMode,
    pub Enabled: bool,
    pub FillColor: Color3,
    pub FillTransparency: f32,
    pub OutlineColor: Color3,
    pub OutlineTransparency: f32,
}
impl_inherits!(Highlight, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HingeConstraint {
    superclass: Constraint,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Hint {
    superclass: Message,
}
impl_inherits!(Hint, Message);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Hole {
    superclass: Feature,
}
impl_inherits!(Hole, Feature);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Hopper {
    superclass: Instance,
}
impl_inherits!(Hopper, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HopperBin {
    superclass: BackpackItem,
    pub Active: bool,
    pub BinType: enums::BinType,
}
impl_inherits!(HopperBin, BackpackItem);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HttpRbxApiService {
    superclass: Instance,
}
impl_inherits!(HttpRbxApiService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HttpRequest {
    superclass: Instance,
}
impl_inherits!(HttpRequest, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HttpService {
    superclass: Instance,
    pub HttpEnabled: bool,
}
impl_inherits!(HttpService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Humanoid {
    superclass: Instance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HumanoidController {
    superclass: Controller,
}
impl_inherits!(HumanoidController, Controller);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HumanoidDescription {
    superclass: Instance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HumanoidRigDescription {
    superclass: Instance,
    pub Chest: Ref,
    pub ChestRangeMax: Vector3,
    pub ChestRangeMin: Vector3,
    pub ChestSize: f32,
    pub ChestTposeAdjustment: CFrame,
    pub Head: Ref,
    pub HeadRangeMax: Vector3,
    pub HeadRangeMin: Vector3,
    pub HeadSize: f32,
    pub HeadTposeAdjustment: CFrame,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct IKControl {
    superclass: Instance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ILegacyStudioBridge {
    superclass: Instance,
}
impl_inherits!(ILegacyStudioBridge, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct IXPService {
    superclass: Instance,
}
impl_inherits!(IXPService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ImageButton {
    superclass: GuiButton,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ImageHandleAdornment {
    superclass: HandleAdornment,
    pub Image: ContentId,
    pub Size: Vector2,
}
impl_inherits!(ImageHandleAdornment, HandleAdornment);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ImageLabel {
    superclass: GuiLabel,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ImportSession {
    superclass: Instance,
}
impl_inherits!(ImportSession, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct IncrementalPatchBuilder {
    superclass: Instance,
    pub AddPathsToBundle: bool,
    pub BuildDebouncePeriod: f64,
    pub HighCompression: bool,
    pub SerializePatch: bool,
    pub ZstdCompression: bool,
}
impl_inherits!(IncrementalPatchBuilder, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InputAction {
    superclass: Instance,
    pub Enabled: bool,
    pub Type: enums::InputActionType,
}
impl_inherits!(InputAction, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InputBinding {
    superclass: Instance,
    pub Down: enums::KeyCode,
    pub KeyCode: enums::KeyCode,
    pub Left: enums::KeyCode,
    pub PressedThreshold: f32,
    pub ReleasedThreshold: f32,
    pub Right: enums::KeyCode,
    pub Scale: f32,
    pub UiButton: Ref,
    pub Up: enums::KeyCode,
    pub Vector2Scale: Vector2,
}
impl_inherits!(InputBinding, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InputContext {
    superclass: Instance,
    pub Enabled: bool,
    pub Priority: i32,
    pub Sink: bool,
}
impl_inherits!(InputContext, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InputObject {
    superclass: Instance,
}
impl_inherits!(InputObject, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InsertService {
    superclass: Instance,
    pub AllowClientInsertModels: bool,
    pub AllowInsertFreeModels: bool,
}
impl_inherits!(InsertService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Instance {
    superclass: Object,
    pub Capabilities: SecurityCapabilities,
    pub HistoryId: UniqueId,
    pub Name: String,
    pub SourceAssetId: i64,
    pub Tags: Tags,
    pub UniqueId: UniqueId,
}
impl_inherits!(Instance, Object);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InstanceAdornment {
    superclass: GuiBase3d,
    pub Adornee: Ref,
}
impl_inherits!(InstanceAdornment, GuiBase3d);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct IntConstrainedValue {
    superclass: ValueBase,
    pub MaxValue: i64,
    pub MinValue: i64,
    pub Value: i64,
}
impl_inherits!(IntConstrainedValue, ValueBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct IntValue {
    superclass: ValueBase,
    pub Value: i64,
}
impl_inherits!(IntValue, ValueBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InternalSyncItem {
    superclass: Instance,
    pub AutoSync: bool,
    pub Enabled: bool,
    pub Path: String,
}
impl_inherits!(InternalSyncItem, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InternalSyncService {
    superclass: Instance,
}
impl_inherits!(InternalSyncService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct IntersectOperation {
    superclass: PartOperation,
}
impl_inherits!(IntersectOperation, PartOperation);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InventoryPages {
    superclass: Pages,
}
impl_inherits!(InventoryPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct JointImportData {
    superclass: BaseImportData,
}
impl_inherits!(JointImportData, BaseImportData);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct JointInstance {
    superclass: Instance,
    pub C0: CFrame,
    pub C1: CFrame,
    pub Enabled: bool,
    pub Part0: Ref,
    pub Part1: Ref,
}
impl_inherits!(JointInstance, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct JointsService {
    superclass: Instance,
}
impl_inherits!(JointsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct KeyboardService {
    superclass: Instance,
}
impl_inherits!(KeyboardService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Keyframe {
    superclass: Instance,
    pub Time: f32,
}
impl_inherits!(Keyframe, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct KeyframeMarker {
    superclass: Instance,
    pub Value: String,
}
impl_inherits!(KeyframeMarker, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct KeyframeSequence {
    superclass: AnimationClip,
    pub AuthoredHipHeight: f32,
}
impl_inherits!(KeyframeSequence, AnimationClip);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct KeyframeSequenceProvider {
    superclass: Instance,
}
impl_inherits!(KeyframeSequenceProvider, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LSPFileSyncService {
    superclass: Instance,
}
impl_inherits!(LSPFileSyncService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LanguageService {
    superclass: Instance,
}
impl_inherits!(LanguageService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LayerCollector {
    superclass: GuiBase2d,
    pub Enabled: bool,
    pub ResetOnSpawn: bool,
    pub ZIndexBehavior: enums::ZIndexBehavior,
}
impl_inherits!(LayerCollector, GuiBase2d);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LegacyStudioBridge {
    superclass: ILegacyStudioBridge,
}
impl_inherits!(LegacyStudioBridge, ILegacyStudioBridge);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Light {
    superclass: Instance,
    pub Brightness: f32,
    pub Color: Color3,
    pub Enabled: bool,
    pub Shadows: bool,
}
impl_inherits!(Light, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Lighting {
    superclass: Instance,
    pub Ambient: Color3,
    pub Brightness: f32,
    pub ClockTime: f32,
    pub ColorShiftBottom: Color3,
    pub ColorShiftTop: Color3,
    pub EnvironmentDiffuseScale: f32,
    pub EnvironmentSpecularScale: f32,
    pub ExposureCompensation: f32,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LineForce {
    superclass: Constraint,
    pub ApplyAtCenterOfMass: bool,
    pub InverseSquareLaw: bool,
    pub Magnitude: f32,
    pub MaxForce: f32,
    pub ReactionForceEnabled: bool,
}
impl_inherits!(LineForce, Constraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LineHandleAdornment {
    superclass: HandleAdornment,
    pub Length: f32,
    pub Thickness: f32,
}
impl_inherits!(LineHandleAdornment, HandleAdornment);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LinearVelocity {
    superclass: Constraint,
    pub ForceLimitMode: enums::ForceLimitMode,
    pub ForceLimitsEnabled: bool,
    pub LineDirection: Vector3,
    pub LineVelocity: f32,
    pub MaxAxesForce: Vector3,
    pub MaxForce: f32,
    pub MaxPlanarAxesForce: Vector2,
    pub PlaneVelocity: Vector2,
    pub PrimaryTangentAxis: Vector3,
    pub RelativeTo: enums::ActuatorRelativeTo,
    pub SecondaryTangentAxis: Vector3,
    pub VectorVelocity: Vector3,
    pub VelocityConstraintMode: enums::VelocityConstraintMode,
}
impl_inherits!(LinearVelocity, Constraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LinkingService {
    superclass: Instance,
}
impl_inherits!(LinkingService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LiveScriptingService {
    superclass: Instance,
}
impl_inherits!(LiveScriptingService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LiveSyncService {
    superclass: Instance,
}
impl_inherits!(LiveSyncService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LocalDebuggerConnection {
    superclass: DebuggerConnection,
}
impl_inherits!(LocalDebuggerConnection, DebuggerConnection);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LocalScript {
    superclass: Script,
}
impl_inherits!(LocalScript, Script);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LocalStorageService {
    superclass: Instance,
}
impl_inherits!(LocalStorageService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LocalizationService {
    superclass: Instance,
}
impl_inherits!(LocalizationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LocalizationTable {
    superclass: Instance,
    pub Contents: String,
    pub SourceLocaleId: String,
}
impl_inherits!(LocalizationTable, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LodDataEntity {
    superclass: Instance,
}
impl_inherits!(LodDataEntity, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LodDataService {
    superclass: Instance,
}
impl_inherits!(LodDataService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LogReporterService {
    superclass: Instance,
}
impl_inherits!(LogReporterService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LogService {
    superclass: Instance,
}
impl_inherits!(LogService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LoginService {
    superclass: Instance,
}
impl_inherits!(LoginService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LuaSettings {
    superclass: Instance,
}
impl_inherits!(LuaSettings, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LuaSourceContainer {
    superclass: Instance,
    pub ScriptGuid: String,
}
impl_inherits!(LuaSourceContainer, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LuaWebService {
    superclass: Instance,
}
impl_inherits!(LuaWebService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LuauScriptAnalyzerService {
    superclass: Instance,
}
impl_inherits!(LuauScriptAnalyzerService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MLModelDeliveryService {
    superclass: Instance,
}
impl_inherits!(MLModelDeliveryService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ManualGlue {
    superclass: ManualSurfaceJointInstance,
}
impl_inherits!(ManualGlue, ManualSurfaceJointInstance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ManualSurfaceJointInstance {
    superclass: JointInstance,
}
impl_inherits!(ManualSurfaceJointInstance, JointInstance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ManualWeld {
    superclass: ManualSurfaceJointInstance,
}
impl_inherits!(ManualWeld, ManualSurfaceJointInstance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MarkerCurve {
    superclass: Instance,
    pub ValuesAndTimes: BinaryString,
}
impl_inherits!(MarkerCurve, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MarketplaceService {
    superclass: Instance,
}
impl_inherits!(MarketplaceService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MatchmakingService {
    superclass: Instance,
}
impl_inherits!(MatchmakingService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MaterialGenerationService {
    superclass: Instance,
}
impl_inherits!(MaterialGenerationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MaterialGenerationSession {
    superclass: Instance,
}
impl_inherits!(MaterialGenerationSession, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MaterialImportData {
    superclass: BaseImportData,
    pub DiffuseFilePath: String,
    pub MetalnessFilePath: String,
    pub NormalFilePath: String,
    pub RoughnessFilePath: String,
}
impl_inherits!(MaterialImportData, BaseImportData);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MaterialService {
    superclass: Instance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MaterialVariant {
    superclass: Instance,
    pub BaseMaterial: enums::Material,
    pub ColorMap: ContentId,
    pub CustomPhysicalProperties: PhysicalProperties,
    pub MaterialPattern: enums::MaterialPattern,
    pub MetalnessMap: ContentId,
    pub NormalMap: ContentId,
    pub RoughnessMap: ContentId,
    pub StudsPerTile: f32,
    pub TexturePack: ContentId,
}
impl_inherits!(MaterialVariant, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MemStorageConnection {
    superclass: Instance,
}
impl_inherits!(MemStorageConnection, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MemStorageService {
    superclass: Instance,
}
impl_inherits!(MemStorageService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MemoryStoreHashMap {
    superclass: Instance,
}
impl_inherits!(MemoryStoreHashMap, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MemoryStoreHashMapPages {
    superclass: Pages,
}
impl_inherits!(MemoryStoreHashMapPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MemoryStoreQueue {
    superclass: Instance,
}
impl_inherits!(MemoryStoreQueue, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MemoryStoreService {
    superclass: Instance,
}
impl_inherits!(MemoryStoreService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MemoryStoreSortedMap {
    superclass: Instance,
}
impl_inherits!(MemoryStoreSortedMap, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MeshContentProvider {
    superclass: CacheableContentProvider,
}
impl_inherits!(MeshContentProvider, CacheableContentProvider);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MeshImportData {
    superclass: BaseImportData,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MeshPart {
    superclass: TriangleMeshPart,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Message {
    superclass: Instance,
    pub Text: String,
}
impl_inherits!(Message, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MessageBusConnection {
    superclass: Instance,
}
impl_inherits!(MessageBusConnection, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MessageBusService {
    superclass: Instance,
}
impl_inherits!(MessageBusService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MessagingService {
    superclass: Instance,
}
impl_inherits!(MessagingService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MetaBreakpoint {
    superclass: Instance,
    pub Condition: String,
    pub ContinueExecution: bool,
    pub Enabled: bool,
    pub Line: i32,
    pub LogMessage: String,
    pub RemoveOnHit: bool,
    pub Script: String,
}
impl_inherits!(MetaBreakpoint, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MetaBreakpointContext {
    superclass: Instance,
    pub ContextDataInternal: String,
}
impl_inherits!(MetaBreakpointContext, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MetaBreakpointManager {
    superclass: Instance,
}
impl_inherits!(MetaBreakpointManager, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Model {
    superclass: PVInstance,
    pub LevelOfDetail: enums::ModelLevelOfDetail,
    pub ModelMeshCFrame: CFrame,
    pub ModelMeshData: SharedString,
    pub ModelMeshSize: Vector3,
    pub ModelStreamingMode: enums::ModelStreamingMode,
    pub NeedsPivotMigration: bool,
    pub PrimaryPart: Ref,
    pub WorldPivotData: Option<CFrame>,
}
impl_inherits!(Model, PVInstance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ModuleScript {
    superclass: LuaSourceContainer,
    pub LinkedSource: ContentId,
    pub Source: String,
}
impl_inherits!(ModuleScript, LuaSourceContainer);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Motor {
    superclass: JointInstance,
    pub DesiredAngle: f32,
    pub MaxVelocity: f32,
}
impl_inherits!(Motor, JointInstance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Motor6D {
    superclass: Motor,
}
impl_inherits!(Motor6D, Motor);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MotorFeature {
    superclass: Feature,
}
impl_inherits!(MotorFeature, Feature);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Mouse {
    superclass: Instance,
    pub Icon: ContentId,
    pub TargetFilter: Ref,
}
impl_inherits!(Mouse, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MouseService {
    superclass: Instance,
}
impl_inherits!(MouseService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MultipleDocumentInterfaceInstance {
    superclass: Instance,
}
impl_inherits!(MultipleDocumentInterfaceInstance, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NegateOperation {
    superclass: PartOperation,
}
impl_inherits!(NegateOperation, PartOperation);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NetworkClient {
    superclass: NetworkPeer,
}
impl_inherits!(NetworkClient, NetworkPeer);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NetworkMarker {
    superclass: Instance,
}
impl_inherits!(NetworkMarker, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NetworkPeer {
    superclass: Instance,
}
impl_inherits!(NetworkPeer, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NetworkReplicator {
    superclass: Instance,
}
impl_inherits!(NetworkReplicator, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NetworkServer {
    superclass: NetworkPeer,
}
impl_inherits!(NetworkServer, NetworkPeer);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NetworkSettings {
    superclass: Instance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NoCollisionConstraint {
    superclass: Instance,
    pub Enabled: bool,
    pub Part0: Ref,
    pub Part1: Ref,
}
impl_inherits!(NoCollisionConstraint, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Noise {
    superclass: Instance,
    pub NoiseType: enums::NoiseType,
    pub Seed: i32,
}
impl_inherits!(Noise, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NonReplicatedCSGDictionaryService {
    superclass: FlyweightService,
}
impl_inherits!(NonReplicatedCSGDictionaryService, FlyweightService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NotificationService {
    superclass: Instance,
}
impl_inherits!(NotificationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NumberPose {
    superclass: PoseBase,
    pub Value: f64,
}
impl_inherits!(NumberPose, PoseBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NumberValue {
    superclass: ValueBase,
    pub Value: f64,
}
impl_inherits!(NumberValue, ValueBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Object {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ObjectValue {
    superclass: ValueBase,
    pub Value: Ref,
}
impl_inherits!(ObjectValue, ValueBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct OmniRecommendationsService {
    superclass: Instance,
}
impl_inherits!(OmniRecommendationsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct OpenCloudApiV1 {
    superclass: Instance,
}
impl_inherits!(OpenCloudApiV1, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct OpenCloudService {
    superclass: Instance,
}
impl_inherits!(OpenCloudService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct OperationGraph {
    superclass: Instance,
}
impl_inherits!(OperationGraph, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct OrderedDataStore {
    superclass: GlobalDataStore,
}
impl_inherits!(OrderedDataStore, GlobalDataStore);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct OutfitPages {
    superclass: Pages,
}
impl_inherits!(OutfitPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PVAdornment {
    superclass: GuiBase3d,
    pub Adornee: Ref,
}
impl_inherits!(PVAdornment, GuiBase3d);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PVInstance {
    superclass: Instance,
}
impl_inherits!(PVInstance, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PackageLink {
    superclass: Instance,
    pub AutoUpdate: bool,
    pub DefaultName: String,
    pub ModifiedState: i32,
    pub SerializedDefaultAttributes: BinaryString,
    pub VersionIdSerialize: i64,
}
impl_inherits!(PackageLink, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PackageService {
    superclass: Instance,
}
impl_inherits!(PackageService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PackageUIService {
    superclass: Instance,
}
impl_inherits!(PackageUIService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Pages {
    superclass: Instance,
}
impl_inherits!(Pages, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Pants {
    superclass: Clothing,
    pub PantsTemplate: ContentId,
}
impl_inherits!(Pants, Clothing);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ParabolaAdornment {
    superclass: PVAdornment,
}
impl_inherits!(ParabolaAdornment, PVAdornment);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Part {
    superclass: FormFactorPart,
}
impl_inherits!(Part, FormFactorPart);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PartAdornment {
    superclass: GuiBase3d,
    pub Adornee: Ref,
}
impl_inherits!(PartAdornment, GuiBase3d);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PartOperation {
    superclass: TriangleMeshPart,
    pub AssetId: ContentId,
    pub ChildData: BinaryString,
    pub ChildData2: SharedString,
    pub FormFactor: enums::FormFactor,
    pub InitialSize: Vector3,
    pub MeshData: BinaryString,
    pub MeshData2: SharedString,
    pub PhysicsData: BinaryString,
    pub RenderFidelity: enums::RenderFidelity,
    pub SmoothingAngle: f32,
    pub UsePartColor: bool,
}
impl_inherits!(PartOperation, TriangleMeshPart);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PartOperationAsset {
    superclass: Instance,
    pub ChildData: BinaryString,
    pub MeshData: BinaryString,
}
impl_inherits!(PartOperationAsset, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ParticleEmitter {
    superclass: Instance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PatchBundlerFileWatch {
    superclass: Instance,
}
impl_inherits!(PatchBundlerFileWatch, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PatchMapping {
    superclass: Instance,
    pub FlattenTree: bool,
    pub PatchId: String,
    pub TargetPath: String,
}
impl_inherits!(PatchMapping, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Path {
    superclass: Instance,
}
impl_inherits!(Path, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Path2D {
    superclass: GuiBase,
    pub Closed: bool,
    pub Color3: Color3,
    pub PropertiesSerialize: BinaryString,
    pub Thickness: f32,
    pub Transparency: f32,
    pub Visible: bool,
    pub ZIndex: i32,
}
impl_inherits!(Path2D, GuiBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PathfindingLink {
    superclass: Instance,
    pub Attachment0: Ref,
    pub Attachment1: Ref,
    pub IsBidirectional: bool,
    pub Label: String,
}
impl_inherits!(PathfindingLink, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PathfindingModifier {
    superclass: Instance,
    pub Label: String,
    pub PassThrough: bool,
}
impl_inherits!(PathfindingModifier, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PathfindingService {
    superclass: Instance,
}
impl_inherits!(PathfindingService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PausedState {
    superclass: Instance,
}
impl_inherits!(PausedState, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PausedStateBreakpoint {
    superclass: PausedState,
}
impl_inherits!(PausedStateBreakpoint, PausedState);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PausedStateException {
    superclass: PausedState,
}
impl_inherits!(PausedStateException, PausedState);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PerformanceControlService {
    superclass: Instance,
}
impl_inherits!(PerformanceControlService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PermissionsService {
    superclass: Instance,
}
impl_inherits!(PermissionsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PhysicsService {
    superclass: Instance,
}
impl_inherits!(PhysicsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PhysicsSettings {
    superclass: Instance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PitchShiftSoundEffect {
    superclass: SoundEffect,
    pub Octave: f32,
}
impl_inherits!(PitchShiftSoundEffect, SoundEffect);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlaceAssetIdsService {
    superclass: Instance,
}
impl_inherits!(PlaceAssetIdsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlaceStatsService {
    superclass: Instance,
}
impl_inherits!(PlaceStatsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlacesService {
    superclass: Instance,
}
impl_inherits!(PlacesService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Plane {
    superclass: PlaneConstraint,
}
impl_inherits!(Plane, PlaneConstraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlaneConstraint {
    superclass: Constraint,
}
impl_inherits!(PlaneConstraint, Constraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Platform {
    superclass: Part,
}
impl_inherits!(Platform, Part);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlatformCloudStorageService {
    superclass: Instance,
}
impl_inherits!(PlatformCloudStorageService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlatformFriendsService {
    superclass: Instance,
}
impl_inherits!(PlatformFriendsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Player {
    superclass: Instance,
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
    pub GameplayPaused: bool,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerData {
    superclass: Instance,
}
impl_inherits!(PlayerData, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerDataRecord {
    superclass: Instance,
}
impl_inherits!(PlayerDataRecord, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerDataRecordConfig {
    superclass: Instance,
}
impl_inherits!(PlayerDataRecordConfig, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerDataService {
    superclass: Instance,
    pub LoadFailureBehavior: enums::PlayerDataLoadFailureBehavior,
}
impl_inherits!(PlayerDataService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerEmulatorService {
    superclass: Instance,
    pub CustomPoliciesEnabled: bool,
    pub EmulatedCountryCode: String,
    pub EmulatedGameLocale: String,
    pub PlayerEmulationEnabled: bool,
    pub PseudolocalizationEnabled: bool,
    pub SerializedEmulatedPolicyInfo: BinaryString,
    pub TextElongationFactor: i32,
}
impl_inherits!(PlayerEmulatorService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerGui {
    superclass: BasePlayerGui,
    pub ScreenOrientation: enums::ScreenOrientation,
    pub SelectionImageObject: Ref,
}
impl_inherits!(PlayerGui, BasePlayerGui);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerHydrationService {
    superclass: Instance,
}
impl_inherits!(PlayerHydrationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerMouse {
    superclass: Mouse,
}
impl_inherits!(PlayerMouse, Mouse);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerScripts {
    superclass: Instance,
}
impl_inherits!(PlayerScripts, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerViewService {
    superclass: Instance,
}
impl_inherits!(PlayerViewService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Players {
    superclass: Instance,
    pub BanningEnabled: bool,
    pub CharacterAutoLoads: bool,
    pub RespawnTime: f32,
    pub UseStrafingAnimations: bool,
}
impl_inherits!(Players, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Plugin {
    superclass: Instance,
    pub DisableUiDragDetectorDrags: bool,
    pub IsDebuggable: bool,
}
impl_inherits!(Plugin, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginAction {
    superclass: Instance,
}
impl_inherits!(PluginAction, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginCapabilities {
    superclass: Instance,
    pub Manifest: String,
}
impl_inherits!(PluginCapabilities, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginDebugService {
    superclass: Instance,
}
impl_inherits!(PluginDebugService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginDragEvent {
    superclass: Instance,
}
impl_inherits!(PluginDragEvent, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginGui {
    superclass: LayerCollector,
    pub Title: String,
}
impl_inherits!(PluginGui, LayerCollector);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginGuiService {
    superclass: Instance,
}
impl_inherits!(PluginGuiService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginManagementService {
    superclass: Instance,
}
impl_inherits!(PluginManagementService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginManager {
    superclass: Instance,
}
impl_inherits!(PluginManager, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginManagerInterface {
    superclass: Instance,
}
impl_inherits!(PluginManagerInterface, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginMenu {
    superclass: Instance,
}
impl_inherits!(PluginMenu, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginMouse {
    superclass: Mouse,
}
impl_inherits!(PluginMouse, Mouse);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginPolicyService {
    superclass: Instance,
}
impl_inherits!(PluginPolicyService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginToolbar {
    superclass: Instance,
}
impl_inherits!(PluginToolbar, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginToolbarButton {
    superclass: Instance,
}
impl_inherits!(PluginToolbarButton, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PointLight {
    superclass: Light,
    pub Range: f32,
}
impl_inherits!(PointLight, Light);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PointsService {
    superclass: Instance,
}
impl_inherits!(PointsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PolicyService {
    superclass: Instance,
    pub IsLuobuServer: enums::TriStateBoolean,
    pub LuobuWhitelisted: enums::TriStateBoolean,
}
impl_inherits!(PolicyService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Pose {
    superclass: PoseBase,
    pub CFrame: CFrame,
}
impl_inherits!(Pose, PoseBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PoseBase {
    superclass: Instance,
    pub EasingDirection: enums::PoseEasingDirection,
    pub EasingStyle: enums::PoseEasingStyle,
    pub Weight: f32,
}
impl_inherits!(PoseBase, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PostEffect {
    superclass: Instance,
    pub Enabled: bool,
}
impl_inherits!(PostEffect, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PrismaticConstraint {
    superclass: SlidingBallConstraint,
}
impl_inherits!(PrismaticConstraint, SlidingBallConstraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ProcessInstancePhysicsService {
    superclass: Instance,
}
impl_inherits!(ProcessInstancePhysicsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ProximityPrompt {
    superclass: Instance,
    pub ActionText: String,
    pub AutoLocalize: bool,
    pub ClickablePrompt: bool,
    pub Enabled: bool,
    pub Exclusivity: enums::ProximityPromptExclusivity,
    pub GamepadKeyCode: enums::KeyCode,
    pub HoldDuration: f32,
    pub KeyboardKeyCode: enums::KeyCode,
    pub MaxActivationDistance: f32,
    pub ObjectText: String,
    pub RequiresLineOfSight: bool,
    pub RootLocalizationTable: Ref,
    pub Style: enums::ProximityPromptStyle,
    pub UiOffset: Vector2,
}
impl_inherits!(ProximityPrompt, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ProximityPromptService {
    superclass: Instance,
    pub Enabled: bool,
    pub MaxPromptsVisible: i32,
}
impl_inherits!(ProximityPromptService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PublishService {
    superclass: Instance,
}
impl_inherits!(PublishService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct QWidgetPluginGui {
    superclass: PluginGui,
}
impl_inherits!(QWidgetPluginGui, PluginGui);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RTAnimationTracker {
    superclass: Instance,
}
impl_inherits!(RTAnimationTracker, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RayValue {
    superclass: ValueBase,
    pub Value: Ray,
}
impl_inherits!(RayValue, ValueBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RbxAnalyticsService {
    superclass: Instance,
}
impl_inherits!(RbxAnalyticsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadata {
    superclass: Instance,
}
impl_inherits!(ReflectionMetadata, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataCallbacks {
    superclass: Instance,
}
impl_inherits!(ReflectionMetadataCallbacks, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataClass {
    superclass: ReflectionMetadataItem,
    pub ExplorerImageIndex: i32,
    pub ExplorerOrder: i32,
    pub Insertable: bool,
    pub PreferredParent: String,
    pub ServiceVisibility: enums::ServiceVisibility,
}
impl_inherits!(ReflectionMetadataClass, ReflectionMetadataItem);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataClasses {
    superclass: Instance,
}
impl_inherits!(ReflectionMetadataClasses, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataEnum {
    superclass: ReflectionMetadataItem,
}
impl_inherits!(ReflectionMetadataEnum, ReflectionMetadataItem);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataEnumItem {
    superclass: ReflectionMetadataItem,
}
impl_inherits!(ReflectionMetadataEnumItem, ReflectionMetadataItem);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataEnums {
    superclass: Instance,
}
impl_inherits!(ReflectionMetadataEnums, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataEvents {
    superclass: Instance,
}
impl_inherits!(ReflectionMetadataEvents, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataFunctions {
    superclass: Instance,
}
impl_inherits!(ReflectionMetadataFunctions, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataItem {
    superclass: Instance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataMember {
    superclass: ReflectionMetadataItem,
}
impl_inherits!(ReflectionMetadataMember, ReflectionMetadataItem);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataProperties {
    superclass: Instance,
}
impl_inherits!(ReflectionMetadataProperties, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataYieldFunctions {
    superclass: Instance,
}
impl_inherits!(ReflectionMetadataYieldFunctions, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionService {
    superclass: Instance,
}
impl_inherits!(ReflectionService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RelativeGui {
    superclass: GuiObject,
}
impl_inherits!(RelativeGui, GuiObject);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RemoteCursorService {
    superclass: Instance,
}
impl_inherits!(RemoteCursorService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RemoteDebuggerServer {
    superclass: Instance,
}
impl_inherits!(RemoteDebuggerServer, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RemoteEvent {
    superclass: BaseRemoteEvent,
}
impl_inherits!(RemoteEvent, BaseRemoteEvent);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RemoteFunction {
    superclass: Instance,
}
impl_inherits!(RemoteFunction, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RenderSettings {
    superclass: Instance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RenderingTest {
    superclass: Instance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReplicatedFirst {
    superclass: Instance,
}
impl_inherits!(ReplicatedFirst, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReplicatedStorage {
    superclass: Instance,
}
impl_inherits!(ReplicatedStorage, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReverbSoundEffect {
    superclass: SoundEffect,
    pub DecayTime: f32,
    pub Density: f32,
    pub Diffusion: f32,
    pub DryLevel: f32,
    pub WetLevel: f32,
}
impl_inherits!(ReverbSoundEffect, SoundEffect);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RibbonNotificationService {
    superclass: Instance,
}
impl_inherits!(RibbonNotificationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RigidConstraint {
    superclass: Constraint,
}
impl_inherits!(RigidConstraint, Constraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RobloxPluginGuiService {
    superclass: Instance,
}
impl_inherits!(RobloxPluginGuiService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RobloxReplicatedStorage {
    superclass: Instance,
}
impl_inherits!(RobloxReplicatedStorage, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RobloxSerializableInstance {
    superclass: Instance,
    pub Data: BinaryString,
}
impl_inherits!(RobloxSerializableInstance, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RobloxServerStorage {
    superclass: Instance,
}
impl_inherits!(RobloxServerStorage, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RocketPropulsion {
    superclass: BodyMover,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RodConstraint {
    superclass: Constraint,
    pub Length: f32,
    pub LimitAngle0: f32,
    pub LimitAngle1: f32,
    pub LimitsEnabled: bool,
    pub Thickness: f32,
}
impl_inherits!(RodConstraint, Constraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RomarkRbxAnalyticsService {
    superclass: Instance,
}
impl_inherits!(RomarkRbxAnalyticsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RomarkService {
    superclass: Instance,
}
impl_inherits!(RomarkService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RootImportData {
    superclass: BaseImportData,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RopeConstraint {
    superclass: Constraint,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Rotate {
    superclass: JointInstance,
}
impl_inherits!(Rotate, JointInstance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RotateP {
    superclass: DynamicRotate,
}
impl_inherits!(RotateP, DynamicRotate);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RotateV {
    superclass: DynamicRotate,
}
impl_inherits!(RotateV, DynamicRotate);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RotationCurve {
    superclass: Instance,
    pub ValuesAndTimes: BinaryString,
}
impl_inherits!(RotationCurve, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RtMessagingService {
    superclass: Instance,
}
impl_inherits!(RtMessagingService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RunService {
    superclass: Instance,
}
impl_inherits!(RunService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RunningAverageItemDouble {
    superclass: StatsItem,
}
impl_inherits!(RunningAverageItemDouble, StatsItem);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RunningAverageItemInt {
    superclass: StatsItem,
}
impl_inherits!(RunningAverageItemInt, StatsItem);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RunningAverageTimeIntervalItem {
    superclass: StatsItem,
}
impl_inherits!(RunningAverageTimeIntervalItem, StatsItem);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RuntimeScriptService {
    superclass: Instance,
}
impl_inherits!(RuntimeScriptService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SafetyService {
    superclass: Instance,
    pub IsCaptureModeForReport: bool,
}
impl_inherits!(SafetyService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScreenGui {
    superclass: LayerCollector,
    pub ClipToDeviceSafeArea: bool,
    pub DisplayOrder: i32,
    pub SafeAreaCompatibility: enums::SafeAreaCompatibility,
    pub ScreenInsets: enums::ScreenInsets,
}
impl_inherits!(ScreenGui, LayerCollector);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScreenshotCapture {
    superclass: Capture,
}
impl_inherits!(ScreenshotCapture, Capture);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScreenshotHud {
    superclass: Instance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Script {
    superclass: BaseScript,
    pub Source: String,
}
impl_inherits!(Script, BaseScript);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptBuilder {
    superclass: Instance,
}
impl_inherits!(ScriptBuilder, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptChangeService {
    superclass: Instance,
}
impl_inherits!(ScriptChangeService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptCloneWatcher {
    superclass: Instance,
}
impl_inherits!(ScriptCloneWatcher, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptCloneWatcherHelper {
    superclass: Instance,
}
impl_inherits!(ScriptCloneWatcherHelper, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptCommitService {
    superclass: Instance,
}
impl_inherits!(ScriptCommitService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptContext {
    superclass: Instance,
}
impl_inherits!(ScriptContext, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptDebugger {
    superclass: Instance,
    pub CoreScriptIdentifier: String,
    pub ScriptGuid: String,
}
impl_inherits!(ScriptDebugger, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptDocument {
    superclass: Instance,
}
impl_inherits!(ScriptDocument, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptEditorService {
    superclass: Instance,
}
impl_inherits!(ScriptEditorService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptProfilerService {
    superclass: Instance,
}
impl_inherits!(ScriptProfilerService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptRegistrationService {
    superclass: Instance,
}
impl_inherits!(ScriptRegistrationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptRuntime {
    superclass: Instance,
}
impl_inherits!(ScriptRuntime, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptService {
    superclass: Instance,
}
impl_inherits!(ScriptService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScrollingFrame {
    superclass: GuiObject,
    pub AutomaticCanvasSize: enums::AutomaticSize,
    pub BottomImage: ContentId,
    pub CanvasPosition: Vector2,
    pub CanvasSize: UDim2,
    pub ElasticBehavior: enums::ElasticBehavior,
    pub HorizontalScrollBarInset: enums::ScrollBarInset,
    pub MidImage: ContentId,
    pub ScrollBarImageColor3: Color3,
    pub ScrollBarImageTransparency: f32,
    pub ScrollBarThickness: i32,
    pub ScrollingDirection: enums::ScrollingDirection,
    pub ScrollingEnabled: bool,
    pub TopImage: ContentId,
    pub VerticalScrollBarInset: enums::ScrollBarInset,
    pub VerticalScrollBarPosition: enums::VerticalScrollBarPosition,
}
impl_inherits!(ScrollingFrame, GuiObject);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Seat {
    superclass: Part,
    pub Disabled: bool,
}
impl_inherits!(Seat, Part);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Selection {
    superclass: Instance,
}
impl_inherits!(Selection, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionBox {
    superclass: InstanceAdornment,
    pub LineThickness: f32,
    pub StudioSelectionBox: bool,
    pub SurfaceColor3: Color3,
    pub SurfaceTransparency: f32,
}
impl_inherits!(SelectionBox, InstanceAdornment);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionHighlightManager {
    superclass: Instance,
}
impl_inherits!(SelectionHighlightManager, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionLasso {
    superclass: GuiBase3d,
    pub Humanoid: Ref,
}
impl_inherits!(SelectionLasso, GuiBase3d);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionPartLasso {
    superclass: SelectionLasso,
    pub Part: Ref,
}
impl_inherits!(SelectionPartLasso, SelectionLasso);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionPointLasso {
    superclass: SelectionLasso,
    pub Point: Vector3,
}
impl_inherits!(SelectionPointLasso, SelectionLasso);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionSphere {
    superclass: PVAdornment,
    pub SurfaceColor3: Color3,
    pub SurfaceTransparency: f32,
}
impl_inherits!(SelectionSphere, PVAdornment);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SensorBase {
    superclass: Instance,
    pub UpdateType: enums::SensorUpdateType,
}
impl_inherits!(SensorBase, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SerializationService {
    superclass: Instance,
}
impl_inherits!(SerializationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ServerReplicator {
    superclass: NetworkReplicator,
}
impl_inherits!(ServerReplicator, NetworkReplicator);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ServerScriptService {
    superclass: Instance,
    pub LoadStringEnabled: bool,
}
impl_inherits!(ServerScriptService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ServerStorage {
    superclass: Instance,
}
impl_inherits!(ServerStorage, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ServiceProvider {
    superclass: Instance,
}
impl_inherits!(ServiceProvider, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ServiceVisibilityService {
    superclass: Instance,
    pub HiddenServices: BinaryString,
    pub VisibleServices: BinaryString,
}
impl_inherits!(ServiceVisibilityService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SessionService {
    superclass: Instance,
}
impl_inherits!(SessionService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SharedTableRegistry {
    superclass: Instance,
}
impl_inherits!(SharedTableRegistry, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Shirt {
    superclass: Clothing,
    pub ShirtTemplate: ContentId,
}
impl_inherits!(Shirt, Clothing);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ShirtGraphic {
    superclass: CharacterAppearance,
    pub Color3: Color3,
    pub Graphic: ContentId,
}
impl_inherits!(ShirtGraphic, CharacterAppearance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SkateboardController {
    superclass: Controller,
}
impl_inherits!(SkateboardController, Controller);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SkateboardPlatform {
    superclass: Part,
    pub Steer: i32,
    pub StickyWheels: bool,
    pub Throttle: i32,
}
impl_inherits!(SkateboardPlatform, Part);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Skin {
    superclass: CharacterAppearance,
    pub SkinColor: BrickColor,
}
impl_inherits!(Skin, CharacterAppearance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Sky {
    superclass: Instance,
    pub CelestialBodiesShown: bool,
    pub MoonAngularSize: f32,
    pub MoonTextureId: ContentId,
    pub SkyboxBk: ContentId,
    pub SkyboxDn: ContentId,
    pub SkyboxFt: ContentId,
    pub SkyboxLf: ContentId,
    pub SkyboxRt: ContentId,
    pub SkyboxUp: ContentId,
    pub StarCount: i32,
    pub SunAngularSize: f32,
    pub SunTextureId: ContentId,
}
impl_inherits!(Sky, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SlidingBallConstraint {
    superclass: Constraint,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Smoke {
    superclass: Instance,
    pub Color: Color3,
    pub Enabled: bool,
    pub TimeScale: f32,
}
impl_inherits!(Smoke, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SmoothVoxelsUpgraderService {
    superclass: Instance,
}
impl_inherits!(SmoothVoxelsUpgraderService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Snap {
    superclass: JointInstance,
}
impl_inherits!(Snap, JointInstance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SnippetService {
    superclass: Instance,
}
impl_inherits!(SnippetService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SocialService {
    superclass: Instance,
}
impl_inherits!(SocialService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SolidModelContentProvider {
    superclass: CacheableContentProvider,
}
impl_inherits!(SolidModelContentProvider, CacheableContentProvider);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Sound {
    superclass: Instance,
    pub LoopRegion: NumberRange,
    pub Looped: bool,
    pub PlayOnRemove: bool,
    pub PlaybackRegion: NumberRange,
    pub PlaybackRegionsEnabled: bool,
    pub PlaybackSpeed: f32,
    pub Playing: bool,
    pub RollOffMode: enums::RollOffMode,
    pub SoundGroup: Ref,
    pub SoundId: ContentId,
    pub TimePosition: f64,
    pub Volume: f32,
}
impl_inherits!(Sound, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SoundEffect {
    superclass: Instance,
    pub Enabled: bool,
    pub Priority: i32,
}
impl_inherits!(SoundEffect, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SoundGroup {
    superclass: Instance,
    pub Volume: f32,
}
impl_inherits!(SoundGroup, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SoundService {
    superclass: Instance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Sparkles {
    superclass: Instance,
    pub Enabled: bool,
    pub SparkleColor: Color3,
    pub TimeScale: f32,
}
impl_inherits!(Sparkles, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SpawnLocation {
    superclass: Part,
    pub AllowTeamChangeOnTouch: bool,
    pub Duration: i32,
    pub Enabled: bool,
    pub Neutral: bool,
    pub TeamColor: BrickColor,
}
impl_inherits!(SpawnLocation, Part);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SpawnerService {
    superclass: Instance,
}
impl_inherits!(SpawnerService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SpecialMesh {
    superclass: FileMesh,
    pub MeshType: enums::MeshType,
}
impl_inherits!(SpecialMesh, FileMesh);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SphereHandleAdornment {
    superclass: HandleAdornment,
    pub Radius: f32,
}
impl_inherits!(SphereHandleAdornment, HandleAdornment);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SpotLight {
    superclass: Light,
    pub Angle: f32,
    pub Face: enums::NormalId,
    pub Range: f32,
}
impl_inherits!(SpotLight, Light);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SpringConstraint {
    superclass: Constraint,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StackFrame {
    superclass: Instance,
}
impl_inherits!(StackFrame, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StandalonePluginScripts {
    superclass: Instance,
}
impl_inherits!(StandalonePluginScripts, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StandardPages {
    superclass: Pages,
}
impl_inherits!(StandardPages, Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StartPageService {
    superclass: Instance,
}
impl_inherits!(StartPageService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StarterCharacterScripts {
    superclass: StarterPlayerScripts,
}
impl_inherits!(StarterCharacterScripts, StarterPlayerScripts);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StarterGear {
    superclass: Instance,
}
impl_inherits!(StarterGear, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StarterGui {
    superclass: BasePlayerGui,
    pub ResetPlayerGuiOnSpawn: bool,
    pub RtlTextSupport: enums::RtlTextSupport,
    pub ScreenOrientation: enums::ScreenOrientation,
    pub ShowDevelopmentGui: bool,
    pub StudioDefaultStyleSheet: Ref,
    pub StudioInsertWidgetLayerCollectorAutoLinkStyleSheet: Ref,
    pub VirtualCursorMode: enums::VirtualCursorMode,
}
impl_inherits!(StarterGui, BasePlayerGui);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StarterPack {
    superclass: Instance,
}
impl_inherits!(StarterPack, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StarterPlayer {
    superclass: Instance,
    pub AllowCustomAnimations: bool,
    pub AutoJumpEnabled: bool,
    pub AvatarJointUpgradeSerializedRollout: enums::RolloutState,
    pub CameraMaxZoomDistance: f32,
    pub CameraMinZoomDistance: f32,
    pub CameraMode: enums::CameraMode,
    pub CharacterJumpHeight: f32,
    pub CharacterJumpPower: f32,
    pub CharacterMaxSlopeAngle: f32,
    pub CharacterUseJumpPower: bool,
    pub CharacterWalkSpeed: f32,
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
    pub RagdollDeath: bool,
    pub UserEmotesEnabled: bool,
}
impl_inherits!(StarterPlayer, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StarterPlayerScripts {
    superclass: Instance,
}
impl_inherits!(StarterPlayerScripts, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StartupMessageService {
    superclass: Instance,
}
impl_inherits!(StartupMessageService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Stats {
    superclass: Instance,
}
impl_inherits!(Stats, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StatsItem {
    superclass: Instance,
}
impl_inherits!(StatsItem, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Status {
    superclass: Model,
}
impl_inherits!(Status, Model);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StopWatchReporter {
    superclass: Instance,
}
impl_inherits!(StopWatchReporter, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StreamingService {
    superclass: Instance,
}
impl_inherits!(StreamingService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StringValue {
    superclass: ValueBase,
    pub Value: String,
}
impl_inherits!(StringValue, ValueBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Studio {
    superclass: Instance,
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
    pub CameraTweenFocus: bool,
    pub CameraZoomSpeed: f32,
    pub CameraZoomToMousePosition: bool,
    pub ClearOutputOnStart: bool,
    pub CommandBarLocalState: bool,
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
    pub EnableTemporaryTabs: bool,
    pub EnableTemporaryTabsInExplorer: bool,
    pub EnableTypeHover: bool,
    pub FormatOnPaste: bool,
    pub FormatOnType: bool,
    pub FreeCameraSpeedScroll: bool,
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
    pub LoadUserPluginsInRunModes: bool,
    pub LuaDebuggerEnabled: bool,
    pub MainVolume: f32,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioAssetService {
    superclass: Instance,
}
impl_inherits!(StudioAssetService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioAttachment {
    superclass: Instance,
    pub AutoHideParent: bool,
    pub IsArrowVisible: bool,
    pub Offset: Vector2,
    pub SourceAnchorPoint: Vector2,
    pub TargetAnchorPoint: Vector2,
}
impl_inherits!(StudioAttachment, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioCallout {
    superclass: Instance,
}
impl_inherits!(StudioCallout, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioCameraService {
    superclass: Instance,
    pub LockCameraSpeed: bool,
}
impl_inherits!(StudioCameraService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioData {
    superclass: Instance,
    pub EnableScriptCollabByDefaultOnLoad: bool,
}
impl_inherits!(StudioData, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioDeviceEmulatorService {
    superclass: Instance,
}
impl_inherits!(StudioDeviceEmulatorService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioObjectBase {
    superclass: Instance,
}
impl_inherits!(StudioObjectBase, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioPublishService {
    superclass: Instance,
    pub PublishLocked: bool,
}
impl_inherits!(StudioPublishService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioScriptDebugEventListener {
    superclass: Instance,
}
impl_inherits!(StudioScriptDebugEventListener, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioSdkService {
    superclass: Instance,
}
impl_inherits!(StudioSdkService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioService {
    superclass: Instance,
    pub Secrets: String,
}
impl_inherits!(StudioService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioTheme {
    superclass: Instance,
}
impl_inherits!(StudioTheme, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioUserService {
    superclass: Instance,
}
impl_inherits!(StudioUserService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioWidget {
    superclass: StudioObjectBase,
}
impl_inherits!(StudioWidget, StudioObjectBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioWidgetsService {
    superclass: Instance,
}
impl_inherits!(StudioWidgetsService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StyleBase {
    superclass: Instance,
}
impl_inherits!(StyleBase, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StyleDerive {
    superclass: Instance,
    pub Index: i32,
    pub StyleSheet: Ref,
}
impl_inherits!(StyleDerive, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StyleLink {
    superclass: Instance,
    pub StyleSheet: Ref,
}
impl_inherits!(StyleLink, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StyleRule {
    superclass: StyleBase,
    pub Priority: i32,
    pub PropertiesSerialize: BinaryString,
    pub Selector: String,
}
impl_inherits!(StyleRule, StyleBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StyleSheet {
    superclass: StyleBase,
}
impl_inherits!(StyleSheet, StyleBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StylingService {
    superclass: Instance,
}
impl_inherits!(StylingService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SunRaysEffect {
    superclass: PostEffect,
    pub Intensity: f32,
    pub Spread: f32,
}
impl_inherits!(SunRaysEffect, PostEffect);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SurfaceAppearance {
    superclass: Instance,
    pub AlphaMode: enums::AlphaMode,
    pub Color: Color3,
    pub ColorMapContent: Content,
    pub MetalnessMapContent: Content,
    pub NormalMapContent: Content,
    pub RoughnessMapContent: Content,
    pub TexturePack: ContentId,
}
impl_inherits!(SurfaceAppearance, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SurfaceGui {
    superclass: SurfaceGuiBase,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SurfaceGuiBase {
    superclass: LayerCollector,
    pub Active: bool,
    pub Adornee: Ref,
    pub Face: enums::NormalId,
}
impl_inherits!(SurfaceGuiBase, LayerCollector);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SurfaceLight {
    superclass: Light,
    pub Angle: f32,
    pub Face: enums::NormalId,
    pub Range: f32,
}
impl_inherits!(SurfaceLight, Light);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SurfaceSelection {
    superclass: PartAdornment,
    pub TargetSurface: enums::NormalId,
}
impl_inherits!(SurfaceSelection, PartAdornment);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SwimController {
    superclass: ControllerBase,
    pub AccelerationTime: f32,
    pub PitchMaxTorque: f32,
    pub PitchSpeedFactor: f32,
    pub RollMaxTorque: f32,
    pub RollSpeedFactor: f32,
}
impl_inherits!(SwimController, ControllerBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SyncScriptBuilder {
    superclass: ScriptBuilder,
    pub CompileTarget: enums::CompileTarget,
    pub CoverageInfo: bool,
    pub DebugInfo: bool,
    pub PackAsSource: bool,
    pub RawBytecode: bool,
}
impl_inherits!(SyncScriptBuilder, ScriptBuilder);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SystemThemeService {
    superclass: Instance,
}
impl_inherits!(SystemThemeService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TaskScheduler {
    superclass: Instance,
    pub ThreadPoolConfig: enums::ThreadPoolConfig,
}
impl_inherits!(TaskScheduler, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Team {
    superclass: Instance,
    pub AutoAssignable: bool,
    pub TeamColor: BrickColor,
}
impl_inherits!(Team, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TeamCreateData {
    superclass: Instance,
}
impl_inherits!(TeamCreateData, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TeamCreatePublishService {
    superclass: Instance,
}
impl_inherits!(TeamCreatePublishService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TeamCreateService {
    superclass: Instance,
}
impl_inherits!(TeamCreateService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Teams {
    superclass: Instance,
}
impl_inherits!(Teams, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TelemetryService {
    superclass: Instance,
}
impl_inherits!(TelemetryService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TeleportAsyncResult {
    superclass: Instance,
}
impl_inherits!(TeleportAsyncResult, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TeleportOptions {
    superclass: Instance,
    pub ReservedServerAccessCode: String,
    pub ServerInstanceId: String,
    pub ShouldReserveServer: bool,
}
impl_inherits!(TeleportOptions, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TeleportService {
    superclass: Instance,
}
impl_inherits!(TeleportService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TemporaryCageMeshProvider {
    superclass: Instance,
}
impl_inherits!(TemporaryCageMeshProvider, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TemporaryScriptService {
    superclass: Instance,
}
impl_inherits!(TemporaryScriptService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Terrain {
    superclass: BasePart,
    pub AcquisitionMethod: enums::TerrainAcquisitionMethod,
    pub Decoration: bool,
    pub GrassLength: f32,
    pub MaterialColors: MaterialColors,
    pub PhysicsGrid: BinaryString,
    pub ShorelinesUpgraded: bool,
    pub SmoothGrid: BinaryString,
    pub SmoothVoxelsUpgraded: bool,
    pub WaterColor: Color3,
    pub WaterReflectance: f32,
    pub WaterTransparency: f32,
    pub WaterWaveSize: f32,
    pub WaterWaveSpeed: f32,
}
impl_inherits!(Terrain, BasePart);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TerrainDetail {
    superclass: Instance,
    pub ColorMap: ContentId,
    pub Face: enums::TerrainFace,
    pub MaterialPattern: enums::MaterialPattern,
    pub MetalnessMap: ContentId,
    pub NormalMap: ContentId,
    pub RoughnessMap: ContentId,
    pub StudsPerTile: f32,
    pub TexturePack: ContentId,
}
impl_inherits!(TerrainDetail, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TerrainRegion {
    superclass: Instance,
    pub ExtentsMax: Vector3int16,
    pub ExtentsMin: Vector3int16,
    pub SmoothGrid: BinaryString,
}
impl_inherits!(TerrainRegion, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TestService {
    superclass: Instance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextBox {
    superclass: GuiObject,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextBoxService {
    superclass: Instance,
}
impl_inherits!(TextBoxService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextButton {
    superclass: GuiButton,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextChannel {
    superclass: Instance,
}
impl_inherits!(TextChannel, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextChatCommand {
    superclass: Instance,
    pub AutocompleteVisible: bool,
    pub Enabled: bool,
    pub PrimaryAlias: String,
    pub SecondaryAlias: String,
}
impl_inherits!(TextChatCommand, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextChatConfigurations {
    superclass: Instance,
}
impl_inherits!(TextChatConfigurations, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextChatMessage {
    superclass: Instance,
    pub BubbleChatMessageProperties: Ref,
    pub ChatWindowMessageProperties: Ref,
    pub TextChannel: Ref,
    pub TextSource: Ref,
}
impl_inherits!(TextChatMessage, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextChatMessageProperties {
    superclass: Instance,
}
impl_inherits!(TextChatMessageProperties, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextChatService {
    superclass: Instance,
    pub ChatTranslationFtuxShown: bool,
    pub ChatTranslationToggleEnabled: bool,
    pub ChatVersion: enums::ChatVersion,
    pub CreateDefaultCommands: bool,
    pub CreateDefaultTextChannels: bool,
    pub HasSeenDeprecationDialog: bool,
}
impl_inherits!(TextChatService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextFilterResult {
    superclass: Instance,
}
impl_inherits!(TextFilterResult, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextFilterTranslatedResult {
    superclass: Instance,
}
impl_inherits!(TextFilterTranslatedResult, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextLabel {
    superclass: GuiLabel,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextService {
    superclass: Instance,
}
impl_inherits!(TextService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextSource {
    superclass: Instance,
    pub CanSend: bool,
}
impl_inherits!(TextSource, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Texture {
    superclass: Decal,
    pub OffsetStudsU: f32,
    pub OffsetStudsV: f32,
    pub StudsPerTileU: f32,
    pub StudsPerTileV: f32,
}
impl_inherits!(Texture, Decal);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextureGenerationPartGroup {
    superclass: Instance,
}
impl_inherits!(TextureGenerationPartGroup, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextureGenerationService {
    superclass: Instance,
}
impl_inherits!(TextureGenerationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextureGenerationUnwrappingRequest {
    superclass: Instance,
}
impl_inherits!(TextureGenerationUnwrappingRequest, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ThirdPartyUserService {
    superclass: Instance,
}
impl_inherits!(ThirdPartyUserService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ThreadState {
    superclass: Instance,
}
impl_inherits!(ThreadState, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TimerService {
    superclass: Instance,
}
impl_inherits!(TimerService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ToastNotificationService {
    superclass: Instance,
}
impl_inherits!(ToastNotificationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Tool {
    superclass: BackpackItem,
    pub CanBeDropped: bool,
    pub Enabled: bool,
    pub Grip: CFrame,
    pub ManualActivationOnly: bool,
    pub RequiresHandle: bool,
    pub ToolTip: String,
}
impl_inherits!(Tool, BackpackItem);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Torque {
    superclass: Constraint,
    pub RelativeTo: enums::ActuatorRelativeTo,
    pub Torque: Vector3,
}
impl_inherits!(Torque, Constraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TorsionSpringConstraint {
    superclass: Constraint,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TotalCountTimeIntervalItem {
    superclass: StatsItem,
}
impl_inherits!(TotalCountTimeIntervalItem, StatsItem);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TouchInputService {
    superclass: Instance,
}
impl_inherits!(TouchInputService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TouchTransmitter {
    superclass: Instance,
}
impl_inherits!(TouchTransmitter, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TracerService {
    superclass: Instance,
}
impl_inherits!(TracerService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TrackerLodController {
    superclass: Instance,
    pub AudioMode: enums::TrackerLodFlagMode,
    pub VideoExtrapolationMode: enums::TrackerExtrapolationFlagMode,
    pub VideoLodMode: enums::TrackerLodValueMode,
    pub VideoMode: enums::TrackerLodFlagMode,
}
impl_inherits!(TrackerLodController, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TrackerStreamAnimation {
    superclass: Instance,
}
impl_inherits!(TrackerStreamAnimation, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Trail {
    superclass: Instance,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Translator {
    superclass: Instance,
}
impl_inherits!(Translator, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TremoloSoundEffect {
    superclass: SoundEffect,
    pub Depth: f32,
    pub Duty: f32,
    pub Frequency: f32,
}
impl_inherits!(TremoloSoundEffect, SoundEffect);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TriangleMeshPart {
    superclass: BasePart,
    pub AeroMeshData: SharedString,
    pub FluidFidelityInternal: enums::FluidFidelity,
    pub PhysicalConfigData: SharedString,
    pub UnscaledCofm: Vector3,
    pub UnscaledVolInertiaDiags: Vector3,
    pub UnscaledVolInertiaOffDiags: Vector3,
    pub UnscaledVolume: f32,
}
impl_inherits!(TriangleMeshPart, BasePart);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TrussPart {
    superclass: BasePart,
    pub Style: enums::Style,
}
impl_inherits!(TrussPart, BasePart);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TutorialService {
    superclass: Instance,
}
impl_inherits!(TutorialService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Tween {
    superclass: TweenBase,
}
impl_inherits!(Tween, TweenBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TweenBase {
    superclass: Instance,
}
impl_inherits!(TweenBase, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TweenService {
    superclass: Instance,
}
impl_inherits!(TweenService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UGCAvatarService {
    superclass: Instance,
}
impl_inherits!(UGCAvatarService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UGCValidationService {
    superclass: Instance,
}
impl_inherits!(UGCValidationService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIAspectRatioConstraint {
    superclass: UIConstraint,
    pub AspectRatio: f32,
    pub AspectType: enums::AspectType,
    pub DominantAxis: enums::DominantAxis,
}
impl_inherits!(UIAspectRatioConstraint, UIConstraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIBase {
    superclass: Instance,
}
impl_inherits!(UIBase, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIComponent {
    superclass: UIBase,
}
impl_inherits!(UIComponent, UIBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIConstraint {
    superclass: UIComponent,
}
impl_inherits!(UIConstraint, UIComponent);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UICorner {
    superclass: UIComponent,
    pub CornerRadius: UDim,
}
impl_inherits!(UICorner, UIComponent);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIDragDetector {
    superclass: UIComponent,
    pub ActivatedCursorIcon: ContentId,
    pub BoundingBehavior: enums::UIDragDetectorBoundingBehavior,
    pub BoundingUi: Ref,
    pub CursorIcon: ContentId,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIDragDetectorService {
    superclass: Instance,
}
impl_inherits!(UIDragDetectorService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIFlexItem {
    superclass: UIComponent,
    pub FlexMode: enums::UIFlexMode,
    pub GrowRatio: f32,
    pub ItemLineAlignment: enums::ItemLineAlignment,
    pub ShrinkRatio: f32,
}
impl_inherits!(UIFlexItem, UIComponent);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIGradient {
    superclass: UIComponent,
    pub Color: ColorSequence,
    pub Enabled: bool,
    pub Offset: Vector2,
    pub Rotation: f32,
    pub Transparency: NumberSequence,
}
impl_inherits!(UIGradient, UIComponent);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIGridLayout {
    superclass: UIGridStyleLayout,
    pub CellPadding: UDim2,
    pub CellSize: UDim2,
    pub FillDirectionMaxCells: i32,
    pub StartCorner: enums::StartCorner,
}
impl_inherits!(UIGridLayout, UIGridStyleLayout);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIGridStyleLayout {
    superclass: UILayout,
    pub FillDirection: enums::FillDirection,
    pub HorizontalAlignment: enums::HorizontalAlignment,
    pub SortOrder: enums::SortOrder,
    pub VerticalAlignment: enums::VerticalAlignment,
}
impl_inherits!(UIGridStyleLayout, UILayout);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UILayout {
    superclass: UIComponent,
}
impl_inherits!(UILayout, UIComponent);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIListLayout {
    superclass: UIGridStyleLayout,
    pub HorizontalFlex: enums::UIFlexAlignment,
    pub ItemLineAlignment: enums::ItemLineAlignment,
    pub Padding: UDim,
    pub VerticalFlex: enums::UIFlexAlignment,
    pub Wraps: bool,
}
impl_inherits!(UIListLayout, UIGridStyleLayout);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIPadding {
    superclass: UIComponent,
    pub PaddingBottom: UDim,
    pub PaddingLeft: UDim,
    pub PaddingRight: UDim,
    pub PaddingTop: UDim,
}
impl_inherits!(UIPadding, UIComponent);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIPageLayout {
    superclass: UIGridStyleLayout,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIScale {
    superclass: UIComponent,
    pub Scale: f32,
}
impl_inherits!(UIScale, UIComponent);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UISizeConstraint {
    superclass: UIConstraint,
    pub MaxSize: Vector2,
    pub MinSize: Vector2,
}
impl_inherits!(UISizeConstraint, UIConstraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIStroke {
    superclass: UIComponent,
    pub ApplyStrokeMode: enums::ApplyStrokeMode,
    pub Color: Color3,
    pub Enabled: bool,
    pub LineJoinMode: enums::LineJoinMode,
    pub Thickness: f32,
    pub Transparency: f32,
}
impl_inherits!(UIStroke, UIComponent);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UITableLayout {
    superclass: UIGridStyleLayout,
    pub FillEmptySpaceColumns: bool,
    pub FillEmptySpaceRows: bool,
    pub MajorAxis: enums::TableMajorAxis,
    pub Padding: UDim2,
}
impl_inherits!(UITableLayout, UIGridStyleLayout);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UITextSizeConstraint {
    superclass: UIConstraint,
    pub MaxTextSize: i32,
    pub MinTextSize: i32,
}
impl_inherits!(UITextSizeConstraint, UIConstraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UnionOperation {
    superclass: PartOperation,
}
impl_inherits!(UnionOperation, PartOperation);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UniqueIdLookupService {
    superclass: Instance,
}
impl_inherits!(UniqueIdLookupService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UniversalConstraint {
    superclass: Constraint,
    pub LimitsEnabled: bool,
    pub MaxAngle: f32,
    pub Radius: f32,
    pub Restitution: f32,
}
impl_inherits!(UniversalConstraint, Constraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UnreliableRemoteEvent {
    superclass: BaseRemoteEvent,
}
impl_inherits!(UnreliableRemoteEvent, BaseRemoteEvent);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UnvalidatedAssetService {
    superclass: Instance,
    pub CachedData: String,
}
impl_inherits!(UnvalidatedAssetService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UserGameSettings {
    superclass: Instance,
    pub AllTutorialsDisabled: bool,
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
    pub PerformanceStatsVisible: bool,
    pub PlayerHeight: f32,
    pub PreferredTextSize: enums::PreferredTextSize,
    pub PreferredTransparency: f32,
    pub QualityResetLevel: i32,
    pub RccProfilerRecordFrameRate: i32,
    pub RccProfilerRecordTimeFrame: i32,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UserInputService {
    superclass: Instance,
    pub LegacyInputEventsEnabled: bool,
    pub MouseBehavior: enums::MouseBehavior,
    pub MouseIcon: ContentId,
    pub MouseIconEnabled: bool,
}
impl_inherits!(UserInputService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UserService {
    superclass: Instance,
}
impl_inherits!(UserService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UserSettings {
    superclass: GenericSettings,
}
impl_inherits!(UserSettings, GenericSettings);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UserStorageService {
    superclass: LocalStorageService,
}
impl_inherits!(UserStorageService, LocalStorageService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VRService {
    superclass: Instance,
    pub AutomaticScaling: enums::VRScaling,
    pub AvatarGestures: bool,
    pub ControllerModels: enums::VRControllerModelMode,
    pub FadeOutViewOnCollision: bool,
    pub LaserPointer: enums::VRLaserPointerMode,
}
impl_inherits!(VRService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VRStatusService {
    superclass: Instance,
}
impl_inherits!(VRStatusService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ValueBase {
    superclass: Instance,
}
impl_inherits!(ValueBase, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Vector3Curve {
    superclass: Instance,
}
impl_inherits!(Vector3Curve, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Vector3Value {
    superclass: ValueBase,
    pub Value: Vector3,
}
impl_inherits!(Vector3Value, ValueBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VectorForce {
    superclass: Constraint,
    pub ApplyAtCenterOfMass: bool,
    pub Force: Vector3,
    pub RelativeTo: enums::ActuatorRelativeTo,
}
impl_inherits!(VectorForce, Constraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VehicleController {
    superclass: Controller,
}
impl_inherits!(VehicleController, Controller);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VehicleSeat {
    superclass: BasePart,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VelocityMotor {
    superclass: JointInstance,
    pub CurrentAngle: f32,
    pub DesiredAngle: f32,
    pub Hole: Ref,
    pub MaxVelocity: f32,
}
impl_inherits!(VelocityMotor, JointInstance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VersionControlService {
    superclass: Instance,
}
impl_inherits!(VersionControlService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VideoCaptureService {
    superclass: Instance,
}
impl_inherits!(VideoCaptureService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VideoDeviceInput {
    superclass: Instance,
    pub Active: bool,
    pub CameraId: String,
    pub CaptureQuality: enums::VideoDeviceCaptureQuality,
}
impl_inherits!(VideoDeviceInput, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VideoDisplay {
    superclass: GuiObject,
    pub ResampleMode: enums::ResamplerMode,
    pub ScaleType: enums::ScaleType,
    pub TileSize: UDim2,
    pub VideoColor3: Color3,
    pub VideoRectOffset: Vector2,
    pub VideoRectSize: Vector2,
    pub VideoTransparency: f32,
}
impl_inherits!(VideoDisplay, GuiObject);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VideoFrame {
    superclass: GuiObject,
    pub Looped: bool,
    pub Playing: bool,
    pub TimePosition: f64,
    pub Video: ContentId,
    pub Volume: f32,
}
impl_inherits!(VideoFrame, GuiObject);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VideoPlayer {
    superclass: Instance,
    pub Asset: ContentId,
    pub Looping: bool,
    pub PlaybackSpeed: f32,
    pub TimePosition: f64,
    pub Volume: f32,
}
impl_inherits!(VideoPlayer, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VideoService {
    superclass: Instance,
}
impl_inherits!(VideoService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ViewportFrame {
    superclass: GuiObject,
    pub Ambient: Color3,
    pub CameraCFrame: CFrame,
    pub CameraFieldOfView: f32,
    pub ImageColor3: Color3,
    pub ImageTransparency: f32,
    pub LightColor: Color3,
    pub LightDirection: Vector3,
}
impl_inherits!(ViewportFrame, GuiObject);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VirtualInputManager {
    superclass: Instance,
}
impl_inherits!(VirtualInputManager, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VirtualUser {
    superclass: Instance,
}
impl_inherits!(VirtualUser, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VisibilityCheckDispatcher {
    superclass: Instance,
}
impl_inherits!(VisibilityCheckDispatcher, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Visit {
    superclass: Instance,
}
impl_inherits!(Visit, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VisualizationMode {
    superclass: Instance,
    pub Enabled: bool,
    pub Title: String,
    pub ToolTip: String,
}
impl_inherits!(VisualizationMode, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VisualizationModeCategory {
    superclass: Instance,
    pub Enabled: bool,
    pub Title: String,
}
impl_inherits!(VisualizationModeCategory, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VisualizationModeService {
    superclass: Instance,
}
impl_inherits!(VisualizationModeService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VoiceChatInternal {
    superclass: Instance,
}
impl_inherits!(VoiceChatInternal, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VoiceChatService {
    superclass: Instance,
    pub DefaultDistanceAttenuation: enums::VoiceChatDistanceAttenuationType,
    pub EnableDefaultVoice: bool,
    pub UseAudioApi: enums::AudioApiRollout,
}
impl_inherits!(VoiceChatService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WebSocketClient {
    superclass: Instance,
}
impl_inherits!(WebSocketClient, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WebSocketService {
    superclass: Instance,
}
impl_inherits!(WebSocketService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WebViewService {
    superclass: Instance,
}
impl_inherits!(WebViewService, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WedgePart {
    superclass: FormFactorPart,
}
impl_inherits!(WedgePart, FormFactorPart);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Weld {
    superclass: JointInstance,
}
impl_inherits!(Weld, JointInstance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WeldConstraint {
    superclass: Instance,
    pub CFrame0: CFrame,
    pub State: i32,
}
impl_inherits!(WeldConstraint, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Wire {
    superclass: Instance,
    pub SourceInstance: Ref,
    pub SourceName: String,
    pub TargetInstance: Ref,
    pub TargetName: String,
}
impl_inherits!(Wire, Instance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WireframeHandleAdornment {
    superclass: HandleAdornment,
    pub Scale: Vector3,
    pub Thickness: f32,
}
impl_inherits!(WireframeHandleAdornment, HandleAdornment);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Workspace {
    superclass: WorldRoot,
    pub AirDensity: f32,
    pub AllowThirdPartySales: bool,
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
}
impl_inherits!(Workspace, WorldRoot);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WorkspaceAnnotation {
    superclass: Annotation,
}
impl_inherits!(WorkspaceAnnotation, Annotation);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WorldModel {
    superclass: WorldRoot,
}
impl_inherits!(WorldModel, WorldRoot);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WorldRoot {
    superclass: Model,
}
impl_inherits!(WorldRoot, Model);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WrapDeformer {
    superclass: BaseWrap,
}
impl_inherits!(WrapDeformer, BaseWrap);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WrapLayer {
    superclass: BaseWrap,
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WrapTarget {
    superclass: BaseWrap,
    pub Stiffness: f32,
}
impl_inherits!(WrapTarget, BaseWrap);
