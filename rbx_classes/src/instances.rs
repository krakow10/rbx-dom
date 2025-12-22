use super::enums;
use crate::{impl_inherits, impl_strong_instance_from};
use core::ops::{Deref, DerefMut};
use rbx_types::*;
#[derive(Debug, Clone)]
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
    AnalyticsService(Box<AnalyticsService>),
    AngularVelocity(Box<AngularVelocity>),
    Animation(Box<Animation>),
    AnimationClip(Box<AnimationClip>),
    AnimationClipProvider(Box<AnimationClipProvider>),
    AnimationConstraint(Box<AnimationConstraint>),
    AnimationController(Box<AnimationController>),
    AnimationFromVideoCreatorService(Box<AnimationFromVideoCreatorService>),
    AnimationFromVideoCreatorStudioService(Box<AnimationFromVideoCreatorStudioService>),
    AnimationGraphDefinition(Box<AnimationGraphDefinition>),
    AnimationImportData(Box<AnimationImportData>),
    AnimationNode(Box<AnimationNode>),
    AnimationNodeDefinition(Box<AnimationNodeDefinition>),
    AnimationRigData(Box<AnimationRigData>),
    AnimationStreamTrack(Box<AnimationStreamTrack>),
    AnimationTrack(Box<AnimationTrack>),
    Animator(Box<Animator>),
    Annotation(Box<Annotation>),
    AnnotationsService(Box<AnnotationsService>),
    AppLifecycleObserverService(Box<AppLifecycleObserverService>),
    AppRatingPromptService(Box<AppRatingPromptService>),
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
    AudioGate(Box<AudioGate>),
    AudioLimiter(Box<AudioLimiter>),
    AudioListener(Box<AudioListener>),
    AudioPages(Box<AudioPages>),
    AudioPitchShifter(Box<AudioPitchShifter>),
    AudioPlayer(Box<AudioPlayer>),
    AudioRecorder(Box<AudioRecorder>),
    AudioReverb(Box<AudioReverb>),
    AudioSearchParams(Box<AudioSearchParams>),
    AudioSpeechToText(Box<AudioSpeechToText>),
    AudioTextToSpeech(Box<AudioTextToSpeech>),
    AudioTremolo(Box<AudioTremolo>),
    AuroraScript(Box<AuroraScript>),
    AuroraScriptObject(Box<AuroraScriptObject>),
    AuroraScriptService(Box<AuroraScriptService>),
    AuroraService(Box<AuroraService>),
    AvatarAccessoryRules(Box<AvatarAccessoryRules>),
    AvatarAnimationRules(Box<AvatarAnimationRules>),
    AvatarBodyRules(Box<AvatarBodyRules>),
    AvatarChatService(Box<AvatarChatService>),
    AvatarClothingRules(Box<AvatarClothingRules>),
    AvatarCollisionRules(Box<AvatarCollisionRules>),
    AvatarCreationService(Box<AvatarCreationService>),
    AvatarEditorService(Box<AvatarEditorService>),
    AvatarImportService(Box<AvatarImportService>),
    AvatarRules(Box<AvatarRules>),
    AvatarSettings(Box<AvatarSettings>),
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
    CapturesPages(Box<CapturesPages>),
    CatalogPages(Box<CatalogPages>),
    ChangeHistoryService(Box<ChangeHistoryService>),
    ChangeHistoryStreamingService(Box<ChangeHistoryStreamingService>),
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
    EncodingService(Box<EncodingService>),
    EqualizerSoundEffect(Box<EqualizerSoundEffect>),
    EulerRotationCurve(Box<EulerRotationCurve>),
    EventIngestService(Box<EventIngestService>),
    ExampleV2Service(Box<ExampleV2Service>),
    ExecutedRemoteCommand(Box<ExecutedRemoteCommand>),
    ExperienceAuthService(Box<ExperienceAuthService>),
    ExperienceInviteOptions(Box<ExperienceInviteOptions>),
    ExperienceNotificationService(Box<ExperienceNotificationService>),
    ExperienceService(Box<ExperienceService>),
    ExperienceStateCaptureService(Box<ExperienceStateCaptureService>),
    ExperienceStateRecordingService(Box<ExperienceStateRecordingService>),
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
    HandRigDescription(Box<HandRigDescription>),
    HandleAdornment(Box<HandleAdornment>),
    Handles(Box<Handles>),
    HandlesBase(Box<HandlesBase>),
    HapticEffect(Box<HapticEffect>),
    HapticService(Box<HapticService>),
    HarmonyService(Box<HarmonyService>),
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
    InstanceExtensionsService(Box<InstanceExtensionsService>),
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
    MLService(Box<MLService>),
    MLSession(Box<MLSession>),
    ManualGlue(Box<ManualGlue>),
    ManualSurfaceJointInstance(Box<ManualSurfaceJointInstance>),
    ManualWeld(Box<ManualWeld>),
    MarkerCurve(Box<MarkerCurve>),
    MarketplaceService(Box<MarketplaceService>),
    MatchmakingService(Box<MatchmakingService>),
    MaterialGenerationService(Box<MaterialGenerationService>),
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
    MicroProfilerService(Box<MicroProfilerService>),
    Model(Box<Model>),
    ModerationService(Box<ModerationService>),
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
    PartyEmulatorService(Box<PartyEmulatorService>),
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
    PyramidHandleAdornment(Box<PyramidHandleAdornment>),
    QWidgetPluginGui(Box<QWidgetPluginGui>),
    RTAnimationTracker(Box<RTAnimationTracker>),
    RayValue(Box<RayValue>),
    RbxAnalyticsService(Box<RbxAnalyticsService>),
    RecommendationPages(Box<RecommendationPages>),
    RecommendationService(Box<RecommendationService>),
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
    RemoteCommandService(Box<RemoteCommandService>),
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
    RuntimeContentService(Box<RuntimeContentService>),
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
    SessionCheckService(Box<SessionCheckService>),
    SessionService(Box<SessionService>),
    SharedTableRegistry(Box<SharedTableRegistry>),
    Shirt(Box<Shirt>),
    ShirtGraphic(Box<ShirtGraphic>),
    SkateboardController(Box<SkateboardController>),
    SkateboardPlatform(Box<SkateboardPlatform>),
    Skin(Box<Skin>),
    Sky(Box<Sky>),
    SlidingBallConstraint(Box<SlidingBallConstraint>),
    SlimContentProvider(Box<SlimContentProvider>),
    SlimService(Box<SlimService>),
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
    SoundShimService(Box<SoundShimService>),
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
    StudioTestService(Box<StudioTestService>),
    StudioTheme(Box<StudioTheme>),
    StudioUserService(Box<StudioUserService>),
    StudioWidget(Box<StudioWidget>),
    StudioWidgetsService(Box<StudioWidgetsService>),
    StyleBase(Box<StyleBase>),
    StyleDerive(Box<StyleDerive>),
    StyleLink(Box<StyleLink>),
    StyleQuery(Box<StyleQuery>),
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
    TerrainIterateOperation(Box<TerrainIterateOperation>),
    TerrainModifyOperation(Box<TerrainModifyOperation>),
    TerrainReadOperation(Box<TerrainReadOperation>),
    TerrainRegion(Box<TerrainRegion>),
    TerrainWriteOperation(Box<TerrainWriteOperation>),
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
    TextGenerator(Box<TextGenerator>),
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
    ValueCurve(Box<ValueCurve>),
    Vector3Curve(Box<Vector3Curve>),
    Vector3Value(Box<Vector3Value>),
    VectorForce(Box<VectorForce>),
    VehicleController(Box<VehicleController>),
    VehicleSeat(Box<VehicleSeat>),
    VelocityMotor(Box<VelocityMotor>),
    VersionControlService(Box<VersionControlService>),
    VideoCapture(Box<VideoCapture>),
    VideoCaptureService(Box<VideoCaptureService>),
    VideoDeviceInput(Box<VideoDeviceInput>),
    VideoDisplay(Box<VideoDisplay>),
    VideoFrame(Box<VideoFrame>),
    VideoPlayer(Box<VideoPlayer>),
    VideoSampler(Box<VideoSampler>),
    VideoScreenCaptureService(Box<VideoScreenCaptureService>),
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
    WebStreamClient(Box<WebStreamClient>),
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
    WrapTextureTransfer(Box<WrapTextureTransfer>),
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Accessory {
    superclass: Accoutrement,
    pub AccessoryType: enums::AccessoryType,
}
impl_inherits!(Accessory, Accoutrement);
impl_strong_instance_from!(Accessory);
impl Default for Accessory {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Accoutrement {
            superclass,
            AttachmentPoint: CFrame::identity(),
        };
        let superclass = Accessory {
            superclass,
            AccessoryType: enums::AccessoryType::Unknown,
        };
        superclass
    }
}
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
impl_strong_instance_from!(AccessoryDescription);
impl Default for AccessoryDescription {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AccessoryDescription {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AccountService {
    superclass: Instance,
}
impl_inherits!(AccountService, Instance);
impl_strong_instance_from!(AccountService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Accoutrement {
    superclass: Instance,
    pub AttachmentPoint: CFrame,
}
impl_inherits!(Accoutrement, Instance);
impl_strong_instance_from!(Accoutrement);
impl Default for Accoutrement {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Accoutrement {
            superclass,
            AttachmentPoint: CFrame::identity(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AchievementService {
    superclass: Instance,
}
impl_inherits!(AchievementService, Instance);
impl_strong_instance_from!(AchievementService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ActivityHistoryEventService {
    superclass: Instance,
}
impl_inherits!(ActivityHistoryEventService, Instance);
impl_strong_instance_from!(ActivityHistoryEventService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Actor {
    superclass: Model,
}
impl_inherits!(Actor, Model);
impl_strong_instance_from!(Actor);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AdGui {
    superclass: SurfaceGuiBase,
    pub AdShape: enums::AdShape,
    pub EnableVideoAds: bool,
    pub FallbackImage: ContentId,
}
impl_inherits!(AdGui, SurfaceGuiBase);
impl_strong_instance_from!(AdGui);
impl Default for AdGui {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = AdGui {
            superclass,
            AdShape: enums::AdShape::HorizontalRectangle,
            EnableVideoAds: false,
            FallbackImage: "".into(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AdPortal {
    superclass: Instance,
}
impl_inherits!(AdPortal, Instance);
impl_strong_instance_from!(AdPortal);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AdService {
    superclass: Instance,
}
impl_inherits!(AdService, Instance);
impl_strong_instance_from!(AdService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AdvancedDragger {
    superclass: Instance,
}
impl_inherits!(AdvancedDragger, Instance);
impl_strong_instance_from!(AdvancedDragger);
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
impl_strong_instance_from!(AirController);
impl Default for AirController {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ControllerBase {
            superclass,
            BalanceRigidityEnabled: false,
            MoveSpeedFactor: 0f32,
        };
        let superclass = AirController {
            superclass,
            BalanceMaxTorque: 10000f32,
            BalanceSpeed: 100f32,
            MaintainAngularMomentum: true,
            MaintainLinearMomentum: true,
            MoveMaxForce: 1000f32,
            TurnMaxTorque: 10000f32,
            TurnSpeedFactor: 1f32,
        };
        superclass
    }
}
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
impl_strong_instance_from!(AlignOrientation);
impl Default for AlignOrientation {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = AlignOrientation {
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
        };
        superclass
    }
}
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
impl_strong_instance_from!(AlignPosition);
impl Default for AlignPosition {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = AlignPosition {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnalyticsService {
    superclass: Instance,
    pub ApiKey: String,
}
impl_inherits!(AnalyticsService, Instance);
impl_strong_instance_from!(AnalyticsService);
impl Default for AnalyticsService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AnalyticsService {
            superclass,
            ApiKey: "".to_owned(),
        };
        superclass
    }
}
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
impl_strong_instance_from!(AngularVelocity);
impl Default for AngularVelocity {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = AngularVelocity {
            superclass,
            AngularVelocity: Vector3::new(0f32, 0f32, 0f32),
            MaxTorque: 0f32,
            ReactionTorqueEnabled: false,
            RelativeTo: enums::ActuatorRelativeTo::World,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Animation {
    superclass: Instance,
    pub AnimationId: ContentId,
}
impl_inherits!(Animation, Instance);
impl_strong_instance_from!(Animation);
impl Default for Animation {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Animation {
            superclass,
            AnimationId: "".into(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationClip {
    superclass: Instance,
    pub GuidBinaryString: BinaryString,
    pub Loop: bool,
    pub Priority: enums::AnimationPriority,
}
impl_inherits!(AnimationClip, Instance);
impl_strong_instance_from!(AnimationClip);
impl Default for AnimationClip {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AnimationClip {
            superclass,
            GuidBinaryString: b"".as_slice().into(),
            Loop: false,
            Priority: enums::AnimationPriority::Idle,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnimationClipProvider {
    superclass: Instance,
}
impl_inherits!(AnimationClipProvider, Instance);
impl_strong_instance_from!(AnimationClipProvider);
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
impl_strong_instance_from!(AnimationConstraint);
impl Default for AnimationConstraint {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = AnimationConstraint {
            superclass,
            IsKinematic: false,
            MaxForce: 10000f32,
            MaxTorque: 10000f32,
            Transform: CFrame::identity(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnimationController {
    superclass: Instance,
}
impl_inherits!(AnimationController, Instance);
impl_strong_instance_from!(AnimationController);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnimationFromVideoCreatorService {
    superclass: Instance,
}
impl_inherits!(AnimationFromVideoCreatorService, Instance);
impl_strong_instance_from!(AnimationFromVideoCreatorService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnimationFromVideoCreatorStudioService {
    superclass: Instance,
}
impl_inherits!(AnimationFromVideoCreatorStudioService, Instance);
impl_strong_instance_from!(AnimationFromVideoCreatorStudioService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnimationGraphDefinition {
    superclass: AnimationClip,
}
impl_inherits!(AnimationGraphDefinition, AnimationClip);
impl_strong_instance_from!(AnimationGraphDefinition);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnimationImportData {
    superclass: BaseImportData,
}
impl_inherits!(AnimationImportData, BaseImportData);
impl_strong_instance_from!(AnimationImportData);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnimationNode {
    superclass: Object,
}
impl_inherits!(AnimationNode, Object);
impl_strong_instance_from!(AnimationNode);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationNodeDefinition {
    superclass: Instance,
    pub InputPinData: BinaryString,
    pub NodeType: enums::AnimationNodeType,
}
impl_inherits!(AnimationNodeDefinition, Instance);
impl_strong_instance_from!(AnimationNodeDefinition);
impl Default for AnimationNodeDefinition {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AnimationNodeDefinition {
            superclass,
            InputPinData: b"\x01\0\0\0\0\0\0\0".as_slice().into(),
            NodeType: enums::AnimationNodeType::InvalidNode,
        };
        superclass
    }
}
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
impl_strong_instance_from!(AnimationRigData);
impl Default for AnimationRigData {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AnimationRigData { superclass , Label : b"\x01\0\0\0\x01\0\0\0\0\0\0\0" . as_slice () . into () , Name : b"\x01\0\0\0\x01\0\0\0\0\0\0\0" . as_slice () . into () , Parent : b"\x01\0\0\0\x01\0\0\0\0\0" . as_slice () . into () , PostTransform : b"\x01\0\0\0\x01\0\0\0\0\0\x80?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80?\0\0\0\0\0\0\0\0\0\0\0\0" . as_slice () . into () , PreTransform : b"\x01\0\0\0\x01\0\0\0\0\0\x80?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80?\0\0\0\0\0\0\0\0\0\0\0\0" . as_slice () . into () , Transform : b"\x01\0\0\0\x01\0\0\0\0\0\x80?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80?\0\0\0\0\0\0\0\0\0\0\0\0" . as_slice () . into () } ;
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnimationStreamTrack {
    superclass: Instance,
}
impl_inherits!(AnimationStreamTrack, Instance);
impl_strong_instance_from!(AnimationStreamTrack);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationTrack {
    superclass: Instance,
    pub Priority: enums::AnimationPriority,
}
impl_inherits!(AnimationTrack, Instance);
impl_strong_instance_from!(AnimationTrack);
impl Default for AnimationTrack {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AnimationTrack {
            superclass,
            Priority: enums::AnimationPriority::Idle,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Animator {
    superclass: Instance,
    pub PreferLodEnabled: bool,
}
impl_inherits!(Animator, Instance);
impl_strong_instance_from!(Animator);
impl Default for Animator {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Animator {
            superclass,
            PreferLodEnabled: true,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Annotation {
    superclass: Instance,
}
impl_inherits!(Annotation, Instance);
impl_strong_instance_from!(Annotation);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AnnotationsService {
    superclass: Instance,
}
impl_inherits!(AnnotationsService, Instance);
impl_strong_instance_from!(AnnotationsService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AppLifecycleObserverService {
    superclass: Instance,
}
impl_inherits!(AppLifecycleObserverService, Instance);
impl_strong_instance_from!(AppLifecycleObserverService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AppRatingPromptService {
    superclass: Instance,
}
impl_inherits!(AppRatingPromptService, Instance);
impl_strong_instance_from!(AppRatingPromptService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AppStorageService {
    superclass: LocalStorageService,
}
impl_inherits!(AppStorageService, LocalStorageService);
impl_strong_instance_from!(AppStorageService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AppUpdateService {
    superclass: Instance,
}
impl_inherits!(AppUpdateService, Instance);
impl_strong_instance_from!(AppUpdateService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ArcHandles {
    superclass: HandlesBase,
    pub Axes: Axes,
}
impl_inherits!(ArcHandles, HandlesBase);
impl_strong_instance_from!(ArcHandles);
impl Default for ArcHandles {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = PartAdornment {
            superclass,
            Adornee: Ref::none(),
        };
        let superclass = HandlesBase { superclass };
        let superclass = ArcHandles {
            superclass,
            Axes: unimplemented!(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AssetCounterService {
    superclass: Instance,
}
impl_inherits!(AssetCounterService, Instance);
impl_strong_instance_from!(AssetCounterService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetDeliveryProxy {
    superclass: Instance,
    pub Interface: String,
    pub Port: i32,
    pub StartServer: bool,
}
impl_inherits!(AssetDeliveryProxy, Instance);
impl_strong_instance_from!(AssetDeliveryProxy);
impl Default for AssetDeliveryProxy {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AssetDeliveryProxy {
            superclass,
            Interface: "".to_owned(),
            Port: 0i32,
            StartServer: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AssetImportService {
    superclass: Instance,
}
impl_inherits!(AssetImportService, Instance);
impl_strong_instance_from!(AssetImportService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AssetImportSession {
    superclass: ImportSession,
}
impl_inherits!(AssetImportSession, ImportSession);
impl_strong_instance_from!(AssetImportSession);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AssetManagerService {
    superclass: Instance,
}
impl_inherits!(AssetManagerService, Instance);
impl_strong_instance_from!(AssetManagerService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetPatchSettings {
    superclass: Instance,
    pub ContentId: String,
    pub OutputPath: String,
    pub PatchId: String,
}
impl_inherits!(AssetPatchSettings, Instance);
impl_strong_instance_from!(AssetPatchSettings);
impl Default for AssetPatchSettings {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AssetPatchSettings {
            superclass,
            ContentId: "".to_owned(),
            OutputPath: "".to_owned(),
            PatchId: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetService {
    superclass: Instance,
    pub AllowInsertFreeAssets: bool,
}
impl_inherits!(AssetService, Instance);
impl_strong_instance_from!(AssetService);
impl Default for AssetService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AssetService {
            superclass,
            AllowInsertFreeAssets: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AssetSoundEffect {
    superclass: CustomSoundEffect,
}
impl_inherits!(AssetSoundEffect, CustomSoundEffect);
impl_strong_instance_from!(AssetSoundEffect);
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
impl_strong_instance_from!(Atmosphere);
impl Default for Atmosphere {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Atmosphere {
            superclass,
            Color: Color3::new(0.7843f32, 0.6667f32, 0.4235f32),
            Decay: Color3::new(0.3608f32, 0.2353f32, 0.0549f32),
            Density: 0.395f32,
            Glare: 0f32,
            Haze: 0f32,
            Offset: 0f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AtmosphereSensor {
    superclass: SensorBase,
}
impl_inherits!(AtmosphereSensor, SensorBase);
impl_strong_instance_from!(AtmosphereSensor);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Attachment {
    superclass: Instance,
    pub CFrame: CFrame,
    pub Visible: bool,
}
impl_inherits!(Attachment, Instance);
impl_strong_instance_from!(Attachment);
impl Default for Attachment {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Attachment {
            superclass,
            CFrame: CFrame::identity(),
            Visible: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioAnalyzer {
    superclass: Instance,
    pub SpectrumEnabled: bool,
    pub WindowSize: enums::AudioWindowSize,
}
impl_inherits!(AudioAnalyzer, Instance);
impl_strong_instance_from!(AudioAnalyzer);
impl Default for AudioAnalyzer {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioAnalyzer {
            superclass,
            SpectrumEnabled: true,
            WindowSize: enums::AudioWindowSize::Medium,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioChannelMixer {
    superclass: Instance,
    pub Layout: enums::AudioChannelLayout,
}
impl_inherits!(AudioChannelMixer, Instance);
impl_strong_instance_from!(AudioChannelMixer);
impl Default for AudioChannelMixer {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioChannelMixer {
            superclass,
            Layout: enums::AudioChannelLayout::Stereo,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioChannelSplitter {
    superclass: Instance,
    pub Layout: enums::AudioChannelLayout,
}
impl_inherits!(AudioChannelSplitter, Instance);
impl_strong_instance_from!(AudioChannelSplitter);
impl Default for AudioChannelSplitter {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioChannelSplitter {
            superclass,
            Layout: enums::AudioChannelLayout::Stereo,
        };
        superclass
    }
}
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
impl_strong_instance_from!(AudioChorus);
impl Default for AudioChorus {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioChorus {
            superclass,
            Bypass: false,
            Depth: 0.45f32,
            Mix: 0.85f32,
            Rate: 5f32,
        };
        superclass
    }
}
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
impl_strong_instance_from!(AudioCompressor);
impl Default for AudioCompressor {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioCompressor {
            superclass,
            Attack: 0.1f32,
            Bypass: false,
            MakeupGain: 0f32,
            Ratio: 40f32,
            Release: 0.1f32,
            Threshold: -40f32,
        };
        superclass
    }
}
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
impl_strong_instance_from!(AudioDeviceInput);
impl Default for AudioDeviceInput {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioDeviceInput {
            superclass,
            AccessType: enums::AccessModifierType::Deny,
            Active: true,
            Muted: false,
            Player: Ref::none(),
            Volume: 1f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioDeviceOutput {
    superclass: Instance,
    pub Player: Ref,
}
impl_inherits!(AudioDeviceOutput, Instance);
impl_strong_instance_from!(AudioDeviceOutput);
impl Default for AudioDeviceOutput {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioDeviceOutput {
            superclass,
            Player: Ref::none(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioDistortion {
    superclass: Instance,
    pub Bypass: bool,
    pub Level: f32,
}
impl_inherits!(AudioDistortion, Instance);
impl_strong_instance_from!(AudioDistortion);
impl Default for AudioDistortion {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioDistortion {
            superclass,
            Bypass: false,
            Level: 0.5f32,
        };
        superclass
    }
}
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
impl_strong_instance_from!(AudioEcho);
impl Default for AudioEcho {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioEcho {
            superclass,
            Bypass: false,
            DelayTime: 1f32,
            DryLevel: 0f32,
            Feedback: 0.5f32,
            RampTime: 0f32,
            WetLevel: 0f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioEmitter {
    superclass: Instance,
    pub AcousticSimulationEnabled: bool,
    pub AngleAttenuation: BinaryString,
    pub AudioInteractionGroup: String,
    pub DistanceAttenuation: BinaryString,
    pub PositionOverride: Ref,
    pub SimulationFidelity: enums::AudioSimulationFidelity,
}
impl_inherits!(AudioEmitter, Instance);
impl_strong_instance_from!(AudioEmitter);
impl Default for AudioEmitter {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioEmitter {
            superclass,
            AcousticSimulationEnabled: true,
            AngleAttenuation: b"\0".as_slice().into(),
            AudioInteractionGroup: "".to_owned(),
            DistanceAttenuation: b"\0".as_slice().into(),
            PositionOverride: Ref::none(),
            SimulationFidelity: enums::AudioSimulationFidelity::Automatic,
        };
        superclass
    }
}
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
impl_strong_instance_from!(AudioEqualizer);
impl Default for AudioEqualizer {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioEqualizer {
            superclass,
            Bypass: false,
            HighGain: 0f32,
            LowGain: 0f32,
            MidGain: 0f32,
            MidRange: NumberRange::new(400f32, 4000f32),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioFader {
    superclass: Instance,
    pub Bypass: bool,
    pub Volume: f32,
}
impl_inherits!(AudioFader, Instance);
impl_strong_instance_from!(AudioFader);
impl Default for AudioFader {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioFader {
            superclass,
            Bypass: false,
            Volume: 1f32,
        };
        superclass
    }
}
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
impl_strong_instance_from!(AudioFilter);
impl Default for AudioFilter {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioFilter {
            superclass,
            Bypass: false,
            FilterType: enums::AudioFilterType::Peak,
            Frequency: 2000f32,
            Gain: 0f32,
            Q: 0.707f32,
        };
        superclass
    }
}
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
impl_strong_instance_from!(AudioFlanger);
impl Default for AudioFlanger {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioFlanger {
            superclass,
            Bypass: false,
            Depth: 0.45f32,
            Mix: 0.85f32,
            Rate: 5f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AudioFocusService {
    superclass: Instance,
}
impl_inherits!(AudioFocusService, Instance);
impl_strong_instance_from!(AudioFocusService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioGate {
    superclass: Instance,
    pub Attack: f32,
    pub Bypass: bool,
    pub Release: f32,
    pub Threshold: NumberRange,
}
impl_inherits!(AudioGate, Instance);
impl_strong_instance_from!(AudioGate);
impl Default for AudioGate {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioGate {
            superclass,
            Attack: 0.01f32,
            Bypass: false,
            Release: 0.1f32,
            Threshold: NumberRange::new(-36f32, -24f32),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioLimiter {
    superclass: Instance,
    pub Bypass: bool,
    pub MaxLevel: f32,
    pub Release: f32,
}
impl_inherits!(AudioLimiter, Instance);
impl_strong_instance_from!(AudioLimiter);
impl Default for AudioLimiter {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioLimiter {
            superclass,
            Bypass: false,
            MaxLevel: 0f32,
            Release: 0.01f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioListener {
    superclass: Instance,
    pub AcousticSimulationEnabled: bool,
    pub AngleAttenuation: BinaryString,
    pub AudioInteractionGroup: String,
    pub DistanceAttenuation: BinaryString,
    pub PositionOverride: Ref,
    pub SimulationFidelity: enums::AudioSimulationFidelity,
}
impl_inherits!(AudioListener, Instance);
impl_strong_instance_from!(AudioListener);
impl Default for AudioListener {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioListener {
            superclass,
            AcousticSimulationEnabled: true,
            AngleAttenuation: b"\0".as_slice().into(),
            AudioInteractionGroup: "".to_owned(),
            DistanceAttenuation: b"\0".as_slice().into(),
            PositionOverride: Ref::none(),
            SimulationFidelity: enums::AudioSimulationFidelity::Automatic,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AudioPages {
    superclass: Pages,
}
impl_inherits!(AudioPages, Pages);
impl_strong_instance_from!(AudioPages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioPitchShifter {
    superclass: Instance,
    pub Bypass: bool,
    pub Pitch: f32,
    pub WindowSize: enums::AudioWindowSize,
}
impl_inherits!(AudioPitchShifter, Instance);
impl_strong_instance_from!(AudioPitchShifter);
impl Default for AudioPitchShifter {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioPitchShifter {
            superclass,
            Bypass: false,
            Pitch: 1.25f32,
            WindowSize: enums::AudioWindowSize::Medium,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioPlayer {
    superclass: Instance,
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
impl_strong_instance_from!(AudioPlayer);
impl Default for AudioPlayer {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioPlayer {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioRecorder {
    superclass: Instance,
    pub IsRecording: bool,
}
impl_inherits!(AudioRecorder, Instance);
impl_strong_instance_from!(AudioRecorder);
impl Default for AudioRecorder {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioRecorder {
            superclass,
            IsRecording: false,
        };
        superclass
    }
}
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
impl_strong_instance_from!(AudioReverb);
impl Default for AudioReverb {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioReverb {
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
        };
        superclass
    }
}
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
impl_strong_instance_from!(AudioSearchParams);
impl Default for AudioSearchParams {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioSearchParams {
            superclass,
            Album: "".to_owned(),
            Artist: "".to_owned(),
            AudioSubType: enums::AudioSubType::Music,
            MaxDuration: 0i32,
            MinDuration: 0i32,
            SearchKeyword: "".to_owned(),
            Tag: "".to_owned(),
            Title: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioSpeechToText {
    superclass: Instance,
    pub Enabled: bool,
    pub Text: String,
}
impl_inherits!(AudioSpeechToText, Instance);
impl_strong_instance_from!(AudioSpeechToText);
impl Default for AudioSpeechToText {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioSpeechToText {
            superclass,
            Enabled: false,
            Text: "".to_owned(),
        };
        superclass
    }
}
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
impl_strong_instance_from!(AudioTextToSpeech);
impl Default for AudioTextToSpeech {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioTextToSpeech {
            superclass,
            Looping: false,
            Pitch: 0f32,
            PlaybackSpeed: 1f32,
            Speed: 1f32,
            Text: "".to_owned(),
            TimePosition: 0f64,
            VoiceId: "".to_owned(),
            Volume: 1f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioTremolo {
    superclass: Instance,
    pub Bypass: bool,
    pub Depth: f32,
    pub Duty: f32,
    pub Frequency: f32,
    pub Shape: f32,
    pub Skew: f32,
    pub Square: f32,
}
impl_inherits!(AudioTremolo, Instance);
impl_strong_instance_from!(AudioTremolo);
impl Default for AudioTremolo {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AudioTremolo {
            superclass,
            Bypass: false,
            Depth: 1f32,
            Duty: 0.5f32,
            Frequency: 5f32,
            Shape: 0f32,
            Skew: 0f32,
            Square: 0f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AuroraScript {
    superclass: LuaSourceContainer,
    pub AuroraScriptBindingsSerialize: BinaryString,
    pub EnableCulling: bool,
    pub EnableLod: bool,
    pub LodCriticality: i32,
    pub Priority: i32,
    pub Source: String,
}
impl_inherits!(AuroraScript, LuaSourceContainer);
impl_strong_instance_from!(AuroraScript);
impl Default for AuroraScript {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = AuroraScript {
            superclass,
            AuroraScriptBindingsSerialize: b"".as_slice().into(),
            EnableCulling: false,
            EnableLod: false,
            LodCriticality: 0i32,
            Priority: 0i32,
            Source: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AuroraScriptObject {
    superclass: Instance,
    pub BehaviorWeak: Ref,
    pub BoundInstanceWeak: Ref,
    pub FrameId: i32,
    pub LodLevel: i32,
    pub PriorFrameInvoked: i32,
}
impl_inherits!(AuroraScriptObject, Instance);
impl_strong_instance_from!(AuroraScriptObject);
impl Default for AuroraScriptObject {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AuroraScriptObject {
            superclass,
            BehaviorWeak: Ref::none(),
            BoundInstanceWeak: Ref::none(),
            FrameId: 0i32,
            LodLevel: 0i32,
            PriorFrameInvoked: 0i32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AuroraScriptService {
    superclass: Instance,
}
impl_inherits!(AuroraScriptService, Instance);
impl_strong_instance_from!(AuroraScriptService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AuroraService {
    superclass: Instance,
    pub HashRoundingPoint: f64,
    pub IgnoreRotation: bool,
    pub LockStepIdOffset: bool,
    pub RollbackOffset: i32,
}
impl_inherits!(AuroraService, Instance);
impl_strong_instance_from!(AuroraService);
impl Default for AuroraService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AuroraService {
            superclass,
            HashRoundingPoint: 0f64,
            IgnoreRotation: false,
            LockStepIdOffset: false,
            RollbackOffset: 0i32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarAccessoryRules {
    superclass: Instance,
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
impl_strong_instance_from!(AvatarAccessoryRules);
impl Default for AvatarAccessoryRules {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AvatarAccessoryRules {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarAnimationRules {
    superclass: Instance,
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
impl_strong_instance_from!(AvatarAnimationRules);
impl Default for AvatarAnimationRules {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AvatarAnimationRules {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarBodyRules {
    superclass: Instance,
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
impl_strong_instance_from!(AvatarBodyRules);
impl Default for AvatarBodyRules {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AvatarBodyRules {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AvatarChatService {
    superclass: Instance,
}
impl_inherits!(AvatarChatService, Instance);
impl_strong_instance_from!(AvatarChatService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarClothingRules {
    superclass: Instance,
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
impl_strong_instance_from!(AvatarClothingRules);
impl Default for AvatarClothingRules {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AvatarClothingRules {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarCollisionRules {
    superclass: Instance,
    pub CollisionMode: enums::AvatarSettingsCollisionMode,
    pub HitAndTouchDetectionMode: enums::AvatarSettingsHitAndTouchDetectionMode,
    pub LegacyCollisionMode: enums::AvatarSettingsLegacyCollisionMode,
    pub SingleColliderSize: Vector3,
}
impl_inherits!(AvatarCollisionRules, Instance);
impl_strong_instance_from!(AvatarCollisionRules);
impl Default for AvatarCollisionRules {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AvatarCollisionRules {
            superclass,
            CollisionMode: enums::AvatarSettingsCollisionMode::Default,
            HitAndTouchDetectionMode: enums::AvatarSettingsHitAndTouchDetectionMode::UseParts,
            LegacyCollisionMode: enums::AvatarSettingsLegacyCollisionMode::InnerBoxColliders,
            SingleColliderSize: Vector3::new(2f32, 3f32, 1f32),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AvatarCreationService {
    superclass: Instance,
}
impl_inherits!(AvatarCreationService, Instance);
impl_strong_instance_from!(AvatarCreationService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AvatarEditorService {
    superclass: Instance,
}
impl_inherits!(AvatarEditorService, Instance);
impl_strong_instance_from!(AvatarEditorService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AvatarImportService {
    superclass: Instance,
}
impl_inherits!(AvatarImportService, Instance);
impl_strong_instance_from!(AvatarImportService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarRules {
    superclass: Instance,
    pub AvatarType: enums::GameAvatarType,
}
impl_inherits!(AvatarRules, Instance);
impl_strong_instance_from!(AvatarRules);
impl Default for AvatarRules {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AvatarRules {
            superclass,
            AvatarType: enums::GameAvatarType::R15,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct AvatarSettings {
    superclass: Instance,
}
impl_inherits!(AvatarSettings, Instance);
impl_strong_instance_from!(AvatarSettings);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Backpack {
    superclass: Instance,
}
impl_inherits!(Backpack, Instance);
impl_strong_instance_from!(Backpack);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BackpackItem {
    superclass: Model,
    pub TextureContent: Content,
}
impl_inherits!(BackpackItem, Model);
impl_strong_instance_from!(BackpackItem);
impl Default for BackpackItem {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = BackpackItem {
            superclass,
            TextureContent: Content::none(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BadgeService {
    superclass: Instance,
}
impl_inherits!(BadgeService, Instance);
impl_strong_instance_from!(BadgeService);
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
impl_strong_instance_from!(BallSocketConstraint);
impl Default for BallSocketConstraint {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = BallSocketConstraint {
            superclass,
            LimitsEnabled: false,
            MaxFrictionTorqueXml: 0f32,
            Radius: 0.15f32,
            Restitution: 0f32,
            TwistLimitsEnabled: false,
            TwistLowerAngle: -45f32,
            TwistUpperAngle: 45f32,
            UpperAngle: 45f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BanHistoryPages {
    superclass: Pages,
}
impl_inherits!(BanHistoryPages, Pages);
impl_strong_instance_from!(BanHistoryPages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BaseImportData {
    superclass: Instance,
    pub ImportName: String,
    pub ShouldImport: bool,
}
impl_inherits!(BaseImportData, Instance);
impl_strong_instance_from!(BaseImportData);
impl Default for BaseImportData {
    fn default() -> Self {
        let superclass = Object {};
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
        superclass
    }
}
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
impl_strong_instance_from!(BasePart);
impl Default for BasePart {
    fn default() -> Self {
        let superclass = Object {};
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
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BasePlayerGui {
    superclass: Instance,
}
impl_inherits!(BasePlayerGui, Instance);
impl_strong_instance_from!(BasePlayerGui);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BaseRemoteEvent {
    superclass: Instance,
}
impl_inherits!(BaseRemoteEvent, Instance);
impl_strong_instance_from!(BaseRemoteEvent);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BaseScript {
    superclass: LuaSourceContainer,
    pub Disabled: bool,
    pub LinkedSource: ContentId,
    pub RunContext: enums::RunContext,
}
impl_inherits!(BaseScript, LuaSourceContainer);
impl_strong_instance_from!(BaseScript);
impl Default for BaseScript {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = BaseScript {
            superclass,
            Disabled: false,
            LinkedSource: "".into(),
            RunContext: enums::RunContext::Legacy,
        };
        superclass
    }
}
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
impl_strong_instance_from!(BaseWrap);
impl Default for BaseWrap {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
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
        superclass
    }
}
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
impl_strong_instance_from!(Beam);
impl Default for Beam {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Beam {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BevelMesh {
    superclass: DataModelMesh,
    pub Bevel: f32,
    pub BevelRoundness: f32,
    pub Bulge: f32,
}
impl_inherits!(BevelMesh, DataModelMesh);
impl_strong_instance_from!(BevelMesh);
impl Default for BevelMesh {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = BevelMesh {
            superclass,
            Bevel: 0f32,
            BevelRoundness: 0f32,
            Bulge: 0f32,
        };
        superclass
    }
}
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
impl_strong_instance_from!(BillboardGui);
impl Default for BillboardGui {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = BillboardGui {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BinaryStringValue {
    superclass: ValueBase,
    pub Value: BinaryString,
}
impl_inherits!(BinaryStringValue, ValueBase);
impl_strong_instance_from!(BinaryStringValue);
impl Default for BinaryStringValue {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        let superclass = BinaryStringValue {
            superclass,
            Value: b"".as_slice().into(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BindableEvent {
    superclass: Instance,
}
impl_inherits!(BindableEvent, Instance);
impl_strong_instance_from!(BindableEvent);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BindableFunction {
    superclass: Instance,
}
impl_inherits!(BindableFunction, Instance);
impl_strong_instance_from!(BindableFunction);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BlockMesh {
    superclass: BevelMesh,
}
impl_inherits!(BlockMesh, BevelMesh);
impl_strong_instance_from!(BlockMesh);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BloomEffect {
    superclass: PostEffect,
    pub Intensity: f32,
    pub Size: f32,
    pub Threshold: f32,
}
impl_inherits!(BloomEffect, PostEffect);
impl_strong_instance_from!(BloomEffect);
impl Default for BloomEffect {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PostEffect {
            superclass,
            Enabled: false,
        };
        let superclass = BloomEffect {
            superclass,
            Intensity: 0.4f32,
            Size: 24f32,
            Threshold: 0.95f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BlurEffect {
    superclass: PostEffect,
    pub Size: f32,
}
impl_inherits!(BlurEffect, PostEffect);
impl_strong_instance_from!(BlurEffect);
impl Default for BlurEffect {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PostEffect {
            superclass,
            Enabled: false,
        };
        let superclass = BlurEffect {
            superclass,
            Size: 24f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyAngularVelocity {
    superclass: BodyMover,
    pub AngularVelocity: Vector3,
    pub MaxTorque: Vector3,
    pub P: f32,
}
impl_inherits!(BodyAngularVelocity, BodyMover);
impl_strong_instance_from!(BodyAngularVelocity);
impl Default for BodyAngularVelocity {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BodyMover { superclass };
        let superclass = BodyAngularVelocity {
            superclass,
            AngularVelocity: Vector3::new(0f32, 2f32, 0f32),
            MaxTorque: Vector3::new(4000f32, 4000f32, 4000f32),
            P: 1250f32,
        };
        superclass
    }
}
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
impl_strong_instance_from!(BodyColors);
impl Default for BodyColors {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = BodyColors {
            superclass,
            HeadColor3: Color3::new(0.9921569f32, 0.9176471f32, 0.5529412f32),
            LeftArmColor3: Color3::new(0.9921569f32, 0.9176471f32, 0.5529412f32),
            LeftLegColor3: Color3::new(0.050980397f32, 0.41176474f32, 0.6745098f32),
            RightArmColor3: Color3::new(0.9921569f32, 0.9176471f32, 0.5529412f32),
            RightLegColor3: Color3::new(0.050980397f32, 0.41176474f32, 0.6745098f32),
            TorsoColor3: Color3::new(0.15686275f32, 0.49803925f32, 0.2784314f32),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyForce {
    superclass: BodyMover,
    pub Force: Vector3,
}
impl_inherits!(BodyForce, BodyMover);
impl_strong_instance_from!(BodyForce);
impl Default for BodyForce {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BodyMover { superclass };
        let superclass = BodyForce {
            superclass,
            Force: Vector3::new(0f32, 1f32, 0f32),
        };
        superclass
    }
}
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
impl_strong_instance_from!(BodyGyro);
impl Default for BodyGyro {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BodyMover { superclass };
        let superclass = BodyGyro {
            superclass,
            CFrame: CFrame::identity(),
            D: 500f32,
            MaxTorque: Vector3::new(400000f32, 0f32, 400000f32),
            P: 3000f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BodyMover {
    superclass: Instance,
}
impl_inherits!(BodyMover, Instance);
impl_strong_instance_from!(BodyMover);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyPartDescription {
    superclass: Instance,
    pub AssetId: i64,
    pub BodyPart: enums::BodyPart,
    pub Color: Color3,
    pub HeadShape: String,
    pub Instance: Ref,
}
impl_inherits!(BodyPartDescription, Instance);
impl_strong_instance_from!(BodyPartDescription);
impl Default for BodyPartDescription {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BodyPartDescription {
            superclass,
            AssetId: 0i64,
            BodyPart: enums::BodyPart::Head,
            Color: Color3::new(0f32, 0f32, 0f32),
            HeadShape: "".to_owned(),
            Instance: Ref::none(),
        };
        superclass
    }
}
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
impl_strong_instance_from!(BodyPosition);
impl Default for BodyPosition {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BodyMover { superclass };
        let superclass = BodyPosition {
            superclass,
            D: 1250f32,
            MaxForce: Vector3::new(4000f32, 4000f32, 4000f32),
            P: 10000f32,
            Position: Vector3::new(0f32, 50f32, 0f32),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyThrust {
    superclass: BodyMover,
    pub Force: Vector3,
    pub Location: Vector3,
}
impl_inherits!(BodyThrust, BodyMover);
impl_strong_instance_from!(BodyThrust);
impl Default for BodyThrust {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BodyMover { superclass };
        let superclass = BodyThrust {
            superclass,
            Force: Vector3::new(0f32, 1f32, 0f32),
            Location: Vector3::new(0f32, 0f32, 0f32),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyVelocity {
    superclass: BodyMover,
    pub MaxForce: Vector3,
    pub P: f32,
    pub Velocity: Vector3,
}
impl_inherits!(BodyVelocity, BodyMover);
impl_strong_instance_from!(BodyVelocity);
impl Default for BodyVelocity {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BodyMover { superclass };
        let superclass = BodyVelocity {
            superclass,
            MaxForce: Vector3::new(4000f32, 4000f32, 4000f32),
            P: 1250f32,
            Velocity: Vector3::new(0f32, 2f32, 0f32),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Bone {
    superclass: Attachment,
}
impl_inherits!(Bone, Attachment);
impl_strong_instance_from!(Bone);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BoolValue {
    superclass: ValueBase,
    pub Value: bool,
}
impl_inherits!(BoolValue, ValueBase);
impl_strong_instance_from!(BoolValue);
impl Default for BoolValue {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        let superclass = BoolValue {
            superclass,
            Value: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BoxHandleAdornment {
    superclass: HandleAdornment,
    pub Shading: enums::AdornShading,
    pub Size: Vector3,
}
impl_inherits!(BoxHandleAdornment, HandleAdornment);
impl_strong_instance_from!(BoxHandleAdornment);
impl Default for BoxHandleAdornment {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = HandleAdornment {
            superclass,
            AdornCullingMode: enums::AdornCullingMode::Automatic,
            AlwaysOnTop: false,
            CFrame: CFrame::identity(),
            SizeRelativeOffset: Vector3::new(0f32, 0f32, 0f32),
            ZIndex: 0i32,
        };
        let superclass = BoxHandleAdornment {
            superclass,
            Shading: enums::AdornShading::Default,
            Size: Vector3::new(1f32, 1f32, 1f32),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Breakpoint {
    superclass: Instance,
}
impl_inherits!(Breakpoint, Instance);
impl_strong_instance_from!(Breakpoint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BrickColorValue {
    superclass: ValueBase,
    pub Value: BrickColor,
}
impl_inherits!(BrickColorValue, ValueBase);
impl_strong_instance_from!(BrickColorValue);
impl Default for BrickColorValue {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        let superclass = BrickColorValue {
            superclass,
            Value: BrickColor::from_number(194u16).unwrap(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BrowserService {
    superclass: Instance,
}
impl_inherits!(BrowserService, Instance);
impl_strong_instance_from!(BrowserService);
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
impl_strong_instance_from!(BubbleChatConfiguration);
impl Default for BubbleChatConfiguration {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = TextChatConfigurations { superclass };
        let superclass = BubbleChatConfiguration {
            superclass,
            AdorneeName: "HumanoidRootPart".to_owned(),
            BackgroundColor3: Color3::new(0.98039216f32, 0.98039216f32, 0.98039216f32),
            BackgroundTransparency: 0.1f64,
            BubbleDuration: 15f32,
            BubblesSpacing: 6f32,
            Enabled: true,
            Font: enums::Font::GothamMedium,
            FontFace: unimplemented!("Font"),
            LocalPlayerStudsOffset: Vector3::new(0f32, 0f32, 0f32),
            MaxBubbles: 3f32,
            MaxDistance: 100f32,
            MinimizeDistance: 40f32,
            TailVisible: true,
            TextColor3: Color3::new(0.22352941f32, 0.23137255f32, 0.23921569f32),
            TextSize: 16i64,
            VerticalStudsOffset: 0f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BubbleChatMessageProperties {
    superclass: TextChatMessageProperties,
}
impl_inherits!(BubbleChatMessageProperties, TextChatMessageProperties);
impl_strong_instance_from!(BubbleChatMessageProperties);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BugReporterService {
    superclass: Instance,
}
impl_inherits!(BugReporterService, Instance);
impl_strong_instance_from!(BugReporterService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct BulkImportService {
    superclass: Instance,
}
impl_inherits!(BulkImportService, Instance);
impl_strong_instance_from!(BulkImportService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BuoyancySensor {
    superclass: SensorBase,
    pub FullySubmerged: bool,
    pub TouchingSurface: bool,
}
impl_inherits!(BuoyancySensor, SensorBase);
impl_strong_instance_from!(BuoyancySensor);
impl Default for BuoyancySensor {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SensorBase {
            superclass,
            UpdateType: enums::SensorUpdateType::OnRead,
        };
        let superclass = BuoyancySensor {
            superclass,
            FullySubmerged: false,
            TouchingSurface: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CFrameValue {
    superclass: ValueBase,
    pub Value: CFrame,
}
impl_inherits!(CFrameValue, ValueBase);
impl_strong_instance_from!(CFrameValue);
impl Default for CFrameValue {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        let superclass = CFrameValue {
            superclass,
            Value: CFrame::identity(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CSGDictionaryService {
    superclass: FlyweightService,
}
impl_inherits!(CSGDictionaryService, FlyweightService);
impl_strong_instance_from!(CSGDictionaryService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CacheableContentProvider {
    superclass: Instance,
}
impl_inherits!(CacheableContentProvider, Instance);
impl_strong_instance_from!(CacheableContentProvider);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CalloutService {
    superclass: Instance,
}
impl_inherits!(CalloutService, Instance);
impl_strong_instance_from!(CalloutService);
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
impl_strong_instance_from!(Camera);
impl Default for Camera {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = Camera {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CanvasGroup {
    superclass: GuiObject,
    pub GroupColor3: Color3,
    pub GroupTransparency: f32,
}
impl_inherits!(CanvasGroup, GuiObject);
impl_strong_instance_from!(CanvasGroup);
impl Default for CanvasGroup {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = CanvasGroup {
            superclass,
            GroupColor3: Color3::new(1f32, 1f32, 1f32),
            GroupTransparency: 0f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Capture {
    superclass: Object,
}
impl_inherits!(Capture, Object);
impl_strong_instance_from!(Capture);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CaptureService {
    superclass: Instance,
}
impl_inherits!(CaptureService, Instance);
impl_strong_instance_from!(CaptureService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CapturesPages {
    superclass: Pages,
}
impl_inherits!(CapturesPages, Pages);
impl_strong_instance_from!(CapturesPages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CatalogPages {
    superclass: Pages,
}
impl_inherits!(CatalogPages, Pages);
impl_strong_instance_from!(CatalogPages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ChangeHistoryService {
    superclass: Instance,
}
impl_inherits!(ChangeHistoryService, Instance);
impl_strong_instance_from!(ChangeHistoryService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ChangeHistoryStreamingService {
    superclass: Instance,
}
impl_inherits!(ChangeHistoryStreamingService, Instance);
impl_strong_instance_from!(ChangeHistoryStreamingService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChannelSelectorSoundEffect {
    superclass: CustomSoundEffect,
    pub Channel: i32,
}
impl_inherits!(ChannelSelectorSoundEffect, CustomSoundEffect);
impl_strong_instance_from!(ChannelSelectorSoundEffect);
impl Default for ChannelSelectorSoundEffect {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundEffect {
            superclass,
            Enabled: false,
            Priority: 0i32,
        };
        let superclass = CustomSoundEffect { superclass };
        let superclass = ChannelSelectorSoundEffect {
            superclass,
            Channel: 1i32,
        };
        superclass
    }
}
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
impl_strong_instance_from!(ChannelTabsConfiguration);
impl Default for ChannelTabsConfiguration {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = TextChatConfigurations { superclass };
        let superclass = ChannelTabsConfiguration {
            superclass,
            BackgroundColor3: Color3::new(0.09803922f32, 0.105882354f32, 0.11372549f32),
            BackgroundTransparency: 0f64,
            Enabled: false,
            FontFace: unimplemented!("Font"),
            HoverBackgroundColor3: Color3::new(0.49019608f32, 0.49019608f32, 0.49019608f32),
            SelectedTabTextColor3: Color3::new(1f32, 1f32, 1f32),
            TextColor3: Color3::new(0.6862745f32, 0.6862745f32, 0.6862745f32),
            TextSize: 18i64,
            TextStrokeColor3: Color3::new(0f32, 0f32, 0f32),
            TextStrokeTransparency: 1f64,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CharacterAppearance {
    superclass: Instance,
}
impl_inherits!(CharacterAppearance, Instance);
impl_strong_instance_from!(CharacterAppearance);
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
impl_strong_instance_from!(CharacterMesh);
impl Default for CharacterMesh {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = CharacterMesh {
            superclass,
            BaseTextureId: 0i64,
            BodyPart: enums::BodyPart::Head,
            MeshId: 0i64,
            OverlayTextureId: 0i64,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Chat {
    superclass: Instance,
    pub BubbleChatEnabled: bool,
    pub IsAutoMigrated: bool,
    pub LoadDefaultChat: bool,
}
impl_inherits!(Chat, Instance);
impl_strong_instance_from!(Chat);
impl Default for Chat {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Chat {
            superclass,
            BubbleChatEnabled: false,
            IsAutoMigrated: false,
            LoadDefaultChat: true,
        };
        superclass
    }
}
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
impl_strong_instance_from!(ChatInputBarConfiguration);
impl Default for ChatInputBarConfiguration {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = TextChatConfigurations { superclass };
        let superclass = ChatInputBarConfiguration {
            superclass,
            AutocompleteEnabled: true,
            BackgroundColor3: Color3::new(0.09803922f32, 0.105882354f32, 0.11372549f32),
            BackgroundTransparency: 0.2f64,
            Enabled: true,
            FontFace: unimplemented!("Font"),
            KeyboardKeyCode: enums::KeyCode::Slash,
            PlaceholderColor3: Color3::new(0.69803923f32, 0.69803923f32, 0.69803923f32),
            TargetTextChannel: Ref::none(),
            TextColor3: Color3::new(1f32, 1f32, 1f32),
            TextSize: 14i64,
            TextStrokeColor3: Color3::new(0f32, 0f32, 0f32),
            TextStrokeTransparency: 0.5f64,
        };
        superclass
    }
}
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
impl_strong_instance_from!(ChatWindowConfiguration);
impl Default for ChatWindowConfiguration {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = TextChatConfigurations { superclass };
        let superclass = ChatWindowConfiguration {
            superclass,
            BackgroundColor3: Color3::new(0.09803922f32, 0.105882354f32, 0.11372549f32),
            BackgroundTransparency: 0.3f64,
            Enabled: true,
            FontFace: unimplemented!("Font"),
            HeightScale: 1f32,
            HorizontalAlignment: enums::HorizontalAlignment::Left,
            TextColor3: Color3::new(1f32, 1f32, 1f32),
            TextSize: 14i64,
            TextStrokeColor3: Color3::new(0f32, 0f32, 0f32),
            TextStrokeTransparency: 0.5f64,
            VerticalAlignment: enums::VerticalAlignment::Top,
            WidthScale: 1f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ChatWindowMessageProperties {
    superclass: TextChatMessageProperties,
}
impl_inherits!(ChatWindowMessageProperties, TextChatMessageProperties);
impl_strong_instance_from!(ChatWindowMessageProperties);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ChatbotUIService {
    superclass: Instance,
}
impl_inherits!(ChatbotUIService, Instance);
impl_strong_instance_from!(ChatbotUIService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChorusSoundEffect {
    superclass: SoundEffect,
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
impl_inherits!(ChorusSoundEffect, SoundEffect);
impl_strong_instance_from!(ChorusSoundEffect);
impl Default for ChorusSoundEffect {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundEffect {
            superclass,
            Enabled: false,
            Priority: 0i32,
        };
        let superclass = ChorusSoundEffect {
            superclass,
            Depth: 0.15f32,
            Mix: 0.5f32,
            Rate: 0.5f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ClickDetector {
    superclass: Instance,
    pub CursorIconContent: Content,
    pub MaxActivationDistance: f32,
}
impl_inherits!(ClickDetector, Instance);
impl_strong_instance_from!(ClickDetector);
impl Default for ClickDetector {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ClickDetector {
            superclass,
            CursorIconContent: Content::none(),
            MaxActivationDistance: 32f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ClientReplicator {
    superclass: NetworkReplicator,
}
impl_inherits!(ClientReplicator, NetworkReplicator);
impl_strong_instance_from!(ClientReplicator);
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
impl_strong_instance_from!(ClimbController);
impl Default for ClimbController {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ControllerBase {
            superclass,
            BalanceRigidityEnabled: false,
            MoveSpeedFactor: 0f32,
        };
        let superclass = ClimbController {
            superclass,
            AccelerationTime: 0f32,
            BalanceMaxTorque: 10000f32,
            BalanceSpeed: 100f32,
            MoveMaxForce: 10000f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Clothing {
    superclass: CharacterAppearance,
    pub Color3: Color3,
}
impl_inherits!(Clothing, CharacterAppearance);
impl_strong_instance_from!(Clothing);
impl Default for Clothing {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = Clothing {
            superclass,
            Color3: Color3::new(0f32, 0f32, 0f32),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CloudCRUDService {
    superclass: Instance,
}
impl_inherits!(CloudCRUDService, Instance);
impl_strong_instance_from!(CloudCRUDService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CloudLocalizationTable {
    superclass: LocalizationTable,
}
impl_inherits!(CloudLocalizationTable, LocalizationTable);
impl_strong_instance_from!(CloudLocalizationTable);
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
impl_strong_instance_from!(Clouds);
impl Default for Clouds {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Clouds {
            superclass,
            Color: Color3::new(1f32, 1f32, 1f32),
            Cover: 0.5f32,
            Density: 0.7f32,
            Enabled: true,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ClusterPacketCache {
    superclass: Instance,
}
impl_inherits!(ClusterPacketCache, Instance);
impl_strong_instance_from!(ClusterPacketCache);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Collaborator {
    superclass: Instance,
}
impl_inherits!(Collaborator, Instance);
impl_strong_instance_from!(Collaborator);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CollaboratorsService {
    superclass: Instance,
}
impl_inherits!(CollaboratorsService, Instance);
impl_strong_instance_from!(CollaboratorsService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CollectionService {
    superclass: Instance,
}
impl_inherits!(CollectionService, Instance);
impl_strong_instance_from!(CollectionService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Color3Value {
    superclass: ValueBase,
    pub Value: Color3,
}
impl_inherits!(Color3Value, ValueBase);
impl_strong_instance_from!(Color3Value);
impl Default for Color3Value {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        let superclass = Color3Value {
            superclass,
            Value: Color3::new(0f32, 0f32, 0f32),
        };
        superclass
    }
}
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
impl_strong_instance_from!(ColorCorrectionEffect);
impl Default for ColorCorrectionEffect {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PostEffect {
            superclass,
            Enabled: false,
        };
        let superclass = ColorCorrectionEffect {
            superclass,
            Brightness: 0f32,
            Contrast: 0f32,
            Saturation: 0f32,
            TintColor: Color3::new(1f32, 1f32, 1f32),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ColorGradingEffect {
    superclass: PostEffect,
    pub TonemapperPreset: enums::TonemapperPreset,
}
impl_inherits!(ColorGradingEffect, PostEffect);
impl_strong_instance_from!(ColorGradingEffect);
impl Default for ColorGradingEffect {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PostEffect {
            superclass,
            Enabled: false,
        };
        let superclass = ColorGradingEffect {
            superclass,
            TonemapperPreset: enums::TonemapperPreset::Default,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CommerceService {
    superclass: Instance,
}
impl_inherits!(CommerceService, Instance);
impl_strong_instance_from!(CommerceService);
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
impl_strong_instance_from!(CompressorSoundEffect);
impl Default for CompressorSoundEffect {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundEffect {
            superclass,
            Enabled: false,
            Priority: 0i32,
        };
        let superclass = CompressorSoundEffect {
            superclass,
            Attack: 0.1f32,
            GainMakeup: 0f32,
            Ratio: 40f32,
            Release: 0.1f32,
            SideChain: Ref::none(),
            Threshold: -40f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ConeHandleAdornment {
    superclass: HandleAdornment,
    pub Height: f32,
    pub Hollow: bool,
    pub Radius: f32,
    pub Shading: enums::AdornShading,
}
impl_inherits!(ConeHandleAdornment, HandleAdornment);
impl_strong_instance_from!(ConeHandleAdornment);
impl Default for ConeHandleAdornment {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = HandleAdornment {
            superclass,
            AdornCullingMode: enums::AdornCullingMode::Automatic,
            AlwaysOnTop: false,
            CFrame: CFrame::identity(),
            SizeRelativeOffset: Vector3::new(0f32, 0f32, 0f32),
            ZIndex: 0i32,
        };
        let superclass = ConeHandleAdornment {
            superclass,
            Height: 2f32,
            Hollow: false,
            Radius: 0.5f32,
            Shading: enums::AdornShading::Default,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ConfigService {
    superclass: Instance,
}
impl_inherits!(ConfigService, Instance);
impl_strong_instance_from!(ConfigService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ConfigSnapshot {
    superclass: Object,
}
impl_inherits!(ConfigSnapshot, Object);
impl_strong_instance_from!(ConfigSnapshot);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Configuration {
    superclass: Instance,
}
impl_inherits!(Configuration, Instance);
impl_strong_instance_from!(Configuration);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ConfigureServerService {
    superclass: Instance,
}
impl_inherits!(ConfigureServerService, Instance);
impl_strong_instance_from!(ConfigureServerService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ConnectivityService {
    superclass: Instance,
}
impl_inherits!(ConnectivityService, Instance);
impl_strong_instance_from!(ConnectivityService);
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
impl_strong_instance_from!(Constraint);
impl Default for Constraint {
    fn default() -> Self {
        let superclass = Object {};
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
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ContentProvider {
    superclass: Instance,
}
impl_inherits!(ContentProvider, Instance);
impl_strong_instance_from!(ContentProvider);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ContextActionService {
    superclass: Instance,
}
impl_inherits!(ContextActionService, Instance);
impl_strong_instance_from!(ContextActionService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Controller {
    superclass: Instance,
}
impl_inherits!(Controller, Instance);
impl_strong_instance_from!(Controller);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ControllerBase {
    superclass: Instance,
    pub BalanceRigidityEnabled: bool,
    pub MoveSpeedFactor: f32,
}
impl_inherits!(ControllerBase, Instance);
impl_strong_instance_from!(ControllerBase);
impl Default for ControllerBase {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ControllerBase {
            superclass,
            BalanceRigidityEnabled: false,
            MoveSpeedFactor: 0f32,
        };
        superclass
    }
}
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
impl_strong_instance_from!(ControllerManager);
impl Default for ControllerManager {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ControllerManager {
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
        };
        superclass
    }
}
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
impl_strong_instance_from!(ControllerPartSensor);
impl Default for ControllerPartSensor {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SensorBase {
            superclass,
            UpdateType: enums::SensorUpdateType::OnRead,
        };
        let superclass = ControllerSensor { superclass };
        let superclass = ControllerPartSensor {
            superclass,
            HitFrame: CFrame::identity(),
            HitNormal: Vector3::new(0f32, 0f32, 0f32),
            SearchDistance: 0f32,
            SensedPart: Ref::none(),
            SensorMode: enums::SensorMode::Floor,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ControllerSensor {
    superclass: SensorBase,
}
impl_inherits!(ControllerSensor, SensorBase);
impl_strong_instance_from!(ControllerSensor);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ControllerService {
    superclass: Instance,
}
impl_inherits!(ControllerService, Instance);
impl_strong_instance_from!(ControllerService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ConversationalAIAcceptanceService {
    superclass: Instance,
}
impl_inherits!(ConversationalAIAcceptanceService, Instance);
impl_strong_instance_from!(ConversationalAIAcceptanceService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CookiesService {
    superclass: Instance,
}
impl_inherits!(CookiesService, Instance);
impl_strong_instance_from!(CookiesService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CoreGui {
    superclass: BasePlayerGui,
    pub SelectionImageObject: Ref,
}
impl_inherits!(CoreGui, BasePlayerGui);
impl_strong_instance_from!(CoreGui);
impl Default for CoreGui {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = CoreGui {
            superclass,
            SelectionImageObject: Ref::none(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CorePackages {
    superclass: Instance,
}
impl_inherits!(CorePackages, Instance);
impl_strong_instance_from!(CorePackages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CoreScript {
    superclass: BaseScript,
}
impl_inherits!(CoreScript, BaseScript);
impl_strong_instance_from!(CoreScript);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CoreScriptDebuggingManagerHelper {
    superclass: Instance,
}
impl_inherits!(CoreScriptDebuggingManagerHelper, Instance);
impl_strong_instance_from!(CoreScriptDebuggingManagerHelper);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CoreScriptSyncService {
    superclass: Instance,
}
impl_inherits!(CoreScriptSyncService, Instance);
impl_strong_instance_from!(CoreScriptSyncService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CornerWedgePart {
    superclass: BasePart,
}
impl_inherits!(CornerWedgePart, BasePart);
impl_strong_instance_from!(CornerWedgePart);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CreationDBService {
    superclass: Instance,
}
impl_inherits!(CreationDBService, Instance);
impl_strong_instance_from!(CreationDBService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CreatorStoreService {
    superclass: Instance,
}
impl_inherits!(CreatorStoreService, Instance);
impl_strong_instance_from!(CreatorStoreService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CrossDMScriptChangeListener {
    superclass: Instance,
}
impl_inherits!(CrossDMScriptChangeListener, Instance);
impl_strong_instance_from!(CrossDMScriptChangeListener);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CurveAnimation {
    superclass: AnimationClip,
}
impl_inherits!(CurveAnimation, AnimationClip);
impl_strong_instance_from!(CurveAnimation);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CustomEvent {
    superclass: Instance,
    pub PersistedCurrentValue: f32,
}
impl_inherits!(CustomEvent, Instance);
impl_strong_instance_from!(CustomEvent);
impl Default for CustomEvent {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = CustomEvent {
            superclass,
            PersistedCurrentValue: 0f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CustomEventReceiver {
    superclass: Instance,
    pub Source: Ref,
}
impl_inherits!(CustomEventReceiver, Instance);
impl_strong_instance_from!(CustomEventReceiver);
impl Default for CustomEventReceiver {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = CustomEventReceiver {
            superclass,
            Source: Ref::none(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CustomLog {
    superclass: Instance,
}
impl_inherits!(CustomLog, Instance);
impl_strong_instance_from!(CustomLog);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CustomSoundEffect {
    superclass: SoundEffect,
}
impl_inherits!(CustomSoundEffect, SoundEffect);
impl_strong_instance_from!(CustomSoundEffect);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CylinderHandleAdornment {
    superclass: HandleAdornment,
    pub Angle: f32,
    pub Height: f32,
    pub InnerRadius: f32,
    pub Radius: f32,
    pub Shading: enums::AdornShading,
}
impl_inherits!(CylinderHandleAdornment, HandleAdornment);
impl_strong_instance_from!(CylinderHandleAdornment);
impl Default for CylinderHandleAdornment {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = HandleAdornment {
            superclass,
            AdornCullingMode: enums::AdornCullingMode::Automatic,
            AlwaysOnTop: false,
            CFrame: CFrame::identity(),
            SizeRelativeOffset: Vector3::new(0f32, 0f32, 0f32),
            ZIndex: 0i32,
        };
        let superclass = CylinderHandleAdornment {
            superclass,
            Angle: 360f32,
            Height: 1f32,
            InnerRadius: 0f32,
            Radius: 1f32,
            Shading: enums::AdornShading::Default,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct CylinderMesh {
    superclass: BevelMesh,
}
impl_inherits!(CylinderMesh, BevelMesh);
impl_strong_instance_from!(CylinderMesh);
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
impl_strong_instance_from!(CylindricalConstraint);
impl Default for CylindricalConstraint {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = SlidingBallConstraint {
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
        };
        let superclass = CylindricalConstraint {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataModel {
    superclass: ServiceProvider,
}
impl_inherits!(DataModel, ServiceProvider);
impl_strong_instance_from!(DataModel);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataModelMesh {
    superclass: Instance,
    pub Offset: Vector3,
    pub Scale: Vector3,
    pub VertexColor: Vector3,
}
impl_inherits!(DataModelMesh, Instance);
impl_strong_instance_from!(DataModelMesh);
impl Default for DataModelMesh {
    fn default() -> Self {
        let superclass = Object {};
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
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataModelPatchService {
    superclass: Instance,
}
impl_inherits!(DataModelPatchService, Instance);
impl_strong_instance_from!(DataModelPatchService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataModelSession {
    superclass: Instance,
}
impl_inherits!(DataModelSession, Instance);
impl_strong_instance_from!(DataModelSession);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStore {
    superclass: GlobalDataStore,
}
impl_inherits!(DataStore, GlobalDataStore);
impl_strong_instance_from!(DataStore);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreGetOptions {
    superclass: Instance,
    pub UseCache: bool,
}
impl_inherits!(DataStoreGetOptions, Instance);
impl_strong_instance_from!(DataStoreGetOptions);
impl Default for DataStoreGetOptions {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = DataStoreGetOptions {
            superclass,
            UseCache: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreIncrementOptions {
    superclass: Instance,
}
impl_inherits!(DataStoreIncrementOptions, Instance);
impl_strong_instance_from!(DataStoreIncrementOptions);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreInfo {
    superclass: Instance,
}
impl_inherits!(DataStoreInfo, Instance);
impl_strong_instance_from!(DataStoreInfo);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreKey {
    superclass: Instance,
}
impl_inherits!(DataStoreKey, Instance);
impl_strong_instance_from!(DataStoreKey);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreKeyInfo {
    superclass: Instance,
}
impl_inherits!(DataStoreKeyInfo, Instance);
impl_strong_instance_from!(DataStoreKeyInfo);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreKeyPages {
    superclass: Pages,
}
impl_inherits!(DataStoreKeyPages, Pages);
impl_strong_instance_from!(DataStoreKeyPages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreListingPages {
    superclass: Pages,
}
impl_inherits!(DataStoreListingPages, Pages);
impl_strong_instance_from!(DataStoreListingPages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreObjectVersionInfo {
    superclass: Instance,
}
impl_inherits!(DataStoreObjectVersionInfo, Instance);
impl_strong_instance_from!(DataStoreObjectVersionInfo);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreOptions {
    superclass: Instance,
    pub AllScopes: bool,
}
impl_inherits!(DataStoreOptions, Instance);
impl_strong_instance_from!(DataStoreOptions);
impl Default for DataStoreOptions {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = DataStoreOptions {
            superclass,
            AllScopes: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStorePages {
    superclass: Pages,
}
impl_inherits!(DataStorePages, Pages);
impl_strong_instance_from!(DataStorePages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreService {
    superclass: Instance,
    pub AutomaticRetry: bool,
    pub LegacyNamingScheme: bool,
}
impl_inherits!(DataStoreService, Instance);
impl_strong_instance_from!(DataStoreService);
impl Default for DataStoreService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = DataStoreService {
            superclass,
            AutomaticRetry: true,
            LegacyNamingScheme: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreSetOptions {
    superclass: Instance,
}
impl_inherits!(DataStoreSetOptions, Instance);
impl_strong_instance_from!(DataStoreSetOptions);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DataStoreVersionPages {
    superclass: Pages,
}
impl_inherits!(DataStoreVersionPages, Pages);
impl_strong_instance_from!(DataStoreVersionPages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Debris {
    superclass: Instance,
    pub MaxItems: i32,
}
impl_inherits!(Debris, Instance);
impl_strong_instance_from!(Debris);
impl Default for Debris {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Debris {
            superclass,
            MaxItems: 1000i32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebugSettings {
    superclass: Instance,
    pub IsScriptStackTracingEnabled: bool,
    pub ReportSoundWarnings: bool,
    pub TickCountPreciseOverride: enums::TickCountSampleMethod,
}
impl_inherits!(DebugSettings, Instance);
impl_strong_instance_from!(DebugSettings);
impl Default for DebugSettings {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = DebugSettings {
            superclass,
            IsScriptStackTracingEnabled: false,
            ReportSoundWarnings: false,
            TickCountPreciseOverride: enums::TickCountSampleMethod::Fast,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DebuggablePluginWatcher {
    superclass: Instance,
}
impl_inherits!(DebuggablePluginWatcher, Instance);
impl_strong_instance_from!(DebuggablePluginWatcher);
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
impl_strong_instance_from!(DebuggerBreakpoint);
impl Default for DebuggerBreakpoint {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = DebuggerBreakpoint {
            superclass,
            Condition: "".to_owned(),
            ContinueExecution: false,
            IsContextDependentBreakpoint: false,
            IsEnabled: false,
            Line: 0i32,
            LogExpression: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DebuggerConnection {
    superclass: Instance,
}
impl_inherits!(DebuggerConnection, Instance);
impl_strong_instance_from!(DebuggerConnection);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerConnectionManager {
    superclass: Instance,
    pub Timeout: f64,
}
impl_inherits!(DebuggerConnectionManager, Instance);
impl_strong_instance_from!(DebuggerConnectionManager);
impl Default for DebuggerConnectionManager {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = DebuggerConnectionManager {
            superclass,
            Timeout: 0f64,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DebuggerLuaResponse {
    superclass: Instance,
}
impl_inherits!(DebuggerLuaResponse, Instance);
impl_strong_instance_from!(DebuggerLuaResponse);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DebuggerManager {
    superclass: Instance,
}
impl_inherits!(DebuggerManager, Instance);
impl_strong_instance_from!(DebuggerManager);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DebuggerUIService {
    superclass: Instance,
}
impl_inherits!(DebuggerUIService, Instance);
impl_strong_instance_from!(DebuggerUIService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DebuggerVariable {
    superclass: Instance,
}
impl_inherits!(DebuggerVariable, Instance);
impl_strong_instance_from!(DebuggerVariable);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerWatch {
    superclass: Instance,
    pub Expression: String,
}
impl_inherits!(DebuggerWatch, Instance);
impl_strong_instance_from!(DebuggerWatch);
impl Default for DebuggerWatch {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = DebuggerWatch {
            superclass,
            Expression: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Decal {
    superclass: FaceInstance,
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
impl_strong_instance_from!(Decal);
impl Default for Decal {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = FaceInstance {
            superclass,
            Face: enums::NormalId::Right,
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
        superclass
    }
}
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
impl_strong_instance_from!(DepthOfFieldEffect);
impl Default for DepthOfFieldEffect {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PostEffect {
            superclass,
            Enabled: false,
        };
        let superclass = DepthOfFieldEffect {
            superclass,
            FarIntensity: 0.75f32,
            FocusDistance: 0.05f32,
            InFocusRadius: 10f32,
            NearIntensity: 0.75f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DeviceIdService {
    superclass: Instance,
}
impl_inherits!(DeviceIdService, Instance);
impl_strong_instance_from!(DeviceIdService);
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
impl_strong_instance_from!(Dialog);
impl Default for Dialog {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Dialog {
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
        };
        superclass
    }
}
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
impl_strong_instance_from!(DialogChoice);
impl Default for DialogChoice {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = DialogChoice {
            superclass,
            GoodbyeChoiceActive: true,
            GoodbyeDialog: "".to_owned(),
            ResponseDialog: "".to_owned(),
            UserDialog: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DistortionSoundEffect {
    superclass: SoundEffect,
    pub Level: f32,
}
impl_inherits!(DistortionSoundEffect, SoundEffect);
impl_strong_instance_from!(DistortionSoundEffect);
impl Default for DistortionSoundEffect {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundEffect {
            superclass,
            Enabled: false,
            Priority: 0i32,
        };
        let superclass = DistortionSoundEffect {
            superclass,
            Level: 0.75f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DockWidgetPluginGui {
    superclass: PluginGui,
}
impl_inherits!(DockWidgetPluginGui, PluginGui);
impl_strong_instance_from!(DockWidgetPluginGui);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DoubleConstrainedValue {
    superclass: ValueBase,
    pub MaxValue: f64,
    pub MinValue: f64,
    pub Value: f64,
}
impl_inherits!(DoubleConstrainedValue, ValueBase);
impl_strong_instance_from!(DoubleConstrainedValue);
impl Default for DoubleConstrainedValue {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        let superclass = DoubleConstrainedValue {
            superclass,
            MaxValue: 1f64,
            MinValue: 0f64,
            Value: 0f64,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DraftsService {
    superclass: Instance,
}
impl_inherits!(DraftsService, Instance);
impl_strong_instance_from!(DraftsService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DragDetector {
    superclass: ClickDetector,
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
impl_strong_instance_from!(DragDetector);
impl Default for DragDetector {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ClickDetector {
            superclass,
            CursorIconContent: Content::none(),
            MaxActivationDistance: 32f32,
        };
        let superclass = DragDetector {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Dragger {
    superclass: Instance,
}
impl_inherits!(Dragger, Instance);
impl_strong_instance_from!(Dragger);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct DraggerService {
    superclass: Instance,
}
impl_inherits!(DraggerService, Instance);
impl_strong_instance_from!(DraggerService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DynamicRotate {
    superclass: JointInstance,
    pub BaseAngle: f32,
}
impl_inherits!(DynamicRotate, JointInstance);
impl_strong_instance_from!(DynamicRotate);
impl Default for DynamicRotate {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = DynamicRotate {
            superclass,
            BaseAngle: 0f32,
        };
        superclass
    }
}
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
impl_strong_instance_from!(EchoSoundEffect);
impl Default for EchoSoundEffect {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundEffect {
            superclass,
            Enabled: false,
            Priority: 0i32,
        };
        let superclass = EchoSoundEffect {
            superclass,
            Delay: 1f32,
            DryLevel: 0f32,
            Feedback: 0.5f32,
            WetLevel: 0f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EditableImage {
    superclass: Object,
    pub ImageData: BinaryString,
}
impl_inherits!(EditableImage, Object);
impl_strong_instance_from!(EditableImage);
impl Default for EditableImage {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = EditableImage {
            superclass,
            ImageData: b"".as_slice().into(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EditableMesh {
    superclass: Object,
    pub MeshData: SharedString,
    pub SkinningEnabled: bool,
}
impl_inherits!(EditableMesh, Object);
impl_strong_instance_from!(EditableMesh);
impl Default for EditableMesh {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = EditableMesh {
            superclass,
            MeshData: SharedString::new(b"".to_vec()),
            SkinningEnabled: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct EditableService {
    superclass: Instance,
}
impl_inherits!(EditableService, Instance);
impl_strong_instance_from!(EditableService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct EmotesPages {
    superclass: InventoryPages,
}
impl_inherits!(EmotesPages, InventoryPages);
impl_strong_instance_from!(EmotesPages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct EncodingService {
    superclass: Instance,
}
impl_inherits!(EncodingService, Instance);
impl_strong_instance_from!(EncodingService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EqualizerSoundEffect {
    superclass: SoundEffect,
    pub HighGain: f32,
    pub LowGain: f32,
    pub MidGain: f32,
}
impl_inherits!(EqualizerSoundEffect, SoundEffect);
impl_strong_instance_from!(EqualizerSoundEffect);
impl Default for EqualizerSoundEffect {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundEffect {
            superclass,
            Enabled: false,
            Priority: 0i32,
        };
        let superclass = EqualizerSoundEffect {
            superclass,
            HighGain: 0f32,
            LowGain: -20f32,
            MidGain: -10f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EulerRotationCurve {
    superclass: Instance,
    pub RotationOrder: enums::RotationOrder,
}
impl_inherits!(EulerRotationCurve, Instance);
impl_strong_instance_from!(EulerRotationCurve);
impl Default for EulerRotationCurve {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = EulerRotationCurve {
            superclass,
            RotationOrder: enums::RotationOrder::XYZ,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct EventIngestService {
    superclass: Instance,
}
impl_inherits!(EventIngestService, Instance);
impl_strong_instance_from!(EventIngestService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExampleV2Service {
    superclass: Instance,
}
impl_inherits!(ExampleV2Service, Instance);
impl_strong_instance_from!(ExampleV2Service);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExecutedRemoteCommand {
    superclass: Object,
}
impl_inherits!(ExecutedRemoteCommand, Object);
impl_strong_instance_from!(ExecutedRemoteCommand);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExperienceAuthService {
    superclass: Instance,
}
impl_inherits!(ExperienceAuthService, Instance);
impl_strong_instance_from!(ExperienceAuthService);
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
impl_strong_instance_from!(ExperienceInviteOptions);
impl Default for ExperienceInviteOptions {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ExperienceInviteOptions {
            superclass,
            InviteMessageId: "".to_owned(),
            InviteUser: 0i64,
            LaunchData: "".to_owned(),
            PromptMessage: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExperienceNotificationService {
    superclass: Instance,
}
impl_inherits!(ExperienceNotificationService, Instance);
impl_strong_instance_from!(ExperienceNotificationService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExperienceService {
    superclass: Instance,
}
impl_inherits!(ExperienceService, Instance);
impl_strong_instance_from!(ExperienceService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExperienceStateCaptureService {
    superclass: Instance,
}
impl_inherits!(ExperienceStateCaptureService, Instance);
impl_strong_instance_from!(ExperienceStateCaptureService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExperienceStateRecordingService {
    superclass: Instance,
}
impl_inherits!(ExperienceStateRecordingService, Instance);
impl_strong_instance_from!(ExperienceStateRecordingService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExplorerFilter {
    superclass: Instance,
}
impl_inherits!(ExplorerFilter, Instance);
impl_strong_instance_from!(ExplorerFilter);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExplorerFilterAutocompleter {
    superclass: Instance,
}
impl_inherits!(ExplorerFilterAutocompleter, Instance);
impl_strong_instance_from!(ExplorerFilterAutocompleter);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ExplorerServiceVisibilityService {
    superclass: Instance,
}
impl_inherits!(ExplorerServiceVisibilityService, Instance);
impl_strong_instance_from!(ExplorerServiceVisibilityService);
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
impl_strong_instance_from!(Explosion);
impl Default for Explosion {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Explosion {
            superclass,
            BlastPressure: 500000f32,
            BlastRadius: 4f32,
            DestroyJointRadiusPercent: 1f32,
            ExplosionType: enums::ExplosionType::Craters,
            Position: Vector3::new(0f32, 0f32, 0f32),
            TimeScale: 1f32,
            Visible: true,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FaceAnimatorService {
    superclass: Instance,
}
impl_inherits!(FaceAnimatorService, Instance);
impl_strong_instance_from!(FaceAnimatorService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FaceControls {
    superclass: Instance,
}
impl_inherits!(FaceControls, Instance);
impl_strong_instance_from!(FaceControls);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FaceInstance {
    superclass: Instance,
    pub Face: enums::NormalId,
}
impl_inherits!(FaceInstance, Instance);
impl_strong_instance_from!(FaceInstance);
impl Default for FaceInstance {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = FaceInstance {
            superclass,
            Face: enums::NormalId::Right,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FacialAgeEstimationService {
    superclass: Instance,
}
impl_inherits!(FacialAgeEstimationService, Instance);
impl_strong_instance_from!(FacialAgeEstimationService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FacialAnimationRecordingService {
    superclass: Instance,
}
impl_inherits!(FacialAnimationRecordingService, Instance);
impl_strong_instance_from!(FacialAnimationRecordingService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FacialAnimationStreamingServiceStats {
    superclass: Instance,
}
impl_inherits!(FacialAnimationStreamingServiceStats, Instance);
impl_strong_instance_from!(FacialAnimationStreamingServiceStats);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FacialAnimationStreamingServiceV2 {
    superclass: Instance,
    pub ServiceState: i32,
}
impl_inherits!(FacialAnimationStreamingServiceV2, Instance);
impl_strong_instance_from!(FacialAnimationStreamingServiceV2);
impl Default for FacialAnimationStreamingServiceV2 {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = FacialAnimationStreamingServiceV2 {
            superclass,
            ServiceState: 0i32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FacialAnimationStreamingSubsessionStats {
    superclass: Instance,
}
impl_inherits!(FacialAnimationStreamingSubsessionStats, Instance);
impl_strong_instance_from!(FacialAnimationStreamingSubsessionStats);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FacsImportData {
    superclass: BaseImportData,
}
impl_inherits!(FacsImportData, BaseImportData);
impl_strong_instance_from!(FacsImportData);
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
impl_strong_instance_from!(Feature);
impl Default for Feature {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Feature {
            superclass,
            FaceId: enums::NormalId::Right,
            InOut: enums::InOut::Edge,
            LeftRight: enums::LeftRight::Left,
            TopBottom: enums::TopBottom::Top,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FeatureRestrictionManager {
    superclass: Instance,
}
impl_inherits!(FeatureRestrictionManager, Instance);
impl_strong_instance_from!(FeatureRestrictionManager);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct File {
    superclass: Instance,
}
impl_inherits!(File, Instance);
impl_strong_instance_from!(File);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FileMesh {
    superclass: DataModelMesh,
    pub MeshId: ContentId,
    pub TextureId: ContentId,
}
impl_inherits!(FileMesh, DataModelMesh);
impl_strong_instance_from!(FileMesh);
impl Default for FileMesh {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = FileMesh {
            superclass,
            MeshId: "".into(),
            TextureId: "".into(),
        };
        superclass
    }
}
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
impl_strong_instance_from!(Fire);
impl Default for Fire {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Fire {
            superclass,
            Color: Color3::new(0.92549026f32, 0.54509807f32, 0.27450982f32),
            Enabled: true,
            SecondaryColor: Color3::new(0.54509807f32, 0.3137255f32, 0.21568629f32),
            TimeScale: 1f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Flag {
    superclass: Tool,
    pub TeamColor: BrickColor,
}
impl_inherits!(Flag, Tool);
impl_strong_instance_from!(Flag);
impl Default for Flag {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = Flag {
            superclass,
            TeamColor: BrickColor::from_number(194u16).unwrap(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FlagStand {
    superclass: Part,
    pub TeamColor: BrickColor,
}
impl_inherits!(FlagStand, Part);
impl_strong_instance_from!(FlagStand);
impl Default for FlagStand {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = FormFactorPart { superclass };
        let superclass = Part { superclass };
        let superclass = FlagStand {
            superclass,
            TeamColor: BrickColor::from_number(194u16).unwrap(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FlagStandService {
    superclass: Instance,
}
impl_inherits!(FlagStandService, Instance);
impl_strong_instance_from!(FlagStandService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FlangeSoundEffect {
    superclass: SoundEffect,
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
impl_inherits!(FlangeSoundEffect, SoundEffect);
impl_strong_instance_from!(FlangeSoundEffect);
impl Default for FlangeSoundEffect {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundEffect {
            superclass,
            Enabled: false,
            Priority: 0i32,
        };
        let superclass = FlangeSoundEffect {
            superclass,
            Depth: 0.45f32,
            Mix: 0.85f32,
            Rate: 5f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FloatCurve {
    superclass: Instance,
    pub ValuesAndTimes: BinaryString,
}
impl_inherits!(FloatCurve, Instance);
impl_strong_instance_from!(FloatCurve);
impl Default for FloatCurve {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = FloatCurve {
            superclass,
            ValuesAndTimes: b"\x02\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0".as_slice().into(),
        };
        superclass
    }
}
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
impl_strong_instance_from!(FloorWire);
impl Default for FloorWire {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = FloorWire {
            superclass,
            CycleOffset: 0f32,
            From: Ref::none(),
            StudsBetweenTextures: 4f32,
            Texture: "".into(),
            TextureSize: Vector2::new(1f32, 1f32),
            To: Ref::none(),
            Velocity: 2f32,
            WireRadius: 0.0625f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FluidForceSensor {
    superclass: SensorBase,
}
impl_inherits!(FluidForceSensor, SensorBase);
impl_strong_instance_from!(FluidForceSensor);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FlyweightService {
    superclass: Instance,
}
impl_inherits!(FlyweightService, Instance);
impl_strong_instance_from!(FlyweightService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Folder {
    superclass: Instance,
}
impl_inherits!(Folder, Instance);
impl_strong_instance_from!(Folder);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ForceField {
    superclass: Instance,
    pub Visible: bool,
}
impl_inherits!(ForceField, Instance);
impl_strong_instance_from!(ForceField);
impl Default for ForceField {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ForceField {
            superclass,
            Visible: true,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FormFactorPart {
    superclass: BasePart,
}
impl_inherits!(FormFactorPart, BasePart);
impl_strong_instance_from!(FormFactorPart);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Frame {
    superclass: GuiObject,
    pub Style: enums::FrameStyle,
}
impl_inherits!(Frame, GuiObject);
impl_strong_instance_from!(Frame);
impl Default for Frame {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = Frame {
            superclass,
            Style: enums::FrameStyle::Custom,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FriendPages {
    superclass: Pages,
}
impl_inherits!(FriendPages, Pages);
impl_strong_instance_from!(FriendPages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct FriendService {
    superclass: Instance,
}
impl_inherits!(FriendService, Instance);
impl_strong_instance_from!(FriendService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FunctionalTest {
    superclass: Instance,
    pub Description: String,
    pub HasMigratedSettingsToTestService: bool,
}
impl_inherits!(FunctionalTest, Instance);
impl_strong_instance_from!(FunctionalTest);
impl Default for FunctionalTest {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = FunctionalTest {
            superclass,
            Description: "?".to_owned(),
            HasMigratedSettingsToTestService: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GamePassService {
    superclass: Instance,
}
impl_inherits!(GamePassService, Instance);
impl_strong_instance_from!(GamePassService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GameSettings {
    superclass: Instance,
    pub VideoCaptureEnabled: bool,
}
impl_inherits!(GameSettings, Instance);
impl_strong_instance_from!(GameSettings);
impl Default for GameSettings {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GameSettings {
            superclass,
            VideoCaptureEnabled: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GamepadService {
    superclass: Instance,
    pub GamepadCursorEnabled: bool,
}
impl_inherits!(GamepadService, Instance);
impl_strong_instance_from!(GamepadService);
impl Default for GamepadService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GamepadService {
            superclass,
            GamepadCursorEnabled: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GenerationService {
    superclass: Instance,
}
impl_inherits!(GenerationService, Instance);
impl_strong_instance_from!(GenerationService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GenericChallengeService {
    superclass: Instance,
}
impl_inherits!(GenericChallengeService, Instance);
impl_strong_instance_from!(GenericChallengeService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GenericSettings {
    superclass: ServiceProvider,
}
impl_inherits!(GenericSettings, ServiceProvider);
impl_strong_instance_from!(GenericSettings);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Geometry {
    superclass: Instance,
}
impl_inherits!(Geometry, Instance);
impl_strong_instance_from!(Geometry);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GeometryService {
    superclass: Instance,
}
impl_inherits!(GeometryService, Instance);
impl_strong_instance_from!(GeometryService);
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
impl_strong_instance_from!(GetTextBoundsParams);
impl Default for GetTextBoundsParams {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GetTextBoundsParams {
            superclass,
            Font: unimplemented!("Font"),
            RichText: false,
            Size: 0f32,
            Text: "".to_owned(),
            Width: 0f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GlobalDataStore {
    superclass: Instance,
}
impl_inherits!(GlobalDataStore, Instance);
impl_strong_instance_from!(GlobalDataStore);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GlobalSettings {
    superclass: GenericSettings,
}
impl_inherits!(GlobalSettings, GenericSettings);
impl_strong_instance_from!(GlobalSettings);
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
impl_strong_instance_from!(Glue);
impl Default for Glue {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = Glue {
            superclass,
            F0: Vector3::new(0f32, 0f32, 0f32),
            F1: Vector3::new(0f32, 0f32, 0f32),
            F2: Vector3::new(0f32, 0f32, 0f32),
            F3: Vector3::new(0f32, 0f32, 0f32),
        };
        superclass
    }
}
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
impl_strong_instance_from!(GroundController);
impl Default for GroundController {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ControllerBase {
            superclass,
            BalanceRigidityEnabled: false,
            MoveSpeedFactor: 0f32,
        };
        let superclass = GroundController {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GroupImportData {
    superclass: BaseImportData,
    pub Anchored: bool,
    pub ImportAsModelAsset: bool,
    pub InsertInWorkspace: bool,
}
impl_inherits!(GroupImportData, BaseImportData);
impl_strong_instance_from!(GroupImportData);
impl Default for GroupImportData {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = GroupImportData {
            superclass,
            Anchored: false,
            ImportAsModelAsset: false,
            InsertInWorkspace: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GroupService {
    superclass: Instance,
}
impl_inherits!(GroupService, Instance);
impl_strong_instance_from!(GroupService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GuiBase {
    superclass: Instance,
}
impl_inherits!(GuiBase, Instance);
impl_strong_instance_from!(GuiBase);
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
impl_strong_instance_from!(GuiBase2d);
impl Default for GuiBase2d {
    fn default() -> Self {
        let superclass = Object {};
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
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiBase3d {
    superclass: GuiBase,
    pub Color3: Color3,
    pub Transparency: f32,
    pub Visible: bool,
}
impl_inherits!(GuiBase3d, GuiBase);
impl_strong_instance_from!(GuiBase3d);
impl Default for GuiBase3d {
    fn default() -> Self {
        let superclass = Object {};
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
        superclass
    }
}
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
impl_strong_instance_from!(GuiButton);
impl Default for GuiButton {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = GuiButton {
            superclass,
            AutoButtonColor: false,
            HoverHapticEffect: Ref::none(),
            Modal: false,
            PressHapticEffect: Ref::none(),
            Selected: false,
            Style: enums::ButtonStyle::Custom,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GuiLabel {
    superclass: GuiObject,
}
impl_inherits!(GuiLabel, GuiObject);
impl_strong_instance_from!(GuiLabel);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GuiMain {
    superclass: ScreenGui,
}
impl_inherits!(GuiMain, ScreenGui);
impl_strong_instance_from!(GuiMain);
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
impl_strong_instance_from!(GuiObject);
impl Default for GuiObject {
    fn default() -> Self {
        let superclass = Object {};
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
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiService {
    superclass: Instance,
    pub AutoSelectGuiEnabled: bool,
    pub GuiNavigationEnabled: bool,
    pub SelectedObject: Ref,
}
impl_inherits!(GuiService, Instance);
impl_strong_instance_from!(GuiService);
impl Default for GuiService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = GuiService {
            superclass,
            AutoSelectGuiEnabled: false,
            GuiNavigationEnabled: false,
            SelectedObject: Ref::none(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct GuidRegistryService {
    superclass: Instance,
}
impl_inherits!(GuidRegistryService, Instance);
impl_strong_instance_from!(GuidRegistryService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HSRDataContentProvider {
    superclass: CacheableContentProvider,
}
impl_inherits!(HSRDataContentProvider, CacheableContentProvider);
impl_strong_instance_from!(HSRDataContentProvider);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HandRigDescription {
    superclass: Instance,
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
impl_strong_instance_from!(HandRigDescription);
impl Default for HandRigDescription {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = HandRigDescription {
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
        };
        superclass
    }
}
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
impl_strong_instance_from!(HandleAdornment);
impl Default for HandleAdornment {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = HandleAdornment {
            superclass,
            AdornCullingMode: enums::AdornCullingMode::Automatic,
            AlwaysOnTop: false,
            CFrame: CFrame::identity(),
            SizeRelativeOffset: Vector3::new(0f32, 0f32, 0f32),
            ZIndex: 0i32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Handles {
    superclass: HandlesBase,
    pub Faces: Faces,
    pub Style: enums::HandlesStyle,
}
impl_inherits!(Handles, HandlesBase);
impl_strong_instance_from!(Handles);
impl Default for Handles {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = PartAdornment {
            superclass,
            Adornee: Ref::none(),
        };
        let superclass = HandlesBase { superclass };
        let superclass = Handles {
            superclass,
            Faces: unimplemented!(),
            Style: enums::HandlesStyle::Resize,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HandlesBase {
    superclass: PartAdornment,
}
impl_inherits!(HandlesBase, PartAdornment);
impl_strong_instance_from!(HandlesBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HapticEffect {
    superclass: Instance,
    pub Looped: bool,
    pub Position: Vector3,
    pub Radius: f32,
    pub Type: enums::HapticEffectType,
    pub Waveform: Ref,
    pub WaveformData: BinaryString,
}
impl_inherits!(HapticEffect, Instance);
impl_strong_instance_from!(HapticEffect);
impl Default for HapticEffect {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = HapticEffect {
            superclass,
            Looped: false,
            Position: Vector3::new(0f32, 0f32, 0f32),
            Radius: 3f32,
            Type: enums::HapticEffectType::UIClick,
            Waveform: Ref::none(),
            WaveformData: b"".as_slice().into(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HapticService {
    superclass: Instance,
}
impl_inherits!(HapticService, Instance);
impl_strong_instance_from!(HapticService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HarmonyService {
    superclass: Instance,
}
impl_inherits!(HarmonyService, Instance);
impl_strong_instance_from!(HarmonyService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Hat {
    superclass: Accoutrement,
}
impl_inherits!(Hat, Accoutrement);
impl_strong_instance_from!(Hat);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HeapProfilerService {
    superclass: Instance,
}
impl_inherits!(HeapProfilerService, Instance);
impl_strong_instance_from!(HeapProfilerService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HeatmapService {
    superclass: Instance,
}
impl_inherits!(HeatmapService, Instance);
impl_strong_instance_from!(HeatmapService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HeightmapImporterService {
    superclass: Instance,
}
impl_inherits!(HeightmapImporterService, Instance);
impl_strong_instance_from!(HeightmapImporterService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HiddenSurfaceRemovalAsset {
    superclass: Instance,
    pub HsrData: BinaryString,
    pub HsrMeshIdData: BinaryString,
}
impl_inherits!(HiddenSurfaceRemovalAsset, Instance);
impl_strong_instance_from!(HiddenSurfaceRemovalAsset);
impl Default for HiddenSurfaceRemovalAsset {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = HiddenSurfaceRemovalAsset {
            superclass,
            HsrData: b"".as_slice().into(),
            HsrMeshIdData: b"".as_slice().into(),
        };
        superclass
    }
}
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
impl_strong_instance_from!(Highlight);
impl Default for Highlight {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Highlight {
            superclass,
            Adornee: Ref::none(),
            DepthMode: enums::HighlightDepthMode::AlwaysOnTop,
            Enabled: true,
            FillColor: Color3::new(1f32, 0f32, 0f32),
            FillTransparency: 0.5f32,
            OutlineColor: Color3::new(1f32, 1f32, 1f32),
            OutlineTransparency: 0f32,
        };
        superclass
    }
}
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
impl_strong_instance_from!(HingeConstraint);
impl Default for HingeConstraint {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = HingeConstraint {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Hint {
    superclass: Message,
}
impl_inherits!(Hint, Message);
impl_strong_instance_from!(Hint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Hole {
    superclass: Feature,
}
impl_inherits!(Hole, Feature);
impl_strong_instance_from!(Hole);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Hopper {
    superclass: Instance,
}
impl_inherits!(Hopper, Instance);
impl_strong_instance_from!(Hopper);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HopperBin {
    superclass: BackpackItem,
    pub Active: bool,
    pub BinType: enums::BinType,
}
impl_inherits!(HopperBin, BackpackItem);
impl_strong_instance_from!(HopperBin);
impl Default for HopperBin {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = BackpackItem {
            superclass,
            TextureContent: Content::none(),
        };
        let superclass = HopperBin {
            superclass,
            Active: false,
            BinType: enums::BinType::Script,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HttpRbxApiService {
    superclass: Instance,
}
impl_inherits!(HttpRbxApiService, Instance);
impl_strong_instance_from!(HttpRbxApiService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HttpRequest {
    superclass: Instance,
}
impl_inherits!(HttpRequest, Instance);
impl_strong_instance_from!(HttpRequest);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HttpService {
    superclass: Instance,
    pub HttpEnabled: bool,
}
impl_inherits!(HttpService, Instance);
impl_strong_instance_from!(HttpService);
impl Default for HttpService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = HttpService {
            superclass,
            HttpEnabled: false,
        };
        superclass
    }
}
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
impl_strong_instance_from!(Humanoid);
impl Default for Humanoid {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Humanoid {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct HumanoidController {
    superclass: Controller,
}
impl_inherits!(HumanoidController, Controller);
impl_strong_instance_from!(HumanoidController);
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
impl_strong_instance_from!(HumanoidDescription);
impl Default for HumanoidDescription {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = HumanoidDescription {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HumanoidRigDescription {
    superclass: Instance,
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
impl_strong_instance_from!(HumanoidRigDescription);
impl Default for HumanoidRigDescription {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = HumanoidRigDescription {
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
        };
        superclass
    }
}
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
impl_strong_instance_from!(IKControl);
impl Default for IKControl {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = IKControl {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ILegacyStudioBridge {
    superclass: Instance,
}
impl_inherits!(ILegacyStudioBridge, Instance);
impl_strong_instance_from!(ILegacyStudioBridge);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct IXPService {
    superclass: Instance,
}
impl_inherits!(IXPService, Instance);
impl_strong_instance_from!(IXPService);
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
impl_strong_instance_from!(ImageButton);
impl Default for ImageButton {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = GuiButton {
            superclass,
            AutoButtonColor: false,
            HoverHapticEffect: Ref::none(),
            Modal: false,
            PressHapticEffect: Ref::none(),
            Selected: false,
            Style: enums::ButtonStyle::Custom,
        };
        let superclass = ImageButton {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ImageHandleAdornment {
    superclass: HandleAdornment,
    pub Image: ContentId,
    pub Size: Vector2,
}
impl_inherits!(ImageHandleAdornment, HandleAdornment);
impl_strong_instance_from!(ImageHandleAdornment);
impl Default for ImageHandleAdornment {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = HandleAdornment {
            superclass,
            AdornCullingMode: enums::AdornCullingMode::Automatic,
            AlwaysOnTop: false,
            CFrame: CFrame::identity(),
            SizeRelativeOffset: Vector3::new(0f32, 0f32, 0f32),
            ZIndex: 0i32,
        };
        let superclass = ImageHandleAdornment {
            superclass,
            Image: "rbxasset://textures/SurfacesDefault.png".into(),
            Size: Vector2::new(1f32, 1f32),
        };
        superclass
    }
}
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
impl_strong_instance_from!(ImageLabel);
impl Default for ImageLabel {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = GuiLabel { superclass };
        let superclass = ImageLabel {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ImportSession {
    superclass: Instance,
}
impl_inherits!(ImportSession, Instance);
impl_strong_instance_from!(ImportSession);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct IncrementalPatchBuilder {
    superclass: Instance,
    pub AddPathsToBundle: bool,
    pub BuildDebouncePeriod: f64,
    pub HighCompression: bool,
    pub SerializePatch: bool,
    pub UseFileLevelCompressionInsteadOfChunk: bool,
    pub ZstdCompression: bool,
}
impl_inherits!(IncrementalPatchBuilder, Instance);
impl_strong_instance_from!(IncrementalPatchBuilder);
impl Default for IncrementalPatchBuilder {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = IncrementalPatchBuilder {
            superclass,
            AddPathsToBundle: false,
            BuildDebouncePeriod: 0f64,
            HighCompression: false,
            SerializePatch: false,
            UseFileLevelCompressionInsteadOfChunk: false,
            ZstdCompression: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InputAction {
    superclass: Instance,
    pub Enabled: bool,
    pub Type: enums::InputActionType,
}
impl_inherits!(InputAction, Instance);
impl_strong_instance_from!(InputAction);
impl Default for InputAction {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = InputAction {
            superclass,
            Enabled: true,
            Type: enums::InputActionType::Bool,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InputBinding {
    superclass: Instance,
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
impl_strong_instance_from!(InputBinding);
impl Default for InputBinding {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = InputBinding {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InputContext {
    superclass: Instance,
    pub Enabled: bool,
    pub Priority: i32,
    pub Sink: bool,
}
impl_inherits!(InputContext, Instance);
impl_strong_instance_from!(InputContext);
impl Default for InputContext {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = InputContext {
            superclass,
            Enabled: true,
            Priority: 1000i32,
            Sink: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct InputObject {
    superclass: Instance,
}
impl_inherits!(InputObject, Instance);
impl_strong_instance_from!(InputObject);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InsertService {
    superclass: Instance,
    pub AllowClientInsertModels: bool,
    pub AllowInsertFreeModels: bool,
}
impl_inherits!(InsertService, Instance);
impl_strong_instance_from!(InsertService);
impl Default for InsertService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = InsertService {
            superclass,
            AllowClientInsertModels: false,
            AllowInsertFreeModels: false,
        };
        superclass
    }
}
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
impl_strong_instance_from!(Instance);
impl Default for Instance {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InstanceAdornment {
    superclass: GuiBase3d,
    pub Adornee: Ref,
}
impl_inherits!(InstanceAdornment, GuiBase3d);
impl_strong_instance_from!(InstanceAdornment);
impl Default for InstanceAdornment {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = InstanceAdornment {
            superclass,
            Adornee: Ref::none(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct InstanceExtensionsService {
    superclass: Instance,
}
impl_inherits!(InstanceExtensionsService, Instance);
impl_strong_instance_from!(InstanceExtensionsService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct IntConstrainedValue {
    superclass: ValueBase,
    pub MaxValue: i64,
    pub MinValue: i64,
    pub Value: i64,
}
impl_inherits!(IntConstrainedValue, ValueBase);
impl_strong_instance_from!(IntConstrainedValue);
impl Default for IntConstrainedValue {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        let superclass = IntConstrainedValue {
            superclass,
            MaxValue: 10i64,
            MinValue: 0i64,
            Value: 0i64,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct IntValue {
    superclass: ValueBase,
    pub Value: i64,
}
impl_inherits!(IntValue, ValueBase);
impl_strong_instance_from!(IntValue);
impl Default for IntValue {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        let superclass = IntValue {
            superclass,
            Value: 0i64,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InternalSyncItem {
    superclass: Instance,
    pub AutoSync: bool,
    pub Enabled: bool,
    pub Path: String,
}
impl_inherits!(InternalSyncItem, Instance);
impl_strong_instance_from!(InternalSyncItem);
impl Default for InternalSyncItem {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = InternalSyncItem {
            superclass,
            AutoSync: false,
            Enabled: false,
            Path: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct InternalSyncService {
    superclass: Instance,
}
impl_inherits!(InternalSyncService, Instance);
impl_strong_instance_from!(InternalSyncService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct IntersectOperation {
    superclass: PartOperation,
}
impl_inherits!(IntersectOperation, PartOperation);
impl_strong_instance_from!(IntersectOperation);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct InventoryPages {
    superclass: Pages,
}
impl_inherits!(InventoryPages, Pages);
impl_strong_instance_from!(InventoryPages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct JointImportData {
    superclass: BaseImportData,
}
impl_inherits!(JointImportData, BaseImportData);
impl_strong_instance_from!(JointImportData);
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
impl_strong_instance_from!(JointInstance);
impl Default for JointInstance {
    fn default() -> Self {
        let superclass = Object {};
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
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct JointsService {
    superclass: Instance,
}
impl_inherits!(JointsService, Instance);
impl_strong_instance_from!(JointsService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct KeyboardService {
    superclass: Instance,
}
impl_inherits!(KeyboardService, Instance);
impl_strong_instance_from!(KeyboardService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Keyframe {
    superclass: Instance,
    pub Time: f32,
}
impl_inherits!(Keyframe, Instance);
impl_strong_instance_from!(Keyframe);
impl Default for Keyframe {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Keyframe {
            superclass,
            Time: 0f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct KeyframeMarker {
    superclass: Instance,
    pub Value: String,
}
impl_inherits!(KeyframeMarker, Instance);
impl_strong_instance_from!(KeyframeMarker);
impl Default for KeyframeMarker {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = KeyframeMarker {
            superclass,
            Value: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct KeyframeSequence {
    superclass: AnimationClip,
    pub AuthoredHipHeight: f32,
}
impl_inherits!(KeyframeSequence, AnimationClip);
impl_strong_instance_from!(KeyframeSequence);
impl Default for KeyframeSequence {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = AnimationClip {
            superclass,
            GuidBinaryString: b"".as_slice().into(),
            Loop: false,
            Priority: enums::AnimationPriority::Idle,
        };
        let superclass = KeyframeSequence {
            superclass,
            AuthoredHipHeight: 2f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct KeyframeSequenceProvider {
    superclass: Instance,
}
impl_inherits!(KeyframeSequenceProvider, Instance);
impl_strong_instance_from!(KeyframeSequenceProvider);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LSPFileSyncService {
    superclass: Instance,
}
impl_inherits!(LSPFileSyncService, Instance);
impl_strong_instance_from!(LSPFileSyncService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LanguageService {
    superclass: Instance,
}
impl_inherits!(LanguageService, Instance);
impl_strong_instance_from!(LanguageService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LayerCollector {
    superclass: GuiBase2d,
    pub Enabled: bool,
    pub ResetOnSpawn: bool,
    pub ZIndexBehavior: enums::ZIndexBehavior,
}
impl_inherits!(LayerCollector, GuiBase2d);
impl_strong_instance_from!(LayerCollector);
impl Default for LayerCollector {
    fn default() -> Self {
        let superclass = Object {};
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
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LegacyStudioBridge {
    superclass: ILegacyStudioBridge,
}
impl_inherits!(LegacyStudioBridge, ILegacyStudioBridge);
impl_strong_instance_from!(LegacyStudioBridge);
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
impl_strong_instance_from!(Light);
impl Default for Light {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Light {
            superclass,
            Brightness: 0f32,
            Color: Color3::new(0f32, 0f32, 0f32),
            Enabled: false,
            Shadows: false,
        };
        superclass
    }
}
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
impl_strong_instance_from!(Lighting);
impl Default for Lighting {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Lighting {
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
        };
        superclass
    }
}
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
impl_strong_instance_from!(LineForce);
impl Default for LineForce {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = LineForce {
            superclass,
            ApplyAtCenterOfMass: false,
            InverseSquareLaw: false,
            Magnitude: 1000f32,
            MaxForce: f32::INFINITY,
            ReactionForceEnabled: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LineHandleAdornment {
    superclass: HandleAdornment,
    pub Length: f32,
    pub Thickness: f32,
}
impl_inherits!(LineHandleAdornment, HandleAdornment);
impl_strong_instance_from!(LineHandleAdornment);
impl Default for LineHandleAdornment {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = HandleAdornment {
            superclass,
            AdornCullingMode: enums::AdornCullingMode::Automatic,
            AlwaysOnTop: false,
            CFrame: CFrame::identity(),
            SizeRelativeOffset: Vector3::new(0f32, 0f32, 0f32),
            ZIndex: 0i32,
        };
        let superclass = LineHandleAdornment {
            superclass,
            Length: 5f32,
            Thickness: 1f32,
        };
        superclass
    }
}
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
    pub ReactionForceEnabled: bool,
    pub RelativeTo: enums::ActuatorRelativeTo,
    pub SecondaryTangentAxis: Vector3,
    pub VectorVelocity: Vector3,
    pub VelocityConstraintMode: enums::VelocityConstraintMode,
}
impl_inherits!(LinearVelocity, Constraint);
impl_strong_instance_from!(LinearVelocity);
impl Default for LinearVelocity {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = LinearVelocity {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LinkingService {
    superclass: Instance,
}
impl_inherits!(LinkingService, Instance);
impl_strong_instance_from!(LinkingService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LiveScriptingService {
    superclass: Instance,
}
impl_inherits!(LiveScriptingService, Instance);
impl_strong_instance_from!(LiveScriptingService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LiveSyncService {
    superclass: Instance,
}
impl_inherits!(LiveSyncService, Instance);
impl_strong_instance_from!(LiveSyncService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LocalDebuggerConnection {
    superclass: DebuggerConnection,
}
impl_inherits!(LocalDebuggerConnection, DebuggerConnection);
impl_strong_instance_from!(LocalDebuggerConnection);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LocalScript {
    superclass: Script,
}
impl_inherits!(LocalScript, Script);
impl_strong_instance_from!(LocalScript);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LocalStorageService {
    superclass: Instance,
}
impl_inherits!(LocalStorageService, Instance);
impl_strong_instance_from!(LocalStorageService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LocalizationService {
    superclass: Instance,
}
impl_inherits!(LocalizationService, Instance);
impl_strong_instance_from!(LocalizationService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LocalizationTable {
    superclass: Instance,
    pub Contents: String,
    pub SourceLocaleId: String,
}
impl_inherits!(LocalizationTable, Instance);
impl_strong_instance_from!(LocalizationTable);
impl Default for LocalizationTable {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = LocalizationTable {
            superclass,
            Contents: "[]".to_owned(),
            SourceLocaleId: "en-us".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LodDataEntity {
    superclass: Instance,
}
impl_inherits!(LodDataEntity, Instance);
impl_strong_instance_from!(LodDataEntity);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LodDataService {
    superclass: Instance,
}
impl_inherits!(LodDataService, Instance);
impl_strong_instance_from!(LodDataService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LogReporterService {
    superclass: Instance,
}
impl_inherits!(LogReporterService, Instance);
impl_strong_instance_from!(LogReporterService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LogService {
    superclass: Instance,
}
impl_inherits!(LogService, Instance);
impl_strong_instance_from!(LogService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LoginService {
    superclass: Instance,
}
impl_inherits!(LoginService, Instance);
impl_strong_instance_from!(LoginService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LuaSettings {
    superclass: Instance,
}
impl_inherits!(LuaSettings, Instance);
impl_strong_instance_from!(LuaSettings);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LuaSourceContainer {
    superclass: Instance,
    pub ScriptGuid: String,
}
impl_inherits!(LuaSourceContainer, Instance);
impl_strong_instance_from!(LuaSourceContainer);
impl Default for LuaSourceContainer {
    fn default() -> Self {
        let superclass = Object {};
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
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LuaWebService {
    superclass: Instance,
}
impl_inherits!(LuaWebService, Instance);
impl_strong_instance_from!(LuaWebService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct LuauScriptAnalyzerService {
    superclass: Instance,
}
impl_inherits!(LuauScriptAnalyzerService, Instance);
impl_strong_instance_from!(LuauScriptAnalyzerService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MLModelDeliveryService {
    superclass: Instance,
}
impl_inherits!(MLModelDeliveryService, Instance);
impl_strong_instance_from!(MLModelDeliveryService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MLService {
    superclass: Instance,
}
impl_inherits!(MLService, Instance);
impl_strong_instance_from!(MLService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MLSession {
    superclass: Object,
}
impl_inherits!(MLSession, Object);
impl_strong_instance_from!(MLSession);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ManualGlue {
    superclass: ManualSurfaceJointInstance,
}
impl_inherits!(ManualGlue, ManualSurfaceJointInstance);
impl_strong_instance_from!(ManualGlue);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ManualSurfaceJointInstance {
    superclass: JointInstance,
}
impl_inherits!(ManualSurfaceJointInstance, JointInstance);
impl_strong_instance_from!(ManualSurfaceJointInstance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ManualWeld {
    superclass: ManualSurfaceJointInstance,
}
impl_inherits!(ManualWeld, ManualSurfaceJointInstance);
impl_strong_instance_from!(ManualWeld);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MarkerCurve {
    superclass: Instance,
    pub ValuesAndTimes: BinaryString,
}
impl_inherits!(MarkerCurve, Instance);
impl_strong_instance_from!(MarkerCurve);
impl Default for MarkerCurve {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = MarkerCurve {
            superclass,
            ValuesAndTimes: b"\x02\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0".as_slice().into(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MarketplaceService {
    superclass: Instance,
}
impl_inherits!(MarketplaceService, Instance);
impl_strong_instance_from!(MarketplaceService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MatchmakingService {
    superclass: Instance,
}
impl_inherits!(MatchmakingService, Instance);
impl_strong_instance_from!(MatchmakingService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MaterialGenerationService {
    superclass: Instance,
}
impl_inherits!(MaterialGenerationService, Instance);
impl_strong_instance_from!(MaterialGenerationService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MaterialImportData {
    superclass: BaseImportData,
    pub DiffuseFilePath: String,
    pub EmissiveFilePath: String,
    pub MetalnessFilePath: String,
    pub NormalFilePath: String,
    pub RoughnessFilePath: String,
}
impl_inherits!(MaterialImportData, BaseImportData);
impl_strong_instance_from!(MaterialImportData);
impl Default for MaterialImportData {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = MaterialImportData {
            superclass,
            DiffuseFilePath: "".to_owned(),
            EmissiveFilePath: "".to_owned(),
            MetalnessFilePath: "".to_owned(),
            NormalFilePath: "".to_owned(),
            RoughnessFilePath: "".to_owned(),
        };
        superclass
    }
}
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
impl_strong_instance_from!(MaterialService);
impl Default for MaterialService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = MaterialService {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MaterialVariant {
    superclass: Instance,
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
impl_strong_instance_from!(MaterialVariant);
impl Default for MaterialVariant {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = MaterialVariant {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MemStorageConnection {
    superclass: Instance,
}
impl_inherits!(MemStorageConnection, Instance);
impl_strong_instance_from!(MemStorageConnection);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MemStorageService {
    superclass: Instance,
}
impl_inherits!(MemStorageService, Instance);
impl_strong_instance_from!(MemStorageService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MemoryStoreHashMap {
    superclass: Instance,
}
impl_inherits!(MemoryStoreHashMap, Instance);
impl_strong_instance_from!(MemoryStoreHashMap);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MemoryStoreHashMapPages {
    superclass: Pages,
}
impl_inherits!(MemoryStoreHashMapPages, Pages);
impl_strong_instance_from!(MemoryStoreHashMapPages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MemoryStoreQueue {
    superclass: Instance,
}
impl_inherits!(MemoryStoreQueue, Instance);
impl_strong_instance_from!(MemoryStoreQueue);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MemoryStoreService {
    superclass: Instance,
}
impl_inherits!(MemoryStoreService, Instance);
impl_strong_instance_from!(MemoryStoreService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MemoryStoreSortedMap {
    superclass: Instance,
}
impl_inherits!(MemoryStoreSortedMap, Instance);
impl_strong_instance_from!(MemoryStoreSortedMap);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MeshContentProvider {
    superclass: CacheableContentProvider,
}
impl_inherits!(MeshContentProvider, CacheableContentProvider);
impl_strong_instance_from!(MeshContentProvider);
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
impl_strong_instance_from!(MeshImportData);
impl Default for MeshImportData {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = MeshImportData {
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
        };
        superclass
    }
}
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
impl_strong_instance_from!(MeshPart);
impl Default for MeshPart {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = TriangleMeshPart {
            superclass,
            AeroMeshData: SharedString::new(b"".to_vec()),
            FluidFidelityInternal: enums::FluidFidelity::Automatic,
            InertiaMigrated: false,
            PhysicalConfigData: SharedString::new(b"".to_vec()),
            UnscaledCofm: Vector3::new(0f32, 0f32, 0f32),
            UnscaledVolInertiaDiags: Vector3::new(0f32, 0f32, 0f32),
            UnscaledVolInertiaOffDiags: Vector3::new(0f32, 0f32, 0f32),
            UnscaledVolume: 0f32,
        };
        let superclass = MeshPart {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Message {
    superclass: Instance,
    pub Text: String,
}
impl_inherits!(Message, Instance);
impl_strong_instance_from!(Message);
impl Default for Message {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Message {
            superclass,
            Text: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MessageBusConnection {
    superclass: Instance,
}
impl_inherits!(MessageBusConnection, Instance);
impl_strong_instance_from!(MessageBusConnection);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MessageBusService {
    superclass: Instance,
}
impl_inherits!(MessageBusService, Instance);
impl_strong_instance_from!(MessageBusService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MessagingService {
    superclass: Instance,
}
impl_inherits!(MessagingService, Instance);
impl_strong_instance_from!(MessagingService);
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
impl_strong_instance_from!(MetaBreakpoint);
impl Default for MetaBreakpoint {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = MetaBreakpoint {
            superclass,
            Condition: "".to_owned(),
            ContinueExecution: false,
            Enabled: true,
            Line: 0i32,
            LogMessage: "".to_owned(),
            RemoveOnHit: false,
            Script: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MetaBreakpointContext {
    superclass: Instance,
    pub ContextDataInternal: String,
}
impl_inherits!(MetaBreakpointContext, Instance);
impl_strong_instance_from!(MetaBreakpointContext);
impl Default for MetaBreakpointContext {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = MetaBreakpointContext {
            superclass,
            ContextDataInternal: "0 1 2 ".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MetaBreakpointManager {
    superclass: Instance,
}
impl_inherits!(MetaBreakpointManager, Instance);
impl_strong_instance_from!(MetaBreakpointManager);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MicroProfilerService {
    superclass: Instance,
    pub ContextLabel: String,
}
impl_inherits!(MicroProfilerService, Instance);
impl_strong_instance_from!(MicroProfilerService);
impl Default for MicroProfilerService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = MicroProfilerService {
            superclass,
            ContextLabel: "".to_owned(),
        };
        superclass
    }
}
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
    pub SlimHash: SharedString,
    pub WorldPivotData: Option<CFrame>,
}
impl_inherits!(Model, PVInstance);
impl_strong_instance_from!(Model);
impl Default for Model {
    fn default() -> Self {
        let superclass = Object {};
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
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ModerationService {
    superclass: Instance,
}
impl_inherits!(ModerationService, Instance);
impl_strong_instance_from!(ModerationService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ModuleScript {
    superclass: LuaSourceContainer,
    pub LinkedSource: ContentId,
    pub Source: String,
}
impl_inherits!(ModuleScript, LuaSourceContainer);
impl_strong_instance_from!(ModuleScript);
impl Default for ModuleScript {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = ModuleScript {
            superclass,
            LinkedSource: "".into(),
            Source: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Motor {
    superclass: JointInstance,
    pub DesiredAngle: f32,
    pub MaxVelocity: f32,
}
impl_inherits!(Motor, JointInstance);
impl_strong_instance_from!(Motor);
impl Default for Motor {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = Motor {
            superclass,
            DesiredAngle: 0f32,
            MaxVelocity: 0f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Motor6D {
    superclass: Motor,
}
impl_inherits!(Motor6D, Motor);
impl_strong_instance_from!(Motor6D);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MotorFeature {
    superclass: Feature,
}
impl_inherits!(MotorFeature, Feature);
impl_strong_instance_from!(MotorFeature);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Mouse {
    superclass: Instance,
    pub IconContent: Content,
    pub TargetFilter: Ref,
}
impl_inherits!(Mouse, Instance);
impl_strong_instance_from!(Mouse);
impl Default for Mouse {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Mouse {
            superclass,
            IconContent: Content::none(),
            TargetFilter: Ref::none(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MouseService {
    superclass: Instance,
}
impl_inherits!(MouseService, Instance);
impl_strong_instance_from!(MouseService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct MultipleDocumentInterfaceInstance {
    superclass: Instance,
}
impl_inherits!(MultipleDocumentInterfaceInstance, Instance);
impl_strong_instance_from!(MultipleDocumentInterfaceInstance);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NegateOperation {
    superclass: PartOperation,
    pub PreviousOperation: enums::NegateOperationHiddenHistory,
}
impl_inherits!(NegateOperation, PartOperation);
impl_strong_instance_from!(NegateOperation);
impl Default for NegateOperation {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = TriangleMeshPart {
            superclass,
            AeroMeshData: SharedString::new(b"".to_vec()),
            FluidFidelityInternal: enums::FluidFidelity::Automatic,
            InertiaMigrated: false,
            PhysicalConfigData: SharedString::new(b"".to_vec()),
            UnscaledCofm: Vector3::new(0f32, 0f32, 0f32),
            UnscaledVolInertiaDiags: Vector3::new(0f32, 0f32, 0f32),
            UnscaledVolInertiaOffDiags: Vector3::new(0f32, 0f32, 0f32),
            UnscaledVolume: 0f32,
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
        let superclass = NegateOperation {
            superclass,
            PreviousOperation: enums::NegateOperationHiddenHistory::None,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct NetworkClient {
    superclass: NetworkPeer,
}
impl_inherits!(NetworkClient, NetworkPeer);
impl_strong_instance_from!(NetworkClient);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct NetworkMarker {
    superclass: Instance,
}
impl_inherits!(NetworkMarker, Instance);
impl_strong_instance_from!(NetworkMarker);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct NetworkPeer {
    superclass: Instance,
}
impl_inherits!(NetworkPeer, Instance);
impl_strong_instance_from!(NetworkPeer);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct NetworkReplicator {
    superclass: Instance,
}
impl_inherits!(NetworkReplicator, Instance);
impl_strong_instance_from!(NetworkReplicator);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct NetworkServer {
    superclass: NetworkPeer,
}
impl_inherits!(NetworkServer, NetworkPeer);
impl_strong_instance_from!(NetworkServer);
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
impl_strong_instance_from!(NetworkSettings);
impl Default for NetworkSettings {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = NetworkSettings {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NoCollisionConstraint {
    superclass: Instance,
    pub Enabled: bool,
    pub Part0: Ref,
    pub Part1: Ref,
}
impl_inherits!(NoCollisionConstraint, Instance);
impl_strong_instance_from!(NoCollisionConstraint);
impl Default for NoCollisionConstraint {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = NoCollisionConstraint {
            superclass,
            Enabled: true,
            Part0: Ref::none(),
            Part1: Ref::none(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Noise {
    superclass: Instance,
    pub NoiseType: enums::NoiseType,
    pub Seed: i32,
}
impl_inherits!(Noise, Instance);
impl_strong_instance_from!(Noise);
impl Default for Noise {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Noise {
            superclass,
            NoiseType: enums::NoiseType::SimplexGabor,
            Seed: 0i32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct NonReplicatedCSGDictionaryService {
    superclass: FlyweightService,
}
impl_inherits!(NonReplicatedCSGDictionaryService, FlyweightService);
impl_strong_instance_from!(NonReplicatedCSGDictionaryService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct NotificationService {
    superclass: Instance,
}
impl_inherits!(NotificationService, Instance);
impl_strong_instance_from!(NotificationService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NumberPose {
    superclass: PoseBase,
    pub Value: f64,
}
impl_inherits!(NumberPose, PoseBase);
impl_strong_instance_from!(NumberPose);
impl Default for NumberPose {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PoseBase {
            superclass,
            EasingDirection: enums::PoseEasingDirection::In,
            EasingStyle: enums::PoseEasingStyle::Linear,
            Weight: 0f32,
        };
        let superclass = NumberPose {
            superclass,
            Value: 0f64,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NumberValue {
    superclass: ValueBase,
    pub Value: f64,
}
impl_inherits!(NumberValue, ValueBase);
impl_strong_instance_from!(NumberValue);
impl Default for NumberValue {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        let superclass = NumberValue {
            superclass,
            Value: 0f64,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Object {}
impl_strong_instance_from!(Object);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ObjectValue {
    superclass: ValueBase,
    pub Value: Ref,
}
impl_inherits!(ObjectValue, ValueBase);
impl_strong_instance_from!(ObjectValue);
impl Default for ObjectValue {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        let superclass = ObjectValue {
            superclass,
            Value: Ref::none(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct OmniRecommendationsService {
    superclass: Instance,
}
impl_inherits!(OmniRecommendationsService, Instance);
impl_strong_instance_from!(OmniRecommendationsService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct OpenCloudApiV1 {
    superclass: Instance,
}
impl_inherits!(OpenCloudApiV1, Instance);
impl_strong_instance_from!(OpenCloudApiV1);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct OpenCloudService {
    superclass: Instance,
}
impl_inherits!(OpenCloudService, Instance);
impl_strong_instance_from!(OpenCloudService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct OperationGraph {
    superclass: Instance,
}
impl_inherits!(OperationGraph, Instance);
impl_strong_instance_from!(OperationGraph);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct OrderedDataStore {
    superclass: GlobalDataStore,
}
impl_inherits!(OrderedDataStore, GlobalDataStore);
impl_strong_instance_from!(OrderedDataStore);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct OutfitPages {
    superclass: Pages,
}
impl_inherits!(OutfitPages, Pages);
impl_strong_instance_from!(OutfitPages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PVAdornment {
    superclass: GuiBase3d,
    pub Adornee: Ref,
}
impl_inherits!(PVAdornment, GuiBase3d);
impl_strong_instance_from!(PVAdornment);
impl Default for PVAdornment {
    fn default() -> Self {
        let superclass = Object {};
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
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PVInstance {
    superclass: Instance,
}
impl_inherits!(PVInstance, Instance);
impl_strong_instance_from!(PVInstance);
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
impl_strong_instance_from!(PackageLink);
impl Default for PackageLink {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PackageLink {
            superclass,
            AutoUpdate: false,
            DefaultName: "".to_owned(),
            ModifiedState: 0i32,
            SerializedDefaultAttributes: b"".as_slice().into(),
            VersionIdSerialize: 0i64,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PackageService {
    superclass: Instance,
}
impl_inherits!(PackageService, Instance);
impl_strong_instance_from!(PackageService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PackageUIService {
    superclass: Instance,
}
impl_inherits!(PackageUIService, Instance);
impl_strong_instance_from!(PackageUIService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Pages {
    superclass: Instance,
}
impl_inherits!(Pages, Instance);
impl_strong_instance_from!(Pages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Pants {
    superclass: Clothing,
    pub PantsTemplate: ContentId,
}
impl_inherits!(Pants, Clothing);
impl_strong_instance_from!(Pants);
impl Default for Pants {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = Clothing {
            superclass,
            Color3: Color3::new(0f32, 0f32, 0f32),
        };
        let superclass = Pants {
            superclass,
            PantsTemplate: "".into(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ParabolaAdornment {
    superclass: PVAdornment,
}
impl_inherits!(ParabolaAdornment, PVAdornment);
impl_strong_instance_from!(ParabolaAdornment);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Part {
    superclass: FormFactorPart,
}
impl_inherits!(Part, FormFactorPart);
impl_strong_instance_from!(Part);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PartAdornment {
    superclass: GuiBase3d,
    pub Adornee: Ref,
}
impl_inherits!(PartAdornment, GuiBase3d);
impl_strong_instance_from!(PartAdornment);
impl Default for PartAdornment {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = PartAdornment {
            superclass,
            Adornee: Ref::none(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PartOperation {
    superclass: TriangleMeshPart,
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
impl_strong_instance_from!(PartOperation);
impl Default for PartOperation {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = TriangleMeshPart {
            superclass,
            AeroMeshData: SharedString::new(b"".to_vec()),
            FluidFidelityInternal: enums::FluidFidelity::Automatic,
            InertiaMigrated: false,
            PhysicalConfigData: SharedString::new(b"".to_vec()),
            UnscaledCofm: Vector3::new(0f32, 0f32, 0f32),
            UnscaledVolInertiaDiags: Vector3::new(0f32, 0f32, 0f32),
            UnscaledVolInertiaOffDiags: Vector3::new(0f32, 0f32, 0f32),
            UnscaledVolume: 0f32,
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
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PartOperationAsset {
    superclass: Instance,
    pub ChildData: BinaryString,
    pub MeshData: BinaryString,
}
impl_inherits!(PartOperationAsset, Instance);
impl_strong_instance_from!(PartOperationAsset);
impl Default for PartOperationAsset {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PartOperationAsset {
            superclass,
            ChildData: b"".as_slice().into(),
            MeshData: b"".as_slice().into(),
        };
        superclass
    }
}
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
impl_strong_instance_from!(ParticleEmitter);
impl Default for ParticleEmitter {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ParticleEmitter {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PartyEmulatorService {
    superclass: Instance,
}
impl_inherits!(PartyEmulatorService, Instance);
impl_strong_instance_from!(PartyEmulatorService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PatchBundlerFileWatch {
    superclass: Instance,
}
impl_inherits!(PatchBundlerFileWatch, Instance);
impl_strong_instance_from!(PatchBundlerFileWatch);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PatchMapping {
    superclass: Instance,
    pub FlattenTree: bool,
    pub PatchId: String,
    pub TargetPath: String,
}
impl_inherits!(PatchMapping, Instance);
impl_strong_instance_from!(PatchMapping);
impl Default for PatchMapping {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PatchMapping {
            superclass,
            FlattenTree: false,
            PatchId: "".to_owned(),
            TargetPath: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Path {
    superclass: Instance,
}
impl_inherits!(Path, Instance);
impl_strong_instance_from!(Path);
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
impl_strong_instance_from!(Path2D);
impl Default for Path2D {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = Path2D {
            superclass,
            Closed: false,
            Color3: Color3::new(0f32, 0f32, 0f32),
            PropertiesSerialize: b"\0\0\0\0".as_slice().into(),
            Thickness: 1f32,
            Transparency: 0f32,
            Visible: true,
            ZIndex: 1i32,
        };
        superclass
    }
}
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
impl_strong_instance_from!(PathfindingLink);
impl Default for PathfindingLink {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PathfindingLink {
            superclass,
            Attachment0: Ref::none(),
            Attachment1: Ref::none(),
            IsBidirectional: true,
            Label: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PathfindingModifier {
    superclass: Instance,
    pub Label: String,
    pub PassThrough: bool,
}
impl_inherits!(PathfindingModifier, Instance);
impl_strong_instance_from!(PathfindingModifier);
impl Default for PathfindingModifier {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PathfindingModifier {
            superclass,
            Label: "".to_owned(),
            PassThrough: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PathfindingService {
    superclass: Instance,
}
impl_inherits!(PathfindingService, Instance);
impl_strong_instance_from!(PathfindingService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PausedState {
    superclass: Instance,
}
impl_inherits!(PausedState, Instance);
impl_strong_instance_from!(PausedState);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PausedStateBreakpoint {
    superclass: PausedState,
}
impl_inherits!(PausedStateBreakpoint, PausedState);
impl_strong_instance_from!(PausedStateBreakpoint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PausedStateException {
    superclass: PausedState,
}
impl_inherits!(PausedStateException, PausedState);
impl_strong_instance_from!(PausedStateException);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PerformanceControlService {
    superclass: Instance,
}
impl_inherits!(PerformanceControlService, Instance);
impl_strong_instance_from!(PerformanceControlService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PermissionsService {
    superclass: Instance,
}
impl_inherits!(PermissionsService, Instance);
impl_strong_instance_from!(PermissionsService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PhysicsService {
    superclass: Instance,
}
impl_inherits!(PhysicsService, Instance);
impl_strong_instance_from!(PhysicsService);
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
impl_strong_instance_from!(PhysicsSettings);
impl Default for PhysicsSettings {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PhysicsSettings {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PitchShiftSoundEffect {
    superclass: SoundEffect,
    pub Octave: f32,
}
impl_inherits!(PitchShiftSoundEffect, SoundEffect);
impl_strong_instance_from!(PitchShiftSoundEffect);
impl Default for PitchShiftSoundEffect {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundEffect {
            superclass,
            Enabled: false,
            Priority: 0i32,
        };
        let superclass = PitchShiftSoundEffect {
            superclass,
            Octave: 1.25f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlaceAssetIdsService {
    superclass: Instance,
}
impl_inherits!(PlaceAssetIdsService, Instance);
impl_strong_instance_from!(PlaceAssetIdsService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlaceStatsService {
    superclass: Instance,
}
impl_inherits!(PlaceStatsService, Instance);
impl_strong_instance_from!(PlaceStatsService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlacesService {
    superclass: Instance,
}
impl_inherits!(PlacesService, Instance);
impl_strong_instance_from!(PlacesService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Plane {
    superclass: PlaneConstraint,
}
impl_inherits!(Plane, PlaneConstraint);
impl_strong_instance_from!(Plane);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlaneConstraint {
    superclass: Constraint,
}
impl_inherits!(PlaneConstraint, Constraint);
impl_strong_instance_from!(PlaneConstraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Platform {
    superclass: Part,
}
impl_inherits!(Platform, Part);
impl_strong_instance_from!(Platform);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlatformCloudStorageService {
    superclass: Instance,
}
impl_inherits!(PlatformCloudStorageService, Instance);
impl_strong_instance_from!(PlatformCloudStorageService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlatformFriendsService {
    superclass: Instance,
}
impl_inherits!(PlatformFriendsService, Instance);
impl_strong_instance_from!(PlatformFriendsService);
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
impl_strong_instance_from!(Player);
impl Default for Player {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Player {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlayerData {
    superclass: Instance,
}
impl_inherits!(PlayerData, Instance);
impl_strong_instance_from!(PlayerData);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlayerDataRecord {
    superclass: Instance,
}
impl_inherits!(PlayerDataRecord, Instance);
impl_strong_instance_from!(PlayerDataRecord);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlayerDataRecordConfig {
    superclass: Instance,
}
impl_inherits!(PlayerDataRecordConfig, Instance);
impl_strong_instance_from!(PlayerDataRecordConfig);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerDataService {
    superclass: Instance,
    pub LoadFailureBehavior: enums::PlayerDataLoadFailureBehavior,
}
impl_inherits!(PlayerDataService, Instance);
impl_strong_instance_from!(PlayerDataService);
impl Default for PlayerDataService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PlayerDataService {
            superclass,
            LoadFailureBehavior: enums::PlayerDataLoadFailureBehavior::Failure,
        };
        superclass
    }
}
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
impl_strong_instance_from!(PlayerEmulatorService);
impl Default for PlayerEmulatorService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PlayerEmulatorService {
            superclass,
            CustomPoliciesEnabled: false,
            EmulatedCountryCode: "".to_owned(),
            EmulatedGameLocale: "".to_owned(),
            PlayerEmulationEnabled: false,
            PseudolocalizationEnabled: false,
            SerializedEmulatedPolicyInfo: b"".as_slice().into(),
            TextElongationFactor: 0i32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerGui {
    superclass: BasePlayerGui,
    pub ScreenOrientation: enums::ScreenOrientation,
    pub SelectionImageObject: Ref,
}
impl_inherits!(PlayerGui, BasePlayerGui);
impl_strong_instance_from!(PlayerGui);
impl Default for PlayerGui {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = PlayerGui {
            superclass,
            ScreenOrientation: enums::ScreenOrientation::LandscapeLeft,
            SelectionImageObject: Ref::none(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlayerHydrationService {
    superclass: Instance,
}
impl_inherits!(PlayerHydrationService, Instance);
impl_strong_instance_from!(PlayerHydrationService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlayerMouse {
    superclass: Mouse,
}
impl_inherits!(PlayerMouse, Mouse);
impl_strong_instance_from!(PlayerMouse);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlayerScripts {
    superclass: Instance,
}
impl_inherits!(PlayerScripts, Instance);
impl_strong_instance_from!(PlayerScripts);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PlayerViewService {
    superclass: Instance,
}
impl_inherits!(PlayerViewService, Instance);
impl_strong_instance_from!(PlayerViewService);
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
impl_strong_instance_from!(Players);
impl Default for Players {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Players {
            superclass,
            BanningEnabled: true,
            CharacterAutoLoads: true,
            RespawnTime: 5f32,
            UseStrafingAnimations: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Plugin {
    superclass: Instance,
    pub DisableUiDragDetectorDrags: bool,
    pub IsDebuggable: bool,
}
impl_inherits!(Plugin, Instance);
impl_strong_instance_from!(Plugin);
impl Default for Plugin {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Plugin {
            superclass,
            DisableUiDragDetectorDrags: false,
            IsDebuggable: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginAction {
    superclass: Instance,
}
impl_inherits!(PluginAction, Instance);
impl_strong_instance_from!(PluginAction);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginCapabilities {
    superclass: Instance,
    pub Manifest: String,
}
impl_inherits!(PluginCapabilities, Instance);
impl_strong_instance_from!(PluginCapabilities);
impl Default for PluginCapabilities {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PluginCapabilities { superclass , Manifest : "{\"Metadata\":{\"TargetDataModels\": [\"Edit\", \"Server\", \"Client\"]},\"Permissions\":{}}" . to_owned () } ;
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginDebugService {
    superclass: Instance,
}
impl_inherits!(PluginDebugService, Instance);
impl_strong_instance_from!(PluginDebugService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginDragEvent {
    superclass: Instance,
}
impl_inherits!(PluginDragEvent, Instance);
impl_strong_instance_from!(PluginDragEvent);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginGui {
    superclass: LayerCollector,
    pub Title: String,
}
impl_inherits!(PluginGui, LayerCollector);
impl_strong_instance_from!(PluginGui);
impl Default for PluginGui {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = PluginGui {
            superclass,
            Title: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginGuiService {
    superclass: Instance,
}
impl_inherits!(PluginGuiService, Instance);
impl_strong_instance_from!(PluginGuiService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginManagementService {
    superclass: Instance,
}
impl_inherits!(PluginManagementService, Instance);
impl_strong_instance_from!(PluginManagementService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginManager {
    superclass: Instance,
}
impl_inherits!(PluginManager, Instance);
impl_strong_instance_from!(PluginManager);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginManagerInterface {
    superclass: Instance,
}
impl_inherits!(PluginManagerInterface, Instance);
impl_strong_instance_from!(PluginManagerInterface);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginMenu {
    superclass: Instance,
}
impl_inherits!(PluginMenu, Instance);
impl_strong_instance_from!(PluginMenu);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginMouse {
    superclass: Mouse,
}
impl_inherits!(PluginMouse, Mouse);
impl_strong_instance_from!(PluginMouse);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginPolicyService {
    superclass: Instance,
}
impl_inherits!(PluginPolicyService, Instance);
impl_strong_instance_from!(PluginPolicyService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginToolbar {
    superclass: Instance,
}
impl_inherits!(PluginToolbar, Instance);
impl_strong_instance_from!(PluginToolbar);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PluginToolbarButton {
    superclass: Instance,
}
impl_inherits!(PluginToolbarButton, Instance);
impl_strong_instance_from!(PluginToolbarButton);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PointLight {
    superclass: Light,
    pub Range: f32,
}
impl_inherits!(PointLight, Light);
impl_strong_instance_from!(PointLight);
impl Default for PointLight {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Light {
            superclass,
            Brightness: 0f32,
            Color: Color3::new(0f32, 0f32, 0f32),
            Enabled: false,
            Shadows: false,
        };
        let superclass = PointLight {
            superclass,
            Range: 8f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PointsService {
    superclass: Instance,
}
impl_inherits!(PointsService, Instance);
impl_strong_instance_from!(PointsService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PolicyService {
    superclass: Instance,
    pub IsLuobuServer: enums::TriStateBoolean,
    pub LuobuWhitelisted: enums::TriStateBoolean,
}
impl_inherits!(PolicyService, Instance);
impl_strong_instance_from!(PolicyService);
impl Default for PolicyService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PolicyService {
            superclass,
            IsLuobuServer: enums::TriStateBoolean::Unknown,
            LuobuWhitelisted: enums::TriStateBoolean::Unknown,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Pose {
    superclass: PoseBase,
    pub CFrame: CFrame,
}
impl_inherits!(Pose, PoseBase);
impl_strong_instance_from!(Pose);
impl Default for Pose {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PoseBase {
            superclass,
            EasingDirection: enums::PoseEasingDirection::In,
            EasingStyle: enums::PoseEasingStyle::Linear,
            Weight: 0f32,
        };
        let superclass = Pose {
            superclass,
            CFrame: CFrame::identity(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PoseBase {
    superclass: Instance,
    pub EasingDirection: enums::PoseEasingDirection,
    pub EasingStyle: enums::PoseEasingStyle,
    pub Weight: f32,
}
impl_inherits!(PoseBase, Instance);
impl_strong_instance_from!(PoseBase);
impl Default for PoseBase {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PoseBase {
            superclass,
            EasingDirection: enums::PoseEasingDirection::In,
            EasingStyle: enums::PoseEasingStyle::Linear,
            Weight: 0f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PostEffect {
    superclass: Instance,
    pub Enabled: bool,
}
impl_inherits!(PostEffect, Instance);
impl_strong_instance_from!(PostEffect);
impl Default for PostEffect {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PostEffect {
            superclass,
            Enabled: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PrismaticConstraint {
    superclass: SlidingBallConstraint,
}
impl_inherits!(PrismaticConstraint, SlidingBallConstraint);
impl_strong_instance_from!(PrismaticConstraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ProcessInstancePhysicsService {
    superclass: Instance,
}
impl_inherits!(ProcessInstancePhysicsService, Instance);
impl_strong_instance_from!(ProcessInstancePhysicsService);
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
    pub MaxIndicatorDistance: f32,
    pub ObjectText: String,
    pub RequiresLineOfSight: bool,
    pub RootLocalizationTable: Ref,
    pub Style: enums::ProximityPromptStyle,
    pub UiOffset: Vector2,
}
impl_inherits!(ProximityPrompt, Instance);
impl_strong_instance_from!(ProximityPrompt);
impl Default for ProximityPrompt {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ProximityPrompt {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ProximityPromptService {
    superclass: Instance,
    pub Enabled: bool,
    pub MaxIndicatorsVisible: i32,
    pub MaxPromptsVisible: i32,
}
impl_inherits!(ProximityPromptService, Instance);
impl_strong_instance_from!(ProximityPromptService);
impl Default for ProximityPromptService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ProximityPromptService {
            superclass,
            Enabled: true,
            MaxIndicatorsVisible: 16i32,
            MaxPromptsVisible: 16i32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct PublishService {
    superclass: Instance,
}
impl_inherits!(PublishService, Instance);
impl_strong_instance_from!(PublishService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PyramidHandleAdornment {
    superclass: HandleAdornment,
    pub Height: f32,
    pub Shading: enums::AdornShading,
    pub Sides: i32,
    pub Size: f32,
}
impl_inherits!(PyramidHandleAdornment, HandleAdornment);
impl_strong_instance_from!(PyramidHandleAdornment);
impl Default for PyramidHandleAdornment {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = HandleAdornment {
            superclass,
            AdornCullingMode: enums::AdornCullingMode::Automatic,
            AlwaysOnTop: false,
            CFrame: CFrame::identity(),
            SizeRelativeOffset: Vector3::new(0f32, 0f32, 0f32),
            ZIndex: 0i32,
        };
        let superclass = PyramidHandleAdornment {
            superclass,
            Height: 2f32,
            Shading: enums::AdornShading::Default,
            Sides: 4i32,
            Size: 1f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct QWidgetPluginGui {
    superclass: PluginGui,
}
impl_inherits!(QWidgetPluginGui, PluginGui);
impl_strong_instance_from!(QWidgetPluginGui);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RTAnimationTracker {
    superclass: Instance,
}
impl_inherits!(RTAnimationTracker, Instance);
impl_strong_instance_from!(RTAnimationTracker);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RayValue {
    superclass: ValueBase,
    pub Value: Ray,
}
impl_inherits!(RayValue, ValueBase);
impl_strong_instance_from!(RayValue);
impl Default for RayValue {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        let superclass = RayValue {
            superclass,
            Value: Ray::new(
                Vector3::new(0f32, 0f32, 0f32),
                Vector3::new(0f32, 0f32, 0f32),
            ),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RbxAnalyticsService {
    superclass: Instance,
}
impl_inherits!(RbxAnalyticsService, Instance);
impl_strong_instance_from!(RbxAnalyticsService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RecommendationPages {
    superclass: Pages,
}
impl_inherits!(RecommendationPages, Pages);
impl_strong_instance_from!(RecommendationPages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RecommendationService {
    superclass: Instance,
}
impl_inherits!(RecommendationService, Instance);
impl_strong_instance_from!(RecommendationService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadata {
    superclass: Instance,
}
impl_inherits!(ReflectionMetadata, Instance);
impl_strong_instance_from!(ReflectionMetadata);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataCallbacks {
    superclass: Instance,
}
impl_inherits!(ReflectionMetadataCallbacks, Instance);
impl_strong_instance_from!(ReflectionMetadataCallbacks);
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
impl_strong_instance_from!(ReflectionMetadataClass);
impl Default for ReflectionMetadataClass {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ReflectionMetadataItem {
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
        };
        let superclass = ReflectionMetadataClass {
            superclass,
            ExplorerImageIndex: 0i32,
            ExplorerOrder: 2147483647i32,
            Insertable: true,
            PreferredParent: "".to_owned(),
            ServiceVisibility: enums::ServiceVisibility::Always,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataClasses {
    superclass: Instance,
}
impl_inherits!(ReflectionMetadataClasses, Instance);
impl_strong_instance_from!(ReflectionMetadataClasses);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataEnum {
    superclass: ReflectionMetadataItem,
}
impl_inherits!(ReflectionMetadataEnum, ReflectionMetadataItem);
impl_strong_instance_from!(ReflectionMetadataEnum);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataEnumItem {
    superclass: ReflectionMetadataItem,
}
impl_inherits!(ReflectionMetadataEnumItem, ReflectionMetadataItem);
impl_strong_instance_from!(ReflectionMetadataEnumItem);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataEnums {
    superclass: Instance,
}
impl_inherits!(ReflectionMetadataEnums, Instance);
impl_strong_instance_from!(ReflectionMetadataEnums);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataEvents {
    superclass: Instance,
}
impl_inherits!(ReflectionMetadataEvents, Instance);
impl_strong_instance_from!(ReflectionMetadataEvents);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataFunctions {
    superclass: Instance,
}
impl_inherits!(ReflectionMetadataFunctions, Instance);
impl_strong_instance_from!(ReflectionMetadataFunctions);
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
impl_strong_instance_from!(ReflectionMetadataItem);
impl Default for ReflectionMetadataItem {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ReflectionMetadataItem {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataMember {
    superclass: ReflectionMetadataItem,
}
impl_inherits!(ReflectionMetadataMember, ReflectionMetadataItem);
impl_strong_instance_from!(ReflectionMetadataMember);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataProperties {
    superclass: Instance,
}
impl_inherits!(ReflectionMetadataProperties, Instance);
impl_strong_instance_from!(ReflectionMetadataProperties);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionMetadataYieldFunctions {
    superclass: Instance,
}
impl_inherits!(ReflectionMetadataYieldFunctions, Instance);
impl_strong_instance_from!(ReflectionMetadataYieldFunctions);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReflectionService {
    superclass: Instance,
}
impl_inherits!(ReflectionService, Instance);
impl_strong_instance_from!(ReflectionService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RelativeGui {
    superclass: GuiObject,
}
impl_inherits!(RelativeGui, GuiObject);
impl_strong_instance_from!(RelativeGui);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RemoteCommandService {
    superclass: Instance,
}
impl_inherits!(RemoteCommandService, Instance);
impl_strong_instance_from!(RemoteCommandService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RemoteCursorService {
    superclass: Instance,
}
impl_inherits!(RemoteCursorService, Instance);
impl_strong_instance_from!(RemoteCursorService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RemoteDebuggerServer {
    superclass: Instance,
}
impl_inherits!(RemoteDebuggerServer, Instance);
impl_strong_instance_from!(RemoteDebuggerServer);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RemoteEvent {
    superclass: BaseRemoteEvent,
}
impl_inherits!(RemoteEvent, BaseRemoteEvent);
impl_strong_instance_from!(RemoteEvent);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RemoteFunction {
    superclass: Instance,
}
impl_inherits!(RemoteFunction, Instance);
impl_strong_instance_from!(RemoteFunction);
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
impl_strong_instance_from!(RenderSettings);
impl Default for RenderSettings {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = RenderSettings {
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
        };
        superclass
    }
}
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
impl_strong_instance_from!(RenderingTest);
impl Default for RenderingTest {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = RenderingTest {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReplicatedFirst {
    superclass: Instance,
}
impl_inherits!(ReplicatedFirst, Instance);
impl_strong_instance_from!(ReplicatedFirst);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ReplicatedStorage {
    superclass: Instance,
}
impl_inherits!(ReplicatedStorage, Instance);
impl_strong_instance_from!(ReplicatedStorage);
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
impl_strong_instance_from!(ReverbSoundEffect);
impl Default for ReverbSoundEffect {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundEffect {
            superclass,
            Enabled: false,
            Priority: 0i32,
        };
        let superclass = ReverbSoundEffect {
            superclass,
            DecayTime: 1.5f32,
            Density: 1f32,
            Diffusion: 1f32,
            DryLevel: -6f32,
            WetLevel: 0f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RibbonNotificationService {
    superclass: Instance,
}
impl_inherits!(RibbonNotificationService, Instance);
impl_strong_instance_from!(RibbonNotificationService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RigidConstraint {
    superclass: Constraint,
}
impl_inherits!(RigidConstraint, Constraint);
impl_strong_instance_from!(RigidConstraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RobloxPluginGuiService {
    superclass: Instance,
}
impl_inherits!(RobloxPluginGuiService, Instance);
impl_strong_instance_from!(RobloxPluginGuiService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RobloxReplicatedStorage {
    superclass: Instance,
}
impl_inherits!(RobloxReplicatedStorage, Instance);
impl_strong_instance_from!(RobloxReplicatedStorage);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RobloxSerializableInstance {
    superclass: Instance,
    pub Data: BinaryString,
}
impl_inherits!(RobloxSerializableInstance, Instance);
impl_strong_instance_from!(RobloxSerializableInstance);
impl Default for RobloxSerializableInstance {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = RobloxSerializableInstance {
            superclass,
            Data: b"".as_slice().into(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RobloxServerStorage {
    superclass: Instance,
}
impl_inherits!(RobloxServerStorage, Instance);
impl_strong_instance_from!(RobloxServerStorage);
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
impl_strong_instance_from!(RocketPropulsion);
impl Default for RocketPropulsion {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = BodyMover { superclass };
        let superclass = RocketPropulsion {
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
        };
        superclass
    }
}
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
impl_strong_instance_from!(RodConstraint);
impl Default for RodConstraint {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = RodConstraint {
            superclass,
            Length: 5f32,
            LimitAngle0: 90f32,
            LimitAngle1: 90f32,
            LimitsEnabled: false,
            Thickness: 0.1f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RomarkRbxAnalyticsService {
    superclass: Instance,
}
impl_inherits!(RomarkRbxAnalyticsService, Instance);
impl_strong_instance_from!(RomarkRbxAnalyticsService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RomarkService {
    superclass: Instance,
}
impl_inherits!(RomarkService, Instance);
impl_strong_instance_from!(RomarkService);
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
impl_strong_instance_from!(RootImportData);
impl Default for RootImportData {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = RootImportData {
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
        };
        superclass
    }
}
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
impl_strong_instance_from!(RopeConstraint);
impl Default for RopeConstraint {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = RopeConstraint {
            superclass,
            Length: 5f32,
            Restitution: 0f32,
            Thickness: 0.1f32,
            WinchEnabled: false,
            WinchForce: 10000f32,
            WinchResponsiveness: 45f32,
            WinchSpeed: 2f32,
            WinchTarget: 5f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Rotate {
    superclass: JointInstance,
}
impl_inherits!(Rotate, JointInstance);
impl_strong_instance_from!(Rotate);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RotateP {
    superclass: DynamicRotate,
}
impl_inherits!(RotateP, DynamicRotate);
impl_strong_instance_from!(RotateP);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RotateV {
    superclass: DynamicRotate,
}
impl_inherits!(RotateV, DynamicRotate);
impl_strong_instance_from!(RotateV);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RotationCurve {
    superclass: Instance,
    pub ValuesAndTimes: BinaryString,
}
impl_inherits!(RotationCurve, Instance);
impl_strong_instance_from!(RotationCurve);
impl Default for RotationCurve {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = RotationCurve {
            superclass,
            ValuesAndTimes: b"\x01\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0".as_slice().into(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RtMessagingService {
    superclass: Instance,
}
impl_inherits!(RtMessagingService, Instance);
impl_strong_instance_from!(RtMessagingService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RunService {
    superclass: Instance,
}
impl_inherits!(RunService, Instance);
impl_strong_instance_from!(RunService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RunningAverageItemDouble {
    superclass: StatsItem,
}
impl_inherits!(RunningAverageItemDouble, StatsItem);
impl_strong_instance_from!(RunningAverageItemDouble);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RunningAverageItemInt {
    superclass: StatsItem,
}
impl_inherits!(RunningAverageItemInt, StatsItem);
impl_strong_instance_from!(RunningAverageItemInt);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RunningAverageTimeIntervalItem {
    superclass: StatsItem,
}
impl_inherits!(RunningAverageTimeIntervalItem, StatsItem);
impl_strong_instance_from!(RunningAverageTimeIntervalItem);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RuntimeContentService {
    superclass: Instance,
}
impl_inherits!(RuntimeContentService, Instance);
impl_strong_instance_from!(RuntimeContentService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct RuntimeScriptService {
    superclass: Instance,
}
impl_inherits!(RuntimeScriptService, Instance);
impl_strong_instance_from!(RuntimeScriptService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SafetyService {
    superclass: Instance,
    pub IsCaptureModeForReport: bool,
}
impl_inherits!(SafetyService, Instance);
impl_strong_instance_from!(SafetyService);
impl Default for SafetyService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SafetyService {
            superclass,
            IsCaptureModeForReport: false,
        };
        superclass
    }
}
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
impl_strong_instance_from!(ScreenGui);
impl Default for ScreenGui {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = ScreenGui {
            superclass,
            ClipToDeviceSafeArea: true,
            DisplayOrder: 0i32,
            SafeAreaCompatibility: enums::SafeAreaCompatibility::FullscreenExtension,
            ScreenInsets: enums::ScreenInsets::CoreUISafeInsets,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScreenshotCapture {
    superclass: Capture,
}
impl_inherits!(ScreenshotCapture, Capture);
impl_strong_instance_from!(ScreenshotCapture);
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
impl_strong_instance_from!(ScreenshotHud);
impl Default for ScreenshotHud {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ScreenshotHud {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Script {
    superclass: BaseScript,
    pub Source: String,
}
impl_inherits!(Script, BaseScript);
impl_strong_instance_from!(Script);
impl Default for Script {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = BaseScript {
            superclass,
            Disabled: false,
            LinkedSource: "".into(),
            RunContext: enums::RunContext::Legacy,
        };
        let superclass = Script {
            superclass,
            Source: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptBuilder {
    superclass: Instance,
}
impl_inherits!(ScriptBuilder, Instance);
impl_strong_instance_from!(ScriptBuilder);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptChangeService {
    superclass: Instance,
}
impl_inherits!(ScriptChangeService, Instance);
impl_strong_instance_from!(ScriptChangeService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptCloneWatcher {
    superclass: Instance,
}
impl_inherits!(ScriptCloneWatcher, Instance);
impl_strong_instance_from!(ScriptCloneWatcher);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptCloneWatcherHelper {
    superclass: Instance,
}
impl_inherits!(ScriptCloneWatcherHelper, Instance);
impl_strong_instance_from!(ScriptCloneWatcherHelper);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptCommitService {
    superclass: Instance,
}
impl_inherits!(ScriptCommitService, Instance);
impl_strong_instance_from!(ScriptCommitService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptContext {
    superclass: Instance,
}
impl_inherits!(ScriptContext, Instance);
impl_strong_instance_from!(ScriptContext);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptDebugger {
    superclass: Instance,
    pub CoreScriptIdentifier: String,
    pub ScriptGuid: String,
}
impl_inherits!(ScriptDebugger, Instance);
impl_strong_instance_from!(ScriptDebugger);
impl Default for ScriptDebugger {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ScriptDebugger {
            superclass,
            CoreScriptIdentifier: "".to_owned(),
            ScriptGuid: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptDocument {
    superclass: Instance,
}
impl_inherits!(ScriptDocument, Instance);
impl_strong_instance_from!(ScriptDocument);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptEditorService {
    superclass: Instance,
}
impl_inherits!(ScriptEditorService, Instance);
impl_strong_instance_from!(ScriptEditorService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptProfilerService {
    superclass: Instance,
}
impl_inherits!(ScriptProfilerService, Instance);
impl_strong_instance_from!(ScriptProfilerService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptRegistrationService {
    superclass: Instance,
}
impl_inherits!(ScriptRegistrationService, Instance);
impl_strong_instance_from!(ScriptRegistrationService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptRuntime {
    superclass: Instance,
}
impl_inherits!(ScriptRuntime, Instance);
impl_strong_instance_from!(ScriptRuntime);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ScriptService {
    superclass: Instance,
}
impl_inherits!(ScriptService, Instance);
impl_strong_instance_from!(ScriptService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScrollingFrame {
    superclass: GuiObject,
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
impl_strong_instance_from!(ScrollingFrame);
impl Default for ScrollingFrame {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = ScrollingFrame {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Seat {
    superclass: Part,
    pub Disabled: bool,
}
impl_inherits!(Seat, Part);
impl_strong_instance_from!(Seat);
impl Default for Seat {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = FormFactorPart { superclass };
        let superclass = Part { superclass };
        let superclass = Seat {
            superclass,
            Disabled: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Selection {
    superclass: Instance,
}
impl_inherits!(Selection, Instance);
impl_strong_instance_from!(Selection);
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
impl_strong_instance_from!(SelectionBox);
impl Default for SelectionBox {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = InstanceAdornment {
            superclass,
            Adornee: Ref::none(),
        };
        let superclass = SelectionBox {
            superclass,
            LineThickness: 0.15f32,
            StudioSelectionBox: false,
            SurfaceColor3: Color3::new(0.050980397f32, 0.41176474f32, 0.6745098f32),
            SurfaceTransparency: 1f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SelectionHighlightManager {
    superclass: Instance,
}
impl_inherits!(SelectionHighlightManager, Instance);
impl_strong_instance_from!(SelectionHighlightManager);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionLasso {
    superclass: GuiBase3d,
    pub Humanoid: Ref,
}
impl_inherits!(SelectionLasso, GuiBase3d);
impl_strong_instance_from!(SelectionLasso);
impl Default for SelectionLasso {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = SelectionLasso {
            superclass,
            Humanoid: Ref::none(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionPartLasso {
    superclass: SelectionLasso,
    pub Part: Ref,
}
impl_inherits!(SelectionPartLasso, SelectionLasso);
impl_strong_instance_from!(SelectionPartLasso);
impl Default for SelectionPartLasso {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = SelectionLasso {
            superclass,
            Humanoid: Ref::none(),
        };
        let superclass = SelectionPartLasso {
            superclass,
            Part: Ref::none(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionPointLasso {
    superclass: SelectionLasso,
    pub Point: Vector3,
}
impl_inherits!(SelectionPointLasso, SelectionLasso);
impl_strong_instance_from!(SelectionPointLasso);
impl Default for SelectionPointLasso {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = SelectionLasso {
            superclass,
            Humanoid: Ref::none(),
        };
        let superclass = SelectionPointLasso {
            superclass,
            Point: Vector3::new(0f32, 0f32, 0f32),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionSphere {
    superclass: PVAdornment,
    pub SurfaceColor3: Color3,
    pub SurfaceTransparency: f32,
}
impl_inherits!(SelectionSphere, PVAdornment);
impl_strong_instance_from!(SelectionSphere);
impl Default for SelectionSphere {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = SelectionSphere {
            superclass,
            SurfaceColor3: Color3::new(0.050980397f32, 0.41176474f32, 0.6745098f32),
            SurfaceTransparency: 1f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SensorBase {
    superclass: Instance,
    pub UpdateType: enums::SensorUpdateType,
}
impl_inherits!(SensorBase, Instance);
impl_strong_instance_from!(SensorBase);
impl Default for SensorBase {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SensorBase {
            superclass,
            UpdateType: enums::SensorUpdateType::OnRead,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SerializationService {
    superclass: Instance,
}
impl_inherits!(SerializationService, Instance);
impl_strong_instance_from!(SerializationService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ServerReplicator {
    superclass: NetworkReplicator,
}
impl_inherits!(ServerReplicator, NetworkReplicator);
impl_strong_instance_from!(ServerReplicator);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ServerScriptService {
    superclass: Instance,
    pub LoadStringEnabled: bool,
}
impl_inherits!(ServerScriptService, Instance);
impl_strong_instance_from!(ServerScriptService);
impl Default for ServerScriptService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ServerScriptService {
            superclass,
            LoadStringEnabled: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ServerStorage {
    superclass: Instance,
}
impl_inherits!(ServerStorage, Instance);
impl_strong_instance_from!(ServerStorage);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ServiceProvider {
    superclass: Instance,
}
impl_inherits!(ServiceProvider, Instance);
impl_strong_instance_from!(ServiceProvider);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ServiceVisibilityService {
    superclass: Instance,
    pub HiddenServices: BinaryString,
    pub VisibleServices: BinaryString,
}
impl_inherits!(ServiceVisibilityService, Instance);
impl_strong_instance_from!(ServiceVisibilityService);
impl Default for ServiceVisibilityService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ServiceVisibilityService {
            superclass,
            HiddenServices: b"\0\0\0\0".as_slice().into(),
            VisibleServices: b"\0\0\0\0".as_slice().into(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SessionCheckService {
    superclass: Instance,
}
impl_inherits!(SessionCheckService, Instance);
impl_strong_instance_from!(SessionCheckService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SessionService {
    superclass: Instance,
}
impl_inherits!(SessionService, Instance);
impl_strong_instance_from!(SessionService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SharedTableRegistry {
    superclass: Instance,
}
impl_inherits!(SharedTableRegistry, Instance);
impl_strong_instance_from!(SharedTableRegistry);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Shirt {
    superclass: Clothing,
    pub ShirtTemplate: ContentId,
}
impl_inherits!(Shirt, Clothing);
impl_strong_instance_from!(Shirt);
impl Default for Shirt {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = Clothing {
            superclass,
            Color3: Color3::new(0f32, 0f32, 0f32),
        };
        let superclass = Shirt {
            superclass,
            ShirtTemplate: "".into(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ShirtGraphic {
    superclass: CharacterAppearance,
    pub Color3: Color3,
    pub Graphic: ContentId,
}
impl_inherits!(ShirtGraphic, CharacterAppearance);
impl_strong_instance_from!(ShirtGraphic);
impl Default for ShirtGraphic {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = ShirtGraphic {
            superclass,
            Color3: Color3::new(1f32, 1f32, 1f32),
            Graphic: "".into(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SkateboardController {
    superclass: Controller,
}
impl_inherits!(SkateboardController, Controller);
impl_strong_instance_from!(SkateboardController);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SkateboardPlatform {
    superclass: Part,
    pub Steer: i32,
    pub StickyWheels: bool,
    pub Throttle: i32,
}
impl_inherits!(SkateboardPlatform, Part);
impl_strong_instance_from!(SkateboardPlatform);
impl Default for SkateboardPlatform {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = FormFactorPart { superclass };
        let superclass = Part { superclass };
        let superclass = SkateboardPlatform {
            superclass,
            Steer: 0i32,
            StickyWheels: true,
            Throttle: 0i32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Skin {
    superclass: CharacterAppearance,
    pub SkinColor: BrickColor,
}
impl_inherits!(Skin, CharacterAppearance);
impl_strong_instance_from!(Skin);
impl Default for Skin {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = Skin {
            superclass,
            SkinColor: BrickColor::from_number(226u16).unwrap(),
        };
        superclass
    }
}
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
    pub SkyboxOrientation: Vector3,
    pub SkyboxRt: ContentId,
    pub SkyboxUp: ContentId,
    pub StarCount: i32,
    pub SunAngularSize: f32,
    pub SunTextureId: ContentId,
}
impl_inherits!(Sky, Instance);
impl_strong_instance_from!(Sky);
impl Default for Sky {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Sky {
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
        };
        superclass
    }
}
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
impl_strong_instance_from!(SlidingBallConstraint);
impl Default for SlidingBallConstraint {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = SlidingBallConstraint {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SlimContentProvider {
    superclass: CacheableContentProvider,
}
impl_inherits!(SlimContentProvider, CacheableContentProvider);
impl_strong_instance_from!(SlimContentProvider);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SlimService {
    superclass: Instance,
}
impl_inherits!(SlimService, Instance);
impl_strong_instance_from!(SlimService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Smoke {
    superclass: Instance,
    pub Color: Color3,
    pub Enabled: bool,
    pub TimeScale: f32,
}
impl_inherits!(Smoke, Instance);
impl_strong_instance_from!(Smoke);
impl Default for Smoke {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Smoke {
            superclass,
            Color: Color3::new(1f32, 1f32, 1f32),
            Enabled: true,
            TimeScale: 1f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SmoothVoxelsUpgraderService {
    superclass: Instance,
}
impl_inherits!(SmoothVoxelsUpgraderService, Instance);
impl_strong_instance_from!(SmoothVoxelsUpgraderService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Snap {
    superclass: JointInstance,
}
impl_inherits!(Snap, JointInstance);
impl_strong_instance_from!(Snap);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SnippetService {
    superclass: Instance,
}
impl_inherits!(SnippetService, Instance);
impl_strong_instance_from!(SnippetService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SocialService {
    superclass: Instance,
}
impl_inherits!(SocialService, Instance);
impl_strong_instance_from!(SocialService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SolidModelContentProvider {
    superclass: CacheableContentProvider,
}
impl_inherits!(SolidModelContentProvider, CacheableContentProvider);
impl_strong_instance_from!(SolidModelContentProvider);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Sound {
    superclass: Instance,
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
impl_strong_instance_from!(Sound);
impl Default for Sound {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Sound {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SoundEffect {
    superclass: Instance,
    pub Enabled: bool,
    pub Priority: i32,
}
impl_inherits!(SoundEffect, Instance);
impl_strong_instance_from!(SoundEffect);
impl Default for SoundEffect {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundEffect {
            superclass,
            Enabled: false,
            Priority: 0i32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SoundGroup {
    superclass: Instance,
    pub Volume: f32,
}
impl_inherits!(SoundGroup, Instance);
impl_strong_instance_from!(SoundGroup);
impl Default for SoundGroup {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundGroup {
            superclass,
            Volume: 0.5f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SoundService {
    superclass: Instance,
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
impl_strong_instance_from!(SoundService);
impl Default for SoundService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundService {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SoundShimService {
    superclass: Instance,
}
impl_inherits!(SoundShimService, Instance);
impl_strong_instance_from!(SoundShimService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Sparkles {
    superclass: Instance,
    pub Enabled: bool,
    pub SparkleColor: Color3,
    pub TimeScale: f32,
}
impl_inherits!(Sparkles, Instance);
impl_strong_instance_from!(Sparkles);
impl Default for Sparkles {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Sparkles {
            superclass,
            Enabled: true,
            SparkleColor: Color3::new(0.5647059f32, 0.098039225f32, 1f32),
            TimeScale: 1f32,
        };
        superclass
    }
}
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
impl_strong_instance_from!(SpawnLocation);
impl Default for SpawnLocation {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = FormFactorPart { superclass };
        let superclass = Part { superclass };
        let superclass = SpawnLocation {
            superclass,
            AllowTeamChangeOnTouch: false,
            Duration: 10i32,
            Enabled: true,
            Neutral: true,
            TeamColor: BrickColor::from_number(194u16).unwrap(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SpawnerService {
    superclass: Instance,
}
impl_inherits!(SpawnerService, Instance);
impl_strong_instance_from!(SpawnerService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SpecialMesh {
    superclass: FileMesh,
    pub MeshType: enums::MeshType,
}
impl_inherits!(SpecialMesh, FileMesh);
impl_strong_instance_from!(SpecialMesh);
impl Default for SpecialMesh {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = FileMesh {
            superclass,
            MeshId: "".into(),
            TextureId: "".into(),
        };
        let superclass = SpecialMesh {
            superclass,
            MeshType: enums::MeshType::Head,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SphereHandleAdornment {
    superclass: HandleAdornment,
    pub Radius: f32,
    pub Shading: enums::AdornShading,
}
impl_inherits!(SphereHandleAdornment, HandleAdornment);
impl_strong_instance_from!(SphereHandleAdornment);
impl Default for SphereHandleAdornment {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = HandleAdornment {
            superclass,
            AdornCullingMode: enums::AdornCullingMode::Automatic,
            AlwaysOnTop: false,
            CFrame: CFrame::identity(),
            SizeRelativeOffset: Vector3::new(0f32, 0f32, 0f32),
            ZIndex: 0i32,
        };
        let superclass = SphereHandleAdornment {
            superclass,
            Radius: 1f32,
            Shading: enums::AdornShading::Default,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SpotLight {
    superclass: Light,
    pub Angle: f32,
    pub Face: enums::NormalId,
    pub Range: f32,
}
impl_inherits!(SpotLight, Light);
impl_strong_instance_from!(SpotLight);
impl Default for SpotLight {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Light {
            superclass,
            Brightness: 0f32,
            Color: Color3::new(0f32, 0f32, 0f32),
            Enabled: false,
            Shadows: false,
        };
        let superclass = SpotLight {
            superclass,
            Angle: 90f32,
            Face: enums::NormalId::Front,
            Range: 16f32,
        };
        superclass
    }
}
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
impl_strong_instance_from!(SpringConstraint);
impl Default for SpringConstraint {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = SpringConstraint {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StackFrame {
    superclass: Instance,
}
impl_inherits!(StackFrame, Instance);
impl_strong_instance_from!(StackFrame);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StandalonePluginScripts {
    superclass: Instance,
}
impl_inherits!(StandalonePluginScripts, Instance);
impl_strong_instance_from!(StandalonePluginScripts);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StandardPages {
    superclass: Pages,
}
impl_inherits!(StandardPages, Pages);
impl_strong_instance_from!(StandardPages);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StartPageService {
    superclass: Instance,
}
impl_inherits!(StartPageService, Instance);
impl_strong_instance_from!(StartPageService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StarterCharacterScripts {
    superclass: StarterPlayerScripts,
}
impl_inherits!(StarterCharacterScripts, StarterPlayerScripts);
impl_strong_instance_from!(StarterCharacterScripts);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StarterGear {
    superclass: Instance,
}
impl_inherits!(StarterGear, Instance);
impl_strong_instance_from!(StarterGear);
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
impl_strong_instance_from!(StarterGui);
impl Default for StarterGui {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = StarterGui {
            superclass,
            ResetPlayerGuiOnSpawn: true,
            RtlTextSupport: enums::RtlTextSupport::Default,
            ScreenOrientation: enums::ScreenOrientation::LandscapeSensor,
            ShowDevelopmentGui: true,
            StudioDefaultStyleSheet: Ref::none(),
            StudioInsertWidgetLayerCollectorAutoLinkStyleSheet: Ref::none(),
            VirtualCursorMode: enums::VirtualCursorMode::Default,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StarterPack {
    superclass: Instance,
}
impl_inherits!(StarterPack, Instance);
impl_strong_instance_from!(StarterPack);
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
impl_strong_instance_from!(StarterPlayer);
impl Default for StarterPlayer {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = StarterPlayer {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StarterPlayerScripts {
    superclass: Instance,
}
impl_inherits!(StarterPlayerScripts, Instance);
impl_strong_instance_from!(StarterPlayerScripts);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StartupMessageService {
    superclass: Instance,
}
impl_inherits!(StartupMessageService, Instance);
impl_strong_instance_from!(StartupMessageService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Stats {
    superclass: Instance,
}
impl_inherits!(Stats, Instance);
impl_strong_instance_from!(Stats);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StatsItem {
    superclass: Instance,
}
impl_inherits!(StatsItem, Instance);
impl_strong_instance_from!(StatsItem);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Status {
    superclass: Model,
}
impl_inherits!(Status, Model);
impl_strong_instance_from!(Status);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StopWatchReporter {
    superclass: Instance,
}
impl_inherits!(StopWatchReporter, Instance);
impl_strong_instance_from!(StopWatchReporter);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StreamingService {
    superclass: Instance,
}
impl_inherits!(StreamingService, Instance);
impl_strong_instance_from!(StreamingService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StringValue {
    superclass: ValueBase,
    pub Value: String,
}
impl_inherits!(StringValue, ValueBase);
impl_strong_instance_from!(StringValue);
impl Default for StringValue {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        let superclass = StringValue {
            superclass,
            Value: "".to_owned(),
        };
        superclass
    }
}
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
impl_strong_instance_from!(Studio);
impl Default for Studio {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Studio {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioAssetService {
    superclass: Instance,
}
impl_inherits!(StudioAssetService, Instance);
impl_strong_instance_from!(StudioAssetService);
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
impl_strong_instance_from!(StudioAttachment);
impl Default for StudioAttachment {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = StudioAttachment {
            superclass,
            AutoHideParent: false,
            IsArrowVisible: false,
            Offset: Vector2::new(0f32, 0f32),
            SourceAnchorPoint: Vector2::new(0f32, 0f32),
            TargetAnchorPoint: Vector2::new(0f32, 0f32),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioCallout {
    superclass: Instance,
}
impl_inherits!(StudioCallout, Instance);
impl_strong_instance_from!(StudioCallout);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioCameraService {
    superclass: Instance,
    pub LockCameraSpeed: bool,
    pub LoggingEnabled: bool,
}
impl_inherits!(StudioCameraService, Instance);
impl_strong_instance_from!(StudioCameraService);
impl Default for StudioCameraService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = StudioCameraService {
            superclass,
            LockCameraSpeed: false,
            LoggingEnabled: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioData {
    superclass: Instance,
    pub EnableScriptCollabByDefaultOnLoad: bool,
}
impl_inherits!(StudioData, Instance);
impl_strong_instance_from!(StudioData);
impl Default for StudioData {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = StudioData {
            superclass,
            EnableScriptCollabByDefaultOnLoad: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioDeviceEmulatorService {
    superclass: Instance,
}
impl_inherits!(StudioDeviceEmulatorService, Instance);
impl_strong_instance_from!(StudioDeviceEmulatorService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioObjectBase {
    superclass: Instance,
}
impl_inherits!(StudioObjectBase, Instance);
impl_strong_instance_from!(StudioObjectBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioPublishService {
    superclass: Instance,
    pub PublishLocked: bool,
}
impl_inherits!(StudioPublishService, Instance);
impl_strong_instance_from!(StudioPublishService);
impl Default for StudioPublishService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = StudioPublishService {
            superclass,
            PublishLocked: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioScriptDebugEventListener {
    superclass: Instance,
}
impl_inherits!(StudioScriptDebugEventListener, Instance);
impl_strong_instance_from!(StudioScriptDebugEventListener);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioSdkService {
    superclass: Instance,
}
impl_inherits!(StudioSdkService, Instance);
impl_strong_instance_from!(StudioSdkService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioService {
    superclass: Instance,
    pub Secrets: String,
}
impl_inherits!(StudioService, Instance);
impl_strong_instance_from!(StudioService);
impl Default for StudioService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = StudioService {
            superclass,
            Secrets: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioTestService {
    superclass: Instance,
}
impl_inherits!(StudioTestService, Instance);
impl_strong_instance_from!(StudioTestService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioTheme {
    superclass: Instance,
}
impl_inherits!(StudioTheme, Instance);
impl_strong_instance_from!(StudioTheme);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioUserService {
    superclass: Instance,
}
impl_inherits!(StudioUserService, Instance);
impl_strong_instance_from!(StudioUserService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioWidget {
    superclass: StudioObjectBase,
}
impl_inherits!(StudioWidget, StudioObjectBase);
impl_strong_instance_from!(StudioWidget);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StudioWidgetsService {
    superclass: Instance,
}
impl_inherits!(StudioWidgetsService, Instance);
impl_strong_instance_from!(StudioWidgetsService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StyleBase {
    superclass: Instance,
}
impl_inherits!(StyleBase, Instance);
impl_strong_instance_from!(StyleBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StyleDerive {
    superclass: Instance,
    pub Priority: i32,
    pub StyleSheet: Ref,
}
impl_inherits!(StyleDerive, Instance);
impl_strong_instance_from!(StyleDerive);
impl Default for StyleDerive {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = StyleDerive {
            superclass,
            Priority: 0i32,
            StyleSheet: Ref::none(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StyleLink {
    superclass: Instance,
    pub StyleSheet: Ref,
}
impl_inherits!(StyleLink, Instance);
impl_strong_instance_from!(StyleLink);
impl Default for StyleLink {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = StyleLink {
            superclass,
            StyleSheet: Ref::none(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StyleQuery {
    superclass: Instance,
    pub AspectRatioRange: NumberRange,
    pub ConditionsSerialize: BinaryString,
    pub MaxSize: Vector2,
    pub MinSize: Vector2,
}
impl_inherits!(StyleQuery, Instance);
impl_strong_instance_from!(StyleQuery);
impl Default for StyleQuery {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = StyleQuery {
            superclass,
            AspectRatioRange: NumberRange::new(0f32, 0f32),
            ConditionsSerialize: b"".as_slice().into(),
            MaxSize: Vector2::new(0f32, 0f32),
            MinSize: Vector2::new(0f32, 0f32),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StyleRule {
    superclass: StyleBase,
    pub Priority: i32,
    pub Selector: String,
}
impl_inherits!(StyleRule, StyleBase);
impl_strong_instance_from!(StyleRule);
impl Default for StyleRule {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = StyleBase { superclass };
        let superclass = StyleRule {
            superclass,
            Priority: 0i32,
            Selector: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StyleSheet {
    superclass: StyleBase,
}
impl_inherits!(StyleSheet, StyleBase);
impl_strong_instance_from!(StyleSheet);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct StylingService {
    superclass: Instance,
}
impl_inherits!(StylingService, Instance);
impl_strong_instance_from!(StylingService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SunRaysEffect {
    superclass: PostEffect,
    pub Intensity: f32,
    pub Spread: f32,
}
impl_inherits!(SunRaysEffect, PostEffect);
impl_strong_instance_from!(SunRaysEffect);
impl Default for SunRaysEffect {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = PostEffect {
            superclass,
            Enabled: false,
        };
        let superclass = SunRaysEffect {
            superclass,
            Intensity: 0.25f32,
            Spread: 1f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SurfaceAppearance {
    superclass: Instance,
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
impl_strong_instance_from!(SurfaceAppearance);
impl Default for SurfaceAppearance {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SurfaceAppearance {
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
        };
        superclass
    }
}
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
impl_strong_instance_from!(SurfaceGui);
impl Default for SurfaceGui {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = SurfaceGui {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SurfaceGuiBase {
    superclass: LayerCollector,
    pub Active: bool,
    pub Adornee: Ref,
    pub Face: enums::NormalId,
}
impl_inherits!(SurfaceGuiBase, LayerCollector);
impl_strong_instance_from!(SurfaceGuiBase);
impl Default for SurfaceGuiBase {
    fn default() -> Self {
        let superclass = Object {};
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
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SurfaceLight {
    superclass: Light,
    pub Angle: f32,
    pub Face: enums::NormalId,
    pub Range: f32,
}
impl_inherits!(SurfaceLight, Light);
impl_strong_instance_from!(SurfaceLight);
impl Default for SurfaceLight {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Light {
            superclass,
            Brightness: 0f32,
            Color: Color3::new(0f32, 0f32, 0f32),
            Enabled: false,
            Shadows: false,
        };
        let superclass = SurfaceLight {
            superclass,
            Angle: 90f32,
            Face: enums::NormalId::Front,
            Range: 16f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SurfaceSelection {
    superclass: PartAdornment,
    pub TargetSurface: enums::NormalId,
}
impl_inherits!(SurfaceSelection, PartAdornment);
impl_strong_instance_from!(SurfaceSelection);
impl Default for SurfaceSelection {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = PartAdornment {
            superclass,
            Adornee: Ref::none(),
        };
        let superclass = SurfaceSelection {
            superclass,
            TargetSurface: enums::NormalId::Right,
        };
        superclass
    }
}
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
impl_strong_instance_from!(SwimController);
impl Default for SwimController {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ControllerBase {
            superclass,
            BalanceRigidityEnabled: false,
            MoveSpeedFactor: 0f32,
        };
        let superclass = SwimController {
            superclass,
            AccelerationTime: 0f32,
            PitchMaxTorque: 10000f32,
            PitchSpeedFactor: 1f32,
            RollMaxTorque: 10000f32,
            RollSpeedFactor: 1f32,
        };
        superclass
    }
}
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
impl_strong_instance_from!(SyncScriptBuilder);
impl Default for SyncScriptBuilder {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = SyncScriptBuilder {
            superclass,
            CompileTarget: enums::CompileTarget::Client,
            CoverageInfo: false,
            DebugInfo: false,
            PackAsSource: false,
            RawBytecode: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct SystemThemeService {
    superclass: Instance,
}
impl_inherits!(SystemThemeService, Instance);
impl_strong_instance_from!(SystemThemeService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TaskScheduler {
    superclass: Instance,
    pub ThreadPoolConfig: enums::ThreadPoolConfig,
}
impl_inherits!(TaskScheduler, Instance);
impl_strong_instance_from!(TaskScheduler);
impl Default for TaskScheduler {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = TaskScheduler {
            superclass,
            ThreadPoolConfig: enums::ThreadPoolConfig::Auto,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Team {
    superclass: Instance,
    pub AutoAssignable: bool,
    pub TeamColor: BrickColor,
}
impl_inherits!(Team, Instance);
impl_strong_instance_from!(Team);
impl Default for Team {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Team {
            superclass,
            AutoAssignable: true,
            TeamColor: BrickColor::from_number(1u16).unwrap(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TeamCreateData {
    superclass: Instance,
}
impl_inherits!(TeamCreateData, Instance);
impl_strong_instance_from!(TeamCreateData);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TeamCreatePublishService {
    superclass: Instance,
}
impl_inherits!(TeamCreatePublishService, Instance);
impl_strong_instance_from!(TeamCreatePublishService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TeamCreateService {
    superclass: Instance,
}
impl_inherits!(TeamCreateService, Instance);
impl_strong_instance_from!(TeamCreateService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Teams {
    superclass: Instance,
}
impl_inherits!(Teams, Instance);
impl_strong_instance_from!(Teams);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TelemetryService {
    superclass: Instance,
}
impl_inherits!(TelemetryService, Instance);
impl_strong_instance_from!(TelemetryService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TeleportAsyncResult {
    superclass: Instance,
}
impl_inherits!(TeleportAsyncResult, Instance);
impl_strong_instance_from!(TeleportAsyncResult);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TeleportOptions {
    superclass: Instance,
    pub ReservedServerAccessCode: String,
    pub ServerInstanceId: String,
    pub ShouldReserveServer: bool,
}
impl_inherits!(TeleportOptions, Instance);
impl_strong_instance_from!(TeleportOptions);
impl Default for TeleportOptions {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = TeleportOptions {
            superclass,
            ReservedServerAccessCode: "".to_owned(),
            ServerInstanceId: "".to_owned(),
            ShouldReserveServer: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TeleportService {
    superclass: Instance,
}
impl_inherits!(TeleportService, Instance);
impl_strong_instance_from!(TeleportService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TemporaryCageMeshProvider {
    superclass: Instance,
}
impl_inherits!(TemporaryCageMeshProvider, Instance);
impl_strong_instance_from!(TemporaryCageMeshProvider);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TemporaryScriptService {
    superclass: Instance,
}
impl_inherits!(TemporaryScriptService, Instance);
impl_strong_instance_from!(TemporaryScriptService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Terrain {
    superclass: BasePart,
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
impl_strong_instance_from!(Terrain);
impl Default for Terrain {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = Terrain {
            superclass,
            AcquisitionMethod: enums::TerrainAcquisitionMethod::None,
            Decoration: false,
            GrassLength: 0.7f32,
            MaterialColors: unimplemented!("MaterialColors"),
            PhysicsGrid: b"\x02\x03\0\0\0\0\0\0\0\0\0\0\0\0".as_slice().into(),
            SmoothGrid: b"\x01\x05".as_slice().into(),
            SmoothVoxelsUpgraded: false,
            WaterColor: Color3::new(0.05f32, 0.33f32, 0.36f32),
            WaterReflectance: 1f32,
            WaterTransparency: 0.3f32,
            WaterWaveSize: 0.15f32,
            WaterWaveSpeed: 10f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TerrainDetail {
    superclass: Instance,
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
impl_strong_instance_from!(TerrainDetail);
impl Default for TerrainDetail {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = TerrainDetail {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TerrainIterateOperation {
    superclass: Object,
}
impl_inherits!(TerrainIterateOperation, Object);
impl_strong_instance_from!(TerrainIterateOperation);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TerrainModifyOperation {
    superclass: Object,
}
impl_inherits!(TerrainModifyOperation, Object);
impl_strong_instance_from!(TerrainModifyOperation);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TerrainReadOperation {
    superclass: Object,
}
impl_inherits!(TerrainReadOperation, Object);
impl_strong_instance_from!(TerrainReadOperation);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TerrainRegion {
    superclass: Instance,
    pub ExtentsMax: Vector3int16,
    pub ExtentsMin: Vector3int16,
    pub SmoothGrid: BinaryString,
}
impl_inherits!(TerrainRegion, Instance);
impl_strong_instance_from!(TerrainRegion);
impl Default for TerrainRegion {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = TerrainRegion {
            superclass,
            ExtentsMax: Vector3int16::new(0i16, 0i16, 0i16),
            ExtentsMin: Vector3int16::new(0i16, 0i16, 0i16),
            SmoothGrid: b"\x01\x05".as_slice().into(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TerrainWriteOperation {
    superclass: Object,
}
impl_inherits!(TerrainWriteOperation, Object);
impl_strong_instance_from!(TerrainWriteOperation);
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
impl_strong_instance_from!(TestService);
impl Default for TestService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = TestService {
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
        };
        superclass
    }
}
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
impl_strong_instance_from!(TextBox);
impl Default for TextBox {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = TextBox {
            superclass,
            ClearTextOnFocus: true,
            FontFace: unimplemented!("Font"),
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextBoxService {
    superclass: Instance,
}
impl_inherits!(TextBoxService, Instance);
impl_strong_instance_from!(TextBoxService);
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
impl_strong_instance_from!(TextButton);
impl Default for TextButton {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = GuiButton {
            superclass,
            AutoButtonColor: false,
            HoverHapticEffect: Ref::none(),
            Modal: false,
            PressHapticEffect: Ref::none(),
            Selected: false,
            Style: enums::ButtonStyle::Custom,
        };
        let superclass = TextButton {
            superclass,
            FontFace: unimplemented!("Font"),
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextChannel {
    superclass: Instance,
}
impl_inherits!(TextChannel, Instance);
impl_strong_instance_from!(TextChannel);
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
impl_strong_instance_from!(TextChatCommand);
impl Default for TextChatCommand {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = TextChatCommand {
            superclass,
            AutocompleteVisible: true,
            Enabled: true,
            PrimaryAlias: "".to_owned(),
            SecondaryAlias: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextChatConfigurations {
    superclass: Instance,
}
impl_inherits!(TextChatConfigurations, Instance);
impl_strong_instance_from!(TextChatConfigurations);
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
impl_strong_instance_from!(TextChatMessage);
impl Default for TextChatMessage {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = TextChatMessage {
            superclass,
            BubbleChatMessageProperties: Ref::none(),
            ChatWindowMessageProperties: Ref::none(),
            TextChannel: Ref::none(),
            TextSource: Ref::none(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextChatMessageProperties {
    superclass: Instance,
}
impl_inherits!(TextChatMessageProperties, Instance);
impl_strong_instance_from!(TextChatMessageProperties);
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
    pub IsLegacyChatDisabled: bool,
}
impl_inherits!(TextChatService, Instance);
impl_strong_instance_from!(TextChatService);
impl Default for TextChatService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = TextChatService {
            superclass,
            ChatTranslationFtuxShown: true,
            ChatTranslationToggleEnabled: false,
            ChatVersion: enums::ChatVersion::LegacyChatService,
            CreateDefaultCommands: true,
            CreateDefaultTextChannels: true,
            HasSeenDeprecationDialog: false,
            IsLegacyChatDisabled: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextFilterResult {
    superclass: Instance,
}
impl_inherits!(TextFilterResult, Instance);
impl_strong_instance_from!(TextFilterResult);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextFilterTranslatedResult {
    superclass: Instance,
}
impl_inherits!(TextFilterTranslatedResult, Instance);
impl_strong_instance_from!(TextFilterTranslatedResult);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextGenerator {
    superclass: Instance,
    pub Seed: i32,
    pub SystemPrompt: String,
    pub Temperature: f32,
    pub TopP: f32,
}
impl_inherits!(TextGenerator, Instance);
impl_strong_instance_from!(TextGenerator);
impl Default for TextGenerator {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = TextGenerator {
            superclass,
            Seed: 0i32,
            SystemPrompt: "".to_owned(),
            Temperature: 0.7f32,
            TopP: 0.9f32,
        };
        superclass
    }
}
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
impl_strong_instance_from!(TextLabel);
impl Default for TextLabel {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = GuiLabel { superclass };
        let superclass = TextLabel {
            superclass,
            FontFace: unimplemented!("Font"),
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextService {
    superclass: Instance,
}
impl_inherits!(TextService, Instance);
impl_strong_instance_from!(TextService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextSource {
    superclass: Instance,
    pub CanSend: bool,
}
impl_inherits!(TextSource, Instance);
impl_strong_instance_from!(TextSource);
impl Default for TextSource {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = TextSource {
            superclass,
            CanSend: false,
        };
        superclass
    }
}
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
impl_strong_instance_from!(Texture);
impl Default for Texture {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = FaceInstance {
            superclass,
            Face: enums::NormalId::Right,
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
        let superclass = Texture {
            superclass,
            OffsetStudsU: 0f32,
            OffsetStudsV: 0f32,
            StudsPerTileU: 2f32,
            StudsPerTileV: 2f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextureGenerationPartGroup {
    superclass: Instance,
}
impl_inherits!(TextureGenerationPartGroup, Instance);
impl_strong_instance_from!(TextureGenerationPartGroup);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextureGenerationService {
    superclass: Instance,
}
impl_inherits!(TextureGenerationService, Instance);
impl_strong_instance_from!(TextureGenerationService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TextureGenerationUnwrappingRequest {
    superclass: Instance,
}
impl_inherits!(TextureGenerationUnwrappingRequest, Instance);
impl_strong_instance_from!(TextureGenerationUnwrappingRequest);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ThirdPartyUserService {
    superclass: Instance,
}
impl_inherits!(ThirdPartyUserService, Instance);
impl_strong_instance_from!(ThirdPartyUserService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ThreadState {
    superclass: Instance,
}
impl_inherits!(ThreadState, Instance);
impl_strong_instance_from!(ThreadState);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TimerService {
    superclass: Instance,
}
impl_inherits!(TimerService, Instance);
impl_strong_instance_from!(TimerService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ToastNotificationService {
    superclass: Instance,
}
impl_inherits!(ToastNotificationService, Instance);
impl_strong_instance_from!(ToastNotificationService);
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
impl_strong_instance_from!(Tool);
impl Default for Tool {
    fn default() -> Self {
        let superclass = Object {};
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
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Torque {
    superclass: Constraint,
    pub RelativeTo: enums::ActuatorRelativeTo,
    pub Torque: Vector3,
}
impl_inherits!(Torque, Constraint);
impl_strong_instance_from!(Torque);
impl Default for Torque {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = Torque {
            superclass,
            RelativeTo: enums::ActuatorRelativeTo::Attachment0,
            Torque: Vector3::new(0f32, 0f32, 0f32),
        };
        superclass
    }
}
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
impl_strong_instance_from!(TorsionSpringConstraint);
impl Default for TorsionSpringConstraint {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = TorsionSpringConstraint {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TotalCountTimeIntervalItem {
    superclass: StatsItem,
}
impl_inherits!(TotalCountTimeIntervalItem, StatsItem);
impl_strong_instance_from!(TotalCountTimeIntervalItem);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TouchInputService {
    superclass: Instance,
}
impl_inherits!(TouchInputService, Instance);
impl_strong_instance_from!(TouchInputService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TouchTransmitter {
    superclass: Instance,
}
impl_inherits!(TouchTransmitter, Instance);
impl_strong_instance_from!(TouchTransmitter);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TracerService {
    superclass: Instance,
}
impl_inherits!(TracerService, Instance);
impl_strong_instance_from!(TracerService);
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
impl_strong_instance_from!(TrackerLodController);
impl Default for TrackerLodController {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = TrackerLodController {
            superclass,
            AudioMode: enums::TrackerLodFlagMode::ForceFalse,
            VideoExtrapolationMode: enums::TrackerExtrapolationFlagMode::ForceDisabled,
            VideoLodMode: enums::TrackerLodValueMode::Force0,
            VideoMode: enums::TrackerLodFlagMode::ForceFalse,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TrackerStreamAnimation {
    superclass: Instance,
}
impl_inherits!(TrackerStreamAnimation, Instance);
impl_strong_instance_from!(TrackerStreamAnimation);
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
impl_strong_instance_from!(Trail);
impl Default for Trail {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Trail {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Translator {
    superclass: Instance,
}
impl_inherits!(Translator, Instance);
impl_strong_instance_from!(Translator);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TremoloSoundEffect {
    superclass: SoundEffect,
    pub Depth: f32,
    pub Duty: f32,
    pub Frequency: f32,
}
impl_inherits!(TremoloSoundEffect, SoundEffect);
impl_strong_instance_from!(TremoloSoundEffect);
impl Default for TremoloSoundEffect {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = SoundEffect {
            superclass,
            Enabled: false,
            Priority: 0i32,
        };
        let superclass = TremoloSoundEffect {
            superclass,
            Depth: 1f32,
            Duty: 0.5f32,
            Frequency: 5f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TriangleMeshPart {
    superclass: BasePart,
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
impl_strong_instance_from!(TriangleMeshPart);
impl Default for TriangleMeshPart {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = TriangleMeshPart {
            superclass,
            AeroMeshData: SharedString::new(b"".to_vec()),
            FluidFidelityInternal: enums::FluidFidelity::Automatic,
            InertiaMigrated: false,
            PhysicalConfigData: SharedString::new(b"".to_vec()),
            UnscaledCofm: Vector3::new(0f32, 0f32, 0f32),
            UnscaledVolInertiaDiags: Vector3::new(0f32, 0f32, 0f32),
            UnscaledVolInertiaOffDiags: Vector3::new(0f32, 0f32, 0f32),
            UnscaledVolume: 0f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TrussPart {
    superclass: BasePart,
    pub Style: enums::Style,
}
impl_inherits!(TrussPart, BasePart);
impl_strong_instance_from!(TrussPart);
impl Default for TrussPart {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = TrussPart {
            superclass,
            Style: enums::Style::AlternatingSupports,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TutorialService {
    superclass: Instance,
}
impl_inherits!(TutorialService, Instance);
impl_strong_instance_from!(TutorialService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Tween {
    superclass: TweenBase,
}
impl_inherits!(Tween, TweenBase);
impl_strong_instance_from!(Tween);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TweenBase {
    superclass: Instance,
}
impl_inherits!(TweenBase, Instance);
impl_strong_instance_from!(TweenBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct TweenService {
    superclass: Instance,
}
impl_inherits!(TweenService, Instance);
impl_strong_instance_from!(TweenService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UGCAvatarService {
    superclass: Instance,
}
impl_inherits!(UGCAvatarService, Instance);
impl_strong_instance_from!(UGCAvatarService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UGCValidationService {
    superclass: Instance,
}
impl_inherits!(UGCValidationService, Instance);
impl_strong_instance_from!(UGCValidationService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIAspectRatioConstraint {
    superclass: UIConstraint,
    pub AspectRatio: f32,
    pub AspectType: enums::AspectType,
    pub DominantAxis: enums::DominantAxis,
}
impl_inherits!(UIAspectRatioConstraint, UIConstraint);
impl_strong_instance_from!(UIAspectRatioConstraint);
impl Default for UIAspectRatioConstraint {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = UIConstraint { superclass };
        let superclass = UIAspectRatioConstraint {
            superclass,
            AspectRatio: 1f32,
            AspectType: enums::AspectType::FitWithinMaxSize,
            DominantAxis: enums::DominantAxis::Width,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UIBase {
    superclass: Instance,
}
impl_inherits!(UIBase, Instance);
impl_strong_instance_from!(UIBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UIComponent {
    superclass: UIBase,
}
impl_inherits!(UIComponent, UIBase);
impl_strong_instance_from!(UIComponent);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UIConstraint {
    superclass: UIComponent,
}
impl_inherits!(UIConstraint, UIComponent);
impl_strong_instance_from!(UIConstraint);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UICorner {
    superclass: UIComponent,
    pub CornerRadius: UDim,
}
impl_inherits!(UICorner, UIComponent);
impl_strong_instance_from!(UICorner);
impl Default for UICorner {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = UICorner {
            superclass,
            CornerRadius: UDim::new(0f32, 8i32),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIDragDetector {
    superclass: UIComponent,
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
impl_strong_instance_from!(UIDragDetector);
impl Default for UIDragDetector {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = UIDragDetector {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UIDragDetectorService {
    superclass: Instance,
}
impl_inherits!(UIDragDetectorService, Instance);
impl_strong_instance_from!(UIDragDetectorService);
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
impl_strong_instance_from!(UIFlexItem);
impl Default for UIFlexItem {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = UIFlexItem {
            superclass,
            FlexMode: enums::UIFlexMode::None,
            GrowRatio: 0f32,
            ItemLineAlignment: enums::ItemLineAlignment::Automatic,
            ShrinkRatio: 0f32,
        };
        superclass
    }
}
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
impl_strong_instance_from!(UIGradient);
impl Default for UIGradient {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = UIGradient {
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
        };
        superclass
    }
}
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
impl_strong_instance_from!(UIGridLayout);
impl Default for UIGridLayout {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = UIGridStyleLayout {
            superclass,
            FillDirection: enums::FillDirection::Horizontal,
            HorizontalAlignment: enums::HorizontalAlignment::Center,
            SortOrder: enums::SortOrder::Name,
            VerticalAlignment: enums::VerticalAlignment::Center,
        };
        let superclass = UIGridLayout {
            superclass,
            CellPadding: UDim2::new(UDim::new(0f32, 5i32), UDim::new(0f32, 5i32)),
            CellSize: UDim2::new(UDim::new(0f32, 100i32), UDim::new(0f32, 100i32)),
            FillDirectionMaxCells: 0i32,
            StartCorner: enums::StartCorner::TopLeft,
        };
        superclass
    }
}
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
impl_strong_instance_from!(UIGridStyleLayout);
impl Default for UIGridStyleLayout {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = UIGridStyleLayout {
            superclass,
            FillDirection: enums::FillDirection::Horizontal,
            HorizontalAlignment: enums::HorizontalAlignment::Center,
            SortOrder: enums::SortOrder::Name,
            VerticalAlignment: enums::VerticalAlignment::Center,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UILayout {
    superclass: UIComponent,
}
impl_inherits!(UILayout, UIComponent);
impl_strong_instance_from!(UILayout);
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
impl_strong_instance_from!(UIListLayout);
impl Default for UIListLayout {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = UIGridStyleLayout {
            superclass,
            FillDirection: enums::FillDirection::Horizontal,
            HorizontalAlignment: enums::HorizontalAlignment::Center,
            SortOrder: enums::SortOrder::Name,
            VerticalAlignment: enums::VerticalAlignment::Center,
        };
        let superclass = UIListLayout {
            superclass,
            HorizontalFlex: enums::UIFlexAlignment::None,
            ItemLineAlignment: enums::ItemLineAlignment::Automatic,
            Padding: UDim::new(0f32, 0i32),
            VerticalFlex: enums::UIFlexAlignment::None,
            Wraps: false,
        };
        superclass
    }
}
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
impl_strong_instance_from!(UIPadding);
impl Default for UIPadding {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = UIPadding {
            superclass,
            PaddingBottom: UDim::new(0f32, 0i32),
            PaddingLeft: UDim::new(0f32, 0i32),
            PaddingRight: UDim::new(0f32, 0i32),
            PaddingTop: UDim::new(0f32, 0i32),
        };
        superclass
    }
}
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
impl_strong_instance_from!(UIPageLayout);
impl Default for UIPageLayout {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = UIGridStyleLayout {
            superclass,
            FillDirection: enums::FillDirection::Horizontal,
            HorizontalAlignment: enums::HorizontalAlignment::Center,
            SortOrder: enums::SortOrder::Name,
            VerticalAlignment: enums::VerticalAlignment::Center,
        };
        let superclass = UIPageLayout {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIScale {
    superclass: UIComponent,
    pub Scale: f32,
}
impl_inherits!(UIScale, UIComponent);
impl_strong_instance_from!(UIScale);
impl Default for UIScale {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = UIScale {
            superclass,
            Scale: 1f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UISizeConstraint {
    superclass: UIConstraint,
    pub MaxSize: Vector2,
    pub MinSize: Vector2,
}
impl_inherits!(UISizeConstraint, UIConstraint);
impl_strong_instance_from!(UISizeConstraint);
impl Default for UISizeConstraint {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = UIConstraint { superclass };
        let superclass = UISizeConstraint {
            superclass,
            MaxSize: Vector2::new(f32::INFINITY, f32::INFINITY),
            MinSize: Vector2::new(0f32, 0f32),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIStroke {
    superclass: UIComponent,
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
impl_strong_instance_from!(UIStroke);
impl Default for UIStroke {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = UIStroke {
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
        };
        superclass
    }
}
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
impl_strong_instance_from!(UITableLayout);
impl Default for UITableLayout {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = UIGridStyleLayout {
            superclass,
            FillDirection: enums::FillDirection::Horizontal,
            HorizontalAlignment: enums::HorizontalAlignment::Center,
            SortOrder: enums::SortOrder::Name,
            VerticalAlignment: enums::VerticalAlignment::Center,
        };
        let superclass = UITableLayout {
            superclass,
            FillEmptySpaceColumns: false,
            FillEmptySpaceRows: false,
            MajorAxis: enums::TableMajorAxis::RowMajor,
            Padding: UDim2::new(UDim::new(0f32, 0i32), UDim::new(0f32, 0i32)),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UITextSizeConstraint {
    superclass: UIConstraint,
    pub MaxTextSize: i32,
    pub MinTextSize: i32,
}
impl_inherits!(UITextSizeConstraint, UIConstraint);
impl_strong_instance_from!(UITextSizeConstraint);
impl Default for UITextSizeConstraint {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = UIConstraint { superclass };
        let superclass = UITextSizeConstraint {
            superclass,
            MaxTextSize: 100i32,
            MinTextSize: 1i32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UnionOperation {
    superclass: PartOperation,
}
impl_inherits!(UnionOperation, PartOperation);
impl_strong_instance_from!(UnionOperation);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UniqueIdLookupService {
    superclass: Instance,
}
impl_inherits!(UniqueIdLookupService, Instance);
impl_strong_instance_from!(UniqueIdLookupService);
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
impl_strong_instance_from!(UniversalConstraint);
impl Default for UniversalConstraint {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = UniversalConstraint {
            superclass,
            LimitsEnabled: false,
            MaxAngle: 45f32,
            Radius: 0.2f32,
            Restitution: 0f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UnreliableRemoteEvent {
    superclass: BaseRemoteEvent,
}
impl_inherits!(UnreliableRemoteEvent, BaseRemoteEvent);
impl_strong_instance_from!(UnreliableRemoteEvent);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UnvalidatedAssetService {
    superclass: Instance,
    pub CachedData: String,
}
impl_inherits!(UnvalidatedAssetService, Instance);
impl_strong_instance_from!(UnvalidatedAssetService);
impl Default for UnvalidatedAssetService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = UnvalidatedAssetService {
            superclass,
            CachedData: "{\"lastSaveTime\":0,\"lastKnownPublishRequest\":0,\"users\":[]}"
                .to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UserGameSettings {
    superclass: Instance,
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
impl_strong_instance_from!(UserGameSettings);
impl Default for UserGameSettings {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = UserGameSettings {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UserInputService {
    superclass: Instance,
    pub LegacyInputEventsEnabled: bool,
    pub MouseBehavior: enums::MouseBehavior,
    pub MouseIconContent: Content,
    pub MouseIconEnabled: bool,
}
impl_inherits!(UserInputService, Instance);
impl_strong_instance_from!(UserInputService);
impl Default for UserInputService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = UserInputService {
            superclass,
            LegacyInputEventsEnabled: false,
            MouseBehavior: enums::MouseBehavior::Default,
            MouseIconContent: Content::none(),
            MouseIconEnabled: false,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UserService {
    superclass: Instance,
}
impl_inherits!(UserService, Instance);
impl_strong_instance_from!(UserService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UserSettings {
    superclass: GenericSettings,
}
impl_inherits!(UserSettings, GenericSettings);
impl_strong_instance_from!(UserSettings);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct UserStorageService {
    superclass: LocalStorageService,
}
impl_inherits!(UserStorageService, LocalStorageService);
impl_strong_instance_from!(UserStorageService);
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
impl_strong_instance_from!(VRService);
impl Default for VRService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = VRService {
            superclass,
            AutomaticScaling: enums::VRScaling::World,
            AvatarGestures: false,
            ControllerModels: enums::VRControllerModelMode::Transparent,
            FadeOutViewOnCollision: true,
            LaserPointer: enums::VRLaserPointerMode::Pointer,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VRStatusService {
    superclass: Instance,
}
impl_inherits!(VRStatusService, Instance);
impl_strong_instance_from!(VRStatusService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct ValueBase {
    superclass: Instance,
}
impl_inherits!(ValueBase, Instance);
impl_strong_instance_from!(ValueBase);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ValueCurve {
    superclass: Instance,
    pub ValuesAndTimes: BinaryString,
}
impl_inherits!(ValueCurve, Instance);
impl_strong_instance_from!(ValueCurve);
impl Default for ValueCurve {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueCurve {
            superclass,
            ValuesAndTimes: b"\x02\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0".as_slice().into(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Vector3Curve {
    superclass: Instance,
}
impl_inherits!(Vector3Curve, Instance);
impl_strong_instance_from!(Vector3Curve);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Vector3Value {
    superclass: ValueBase,
    pub Value: Vector3,
}
impl_inherits!(Vector3Value, ValueBase);
impl_strong_instance_from!(Vector3Value);
impl Default for Vector3Value {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = ValueBase { superclass };
        let superclass = Vector3Value {
            superclass,
            Value: Vector3::new(0f32, 0f32, 0f32),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VectorForce {
    superclass: Constraint,
    pub ApplyAtCenterOfMass: bool,
    pub Force: Vector3,
    pub RelativeTo: enums::ActuatorRelativeTo,
}
impl_inherits!(VectorForce, Constraint);
impl_strong_instance_from!(VectorForce);
impl Default for VectorForce {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = VectorForce {
            superclass,
            ApplyAtCenterOfMass: false,
            Force: Vector3::new(1000f32, 0f32, 0f32),
            RelativeTo: enums::ActuatorRelativeTo::Attachment0,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VehicleController {
    superclass: Controller,
}
impl_inherits!(VehicleController, Controller);
impl_strong_instance_from!(VehicleController);
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
impl_strong_instance_from!(VehicleSeat);
impl Default for VehicleSeat {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = VehicleSeat {
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
        };
        superclass
    }
}
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
impl_strong_instance_from!(VelocityMotor);
impl Default for VelocityMotor {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = VelocityMotor {
            superclass,
            CurrentAngle: 0f32,
            DesiredAngle: 0f32,
            Hole: Ref::none(),
            MaxVelocity: 0f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VersionControlService {
    superclass: Instance,
}
impl_inherits!(VersionControlService, Instance);
impl_strong_instance_from!(VersionControlService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VideoCapture {
    superclass: Capture,
}
impl_inherits!(VideoCapture, Capture);
impl_strong_instance_from!(VideoCapture);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VideoCaptureService {
    superclass: Instance,
}
impl_inherits!(VideoCaptureService, Instance);
impl_strong_instance_from!(VideoCaptureService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VideoDeviceInput {
    superclass: Instance,
    pub Active: bool,
    pub CameraId: String,
    pub CaptureQuality: enums::VideoDeviceCaptureQuality,
}
impl_inherits!(VideoDeviceInput, Instance);
impl_strong_instance_from!(VideoDeviceInput);
impl Default for VideoDeviceInput {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = VideoDeviceInput {
            superclass,
            Active: false,
            CameraId: "".to_owned(),
            CaptureQuality: enums::VideoDeviceCaptureQuality::Default,
        };
        superclass
    }
}
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
impl_strong_instance_from!(VideoDisplay);
impl Default for VideoDisplay {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = VideoDisplay {
            superclass,
            ResampleMode: enums::ResamplerMode::Default,
            ScaleType: enums::ScaleType::Stretch,
            TileSize: UDim2::new(UDim::new(1f32, 0i32), UDim::new(1f32, 0i32)),
            VideoColor3: Color3::new(1f32, 1f32, 1f32),
            VideoRectOffset: Vector2::new(0f32, 0f32),
            VideoRectSize: Vector2::new(0f32, 0f32),
            VideoTransparency: 0f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VideoFrame {
    superclass: GuiObject,
    pub Looped: bool,
    pub Playing: bool,
    pub TimePosition: f64,
    pub VideoContent: Content,
    pub Volume: f32,
}
impl_inherits!(VideoFrame, GuiObject);
impl_strong_instance_from!(VideoFrame);
impl Default for VideoFrame {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = VideoFrame {
            superclass,
            Looped: false,
            Playing: false,
            TimePosition: 0f64,
            VideoContent: Content::none(),
            Volume: 1f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VideoPlayer {
    superclass: Instance,
    pub Looping: bool,
    pub PlaybackSpeed: f32,
    pub TimePosition: f64,
    pub VideoContent: Content,
    pub Volume: f32,
}
impl_inherits!(VideoPlayer, Instance);
impl_strong_instance_from!(VideoPlayer);
impl Default for VideoPlayer {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = VideoPlayer {
            superclass,
            Looping: false,
            PlaybackSpeed: 1f32,
            TimePosition: 0f64,
            VideoContent: Content::none(),
            Volume: 1f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VideoSampler {
    superclass: Object,
}
impl_inherits!(VideoSampler, Object);
impl_strong_instance_from!(VideoSampler);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VideoScreenCaptureService {
    superclass: Instance,
}
impl_inherits!(VideoScreenCaptureService, Instance);
impl_strong_instance_from!(VideoScreenCaptureService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VideoService {
    superclass: Instance,
}
impl_inherits!(VideoService, Instance);
impl_strong_instance_from!(VideoService);
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
impl_strong_instance_from!(ViewportFrame);
impl Default for ViewportFrame {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = ViewportFrame {
            superclass,
            Ambient: Color3::new(0.78431374f32, 0.78431374f32, 0.78431374f32),
            CameraCFrame: CFrame::identity(),
            CameraFieldOfView: 1.2217306f32,
            ImageColor3: Color3::new(1f32, 1f32, 1f32),
            ImageTransparency: 0f32,
            LightColor: Color3::new(0.54901963f32, 0.54901963f32, 0.54901963f32),
            LightDirection: Vector3::new(-1f32, -1f32, -1f32),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VirtualInputManager {
    superclass: Instance,
}
impl_inherits!(VirtualInputManager, Instance);
impl_strong_instance_from!(VirtualInputManager);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VirtualUser {
    superclass: Instance,
}
impl_inherits!(VirtualUser, Instance);
impl_strong_instance_from!(VirtualUser);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VisibilityCheckDispatcher {
    superclass: Instance,
}
impl_inherits!(VisibilityCheckDispatcher, Instance);
impl_strong_instance_from!(VisibilityCheckDispatcher);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Visit {
    superclass: Instance,
}
impl_inherits!(Visit, Instance);
impl_strong_instance_from!(Visit);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VisualizationMode {
    superclass: Instance,
    pub Enabled: bool,
    pub Title: String,
    pub ToolTip: String,
}
impl_inherits!(VisualizationMode, Instance);
impl_strong_instance_from!(VisualizationMode);
impl Default for VisualizationMode {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = VisualizationMode {
            superclass,
            Enabled: false,
            Title: "".to_owned(),
            ToolTip: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VisualizationModeCategory {
    superclass: Instance,
    pub Enabled: bool,
    pub Title: String,
}
impl_inherits!(VisualizationModeCategory, Instance);
impl_strong_instance_from!(VisualizationModeCategory);
impl Default for VisualizationModeCategory {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = VisualizationModeCategory {
            superclass,
            Enabled: false,
            Title: "".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VisualizationModeService {
    superclass: Instance,
}
impl_inherits!(VisualizationModeService, Instance);
impl_strong_instance_from!(VisualizationModeService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct VoiceChatInternal {
    superclass: Instance,
}
impl_inherits!(VoiceChatInternal, Instance);
impl_strong_instance_from!(VoiceChatInternal);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VoiceChatService {
    superclass: Instance,
    pub DefaultDistanceAttenuation: enums::VoiceChatDistanceAttenuationType,
    pub EnableDefaultVoice: bool,
    pub UseAudioApi: enums::AudioApiRollout,
}
impl_inherits!(VoiceChatService, Instance);
impl_strong_instance_from!(VoiceChatService);
impl Default for VoiceChatService {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = VoiceChatService {
            superclass,
            DefaultDistanceAttenuation: enums::VoiceChatDistanceAttenuationType::Inverse,
            EnableDefaultVoice: true,
            UseAudioApi: enums::AudioApiRollout::Automatic,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WebSocketClient {
    superclass: Instance,
}
impl_inherits!(WebSocketClient, Instance);
impl_strong_instance_from!(WebSocketClient);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WebSocketService {
    superclass: Instance,
}
impl_inherits!(WebSocketService, Instance);
impl_strong_instance_from!(WebSocketService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WebStreamClient {
    superclass: Object,
}
impl_inherits!(WebStreamClient, Object);
impl_strong_instance_from!(WebStreamClient);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WebViewService {
    superclass: Instance,
}
impl_inherits!(WebViewService, Instance);
impl_strong_instance_from!(WebViewService);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WedgePart {
    superclass: FormFactorPart,
}
impl_inherits!(WedgePart, FormFactorPart);
impl_strong_instance_from!(WedgePart);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct Weld {
    superclass: JointInstance,
}
impl_inherits!(Weld, JointInstance);
impl_strong_instance_from!(Weld);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WeldConstraint {
    superclass: Instance,
    pub CFrame0: CFrame,
    pub State: i32,
}
impl_inherits!(WeldConstraint, Instance);
impl_strong_instance_from!(WeldConstraint);
impl Default for WeldConstraint {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = WeldConstraint {
            superclass,
            CFrame0: CFrame::identity(),
            State: 3i32,
        };
        superclass
    }
}
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
impl_strong_instance_from!(Wire);
impl Default for Wire {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = Wire {
            superclass,
            SourceInstance: Ref::none(),
            SourceName: "Output".to_owned(),
            TargetInstance: Ref::none(),
            TargetName: "Input".to_owned(),
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WireframeHandleAdornment {
    superclass: HandleAdornment,
    pub Scale: Vector3,
    pub Thickness: f32,
}
impl_inherits!(WireframeHandleAdornment, HandleAdornment);
impl_strong_instance_from!(WireframeHandleAdornment);
impl Default for WireframeHandleAdornment {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = HandleAdornment {
            superclass,
            AdornCullingMode: enums::AdornCullingMode::Automatic,
            AlwaysOnTop: false,
            CFrame: CFrame::identity(),
            SizeRelativeOffset: Vector3::new(0f32, 0f32, 0f32),
            ZIndex: 0i32,
        };
        let superclass = WireframeHandleAdornment {
            superclass,
            Scale: Vector3::new(1f32, 1f32, 1f32),
            Thickness: 1f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Workspace {
    superclass: WorldRoot,
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
impl_strong_instance_from!(Workspace);
impl Default for Workspace {
    fn default() -> Self {
        let superclass = Object {};
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
        let superclass = WorldRoot { superclass };
        let superclass = Workspace {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WorkspaceAnnotation {
    superclass: Annotation,
}
impl_inherits!(WorkspaceAnnotation, Annotation);
impl_strong_instance_from!(WorkspaceAnnotation);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WorldModel {
    superclass: WorldRoot,
}
impl_inherits!(WorldModel, WorldRoot);
impl_strong_instance_from!(WorldModel);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WorldRoot {
    superclass: Model,
}
impl_inherits!(WorldRoot, Model);
impl_strong_instance_from!(WorldRoot);
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
#[derive(Default)]
pub struct WrapDeformer {
    superclass: BaseWrap,
}
impl_inherits!(WrapDeformer, BaseWrap);
impl_strong_instance_from!(WrapDeformer);
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
impl_strong_instance_from!(WrapLayer);
impl Default for WrapLayer {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
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
        let superclass = WrapLayer {
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
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WrapTarget {
    superclass: BaseWrap,
    pub Stiffness: f32,
}
impl_inherits!(WrapTarget, BaseWrap);
impl_strong_instance_from!(WrapTarget);
impl Default for WrapTarget {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
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
        let superclass = WrapTarget {
            superclass,
            Stiffness: 0f32,
        };
        superclass
    }
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WrapTextureTransfer {
    superclass: Instance,
    pub ReferenceCageMeshContent: Content,
    pub UvMaxBound: Vector2,
    pub UvMinBound: Vector2,
}
impl_inherits!(WrapTextureTransfer, Instance);
impl_strong_instance_from!(WrapTextureTransfer);
impl Default for WrapTextureTransfer {
    fn default() -> Self {
        let superclass = Object {};
        let superclass = Instance {
            superclass,
            Capabilities: SecurityCapabilities::from_bits(0u64),
            HistoryId: UniqueId::nil(),
            Name: "".to_owned(),
            SourceAssetId: 0i64,
            Tags: Tags::new(),
            UniqueId: UniqueId::nil(),
        };
        let superclass = WrapTextureTransfer {
            superclass,
            ReferenceCageMeshContent: Content::none(),
            UvMaxBound: Vector2::new(f32::NEG_INFINITY, f32::NEG_INFINITY),
            UvMinBound: Vector2::new(f32::INFINITY, f32::INFINITY),
        };
        superclass
    }
}
