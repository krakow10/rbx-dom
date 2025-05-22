use super::enums;
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
    pub AccessoryType: enums::AccessoryType,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AccessoryDescription {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AccountService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Accoutrement {
    pub AttachmentPoint: CFrame,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AchievementService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ActivityHistoryEventService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Actor {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AdGui {
    pub AdShape: enums::AdShape,
    pub EnableVideoAds: bool,
    pub FallbackImage: ContentId,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AdPortal {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AdService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AdvancedDragger {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AirController {
    pub BalanceMaxTorque: f32,
    pub BalanceSpeed: f32,
    pub MaintainAngularMomentum: bool,
    pub MaintainLinearMomentum: bool,
    pub MoveMaxForce: f32,
    pub TurnMaxTorque: f32,
    pub TurnSpeedFactor: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AlignOrientation {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AlignPosition {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnalysticsSettings {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnalyticsService {
    pub ApiKey: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AngularVelocity {
    pub AngularVelocity: Vector3,
    pub MaxTorque: f32,
    pub ReactionTorqueEnabled: bool,
    pub RelativeTo: enums::ActuatorRelativeTo,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Animation {
    pub AnimationId: ContentId,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationClip {
    pub GuidBinaryString: BinaryString,
    pub Loop: bool,
    pub Priority: enums::AnimationPriority,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationClipProvider {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationConstraint {
    pub IsKinematic: bool,
    pub MaxForce: f32,
    pub MaxTorque: f32,
    pub Transform: CFrame,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationController {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationFromVideoCreatorService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationFromVideoCreatorStudioService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationImportData {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationRigData {
    pub Label: BinaryString,
    pub Name: BinaryString,
    pub Parent: BinaryString,
    pub PostTransform: BinaryString,
    pub PreTransform: BinaryString,
    pub Transform: BinaryString,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationStreamTrack {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnimationTrack {
    pub Priority: enums::AnimationPriority,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Animator {
    pub PreferLodEnabled: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Annotation {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AnnotationsService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AppLifecycleObserverService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AppStorageService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AppUpdateService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ArcHandles {
    pub Axes: Axes,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetCounterService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetDeliveryProxy {
    pub Interface: String,
    pub Port: i32,
    pub StartServer: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetImportService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetImportSession {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetManagerService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetPatchSettings {
    pub ContentId: String,
    pub OutputPath: String,
    pub PatchId: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AssetSoundEffect {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Atmosphere {
    pub Color: Color3,
    pub Decay: Color3,
    pub Density: f32,
    pub Glare: f32,
    pub Haze: f32,
    pub Offset: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AtmosphereSensor {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Attachment {
    pub CFrame: CFrame,
    pub Visible: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioAnalyzer {
    pub SpectrumEnabled: bool,
    pub WindowSize: enums::AudioWindowSize,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioChannelMixer {
    pub Layout: enums::AudioChannelLayout,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioChannelSplitter {
    pub Layout: enums::AudioChannelLayout,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioChorus {
    pub Bypass: bool,
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioCompressor {
    pub Attack: f32,
    pub Bypass: bool,
    pub MakeupGain: f32,
    pub Ratio: f32,
    pub Release: f32,
    pub Threshold: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioDeviceInput {
    pub AccessType: enums::AccessModifierType,
    pub Active: bool,
    pub Muted: bool,
    pub Player: Ref,
    pub Volume: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioDeviceOutput {
    pub Player: Ref,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioDistortion {
    pub Bypass: bool,
    pub Level: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioEcho {
    pub Bypass: bool,
    pub DelayTime: f32,
    pub DryLevel: f32,
    pub Feedback: f32,
    pub RampTime: f32,
    pub WetLevel: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioEmitter {
    pub AngleAttenuation: BinaryString,
    pub AudioInteractionGroup: String,
    pub DistanceAttenuation: BinaryString,
    pub SimulationFidelity: enums::AudioSimulationFidelity,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioEqualizer {
    pub Bypass: bool,
    pub HighGain: f32,
    pub LowGain: f32,
    pub MidGain: f32,
    pub MidRange: NumberRange,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioFader {
    pub Bypass: bool,
    pub Volume: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioFilter {
    pub Bypass: bool,
    pub FilterType: enums::AudioFilterType,
    pub Frequency: f32,
    pub Gain: f32,
    pub Q: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioFlanger {
    pub Bypass: bool,
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioFocusService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioLimiter {
    pub Bypass: bool,
    pub MaxLevel: f32,
    pub Release: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioListener {
    pub AngleAttenuation: BinaryString,
    pub AudioInteractionGroup: String,
    pub DistanceAttenuation: BinaryString,
    pub SimulationFidelity: enums::AudioSimulationFidelity,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioPages {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioPitchShifter {
    pub Bypass: bool,
    pub Pitch: f32,
    pub WindowSize: enums::AudioWindowSize,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioPlayer {
    pub Asset: ContentId,
    pub AutoLoad: bool,
    pub LoopRegion: NumberRange,
    pub Looping: bool,
    pub PlaybackRegion: NumberRange,
    pub PlaybackSpeed: f64,
    pub TimePosition: f64,
    pub Volume: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioReverb {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioSearchParams {
    pub Album: String,
    pub Artist: String,
    pub AudioSubType: enums::AudioSubType,
    pub MaxDuration: i32,
    pub MinDuration: i32,
    pub SearchKeyword: String,
    pub Tag: String,
    pub Title: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AudioTextToSpeech {
    pub Looping: bool,
    pub Pitch: f32,
    pub PlaybackSpeed: f32,
    pub Speed: f32,
    pub Text: String,
    pub TimePosition: f64,
    pub VoiceId: String,
    pub Volume: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AuroraScript {
    pub EnableCulling: bool,
    pub EnableLod: bool,
    pub LodCriticality: i32,
    pub Priority: i32,
    pub Source: String,
    pub Tag: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AuroraScriptService {
    pub BufferSize: i32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AuroraService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarChatService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarCreationService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarEditorService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarImportService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct AvatarPreloader {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Backpack {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BackpackItem {
    pub TextureId: ContentId,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BadgeService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BallSocketConstraint {
    pub LimitsEnabled: bool,
    pub MaxFrictionTorqueXml: f32,
    pub Radius: f32,
    pub Restitution: f32,
    pub TwistLimitsEnabled: bool,
    pub TwistLowerAngle: f32,
    pub TwistUpperAngle: f32,
    pub UpperAngle: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BanHistoryPages {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BaseImportData {
    pub ImportName: String,
    pub ShouldImport: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BasePart {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BasePlayerGui {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BaseRemoteEvent {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BaseScript {
    pub Disabled: bool,
    pub LinkedSource: ContentId,
    pub RunContext: enums::RunContext,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BaseWrap {
    pub CageMeshContent: Content,
    pub CageOrigin: CFrame,
    pub HsrAssetId: ContentId,
    pub HsrData: SharedString,
    pub HsrMeshIdData: SharedString,
    pub ImportOrigin: CFrame,
    pub TemporaryCageMeshId: ContentId,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Beam {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BevelMesh {
    pub Bevel: f32,
    pub BevelRoundness: f32,
    pub Bulge: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BillboardGui {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BinaryStringValue {
    pub Value: BinaryString,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BindableEvent {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BindableFunction {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BlockMesh {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BloomEffect {
    pub Intensity: f32,
    pub Size: f32,
    pub Threshold: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BlurEffect {
    pub Size: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyAngularVelocity {
    pub AngularVelocity: Vector3,
    pub MaxTorque: Vector3,
    pub P: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyColors {
    pub HeadColor3: Color3,
    pub LeftArmColor3: Color3,
    pub LeftLegColor3: Color3,
    pub RightArmColor3: Color3,
    pub RightLegColor3: Color3,
    pub TorsoColor3: Color3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyForce {
    pub Force: Vector3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyGyro {
    pub CFrame: CFrame,
    pub D: f32,
    pub MaxTorque: Vector3,
    pub P: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyMover {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyPartDescription {
    pub AssetId: i64,
    pub BodyPart: enums::BodyPart,
    pub Color: Color3,
    pub Instance: Ref,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyPosition {
    pub D: f32,
    pub MaxForce: Vector3,
    pub P: f32,
    pub Position: Vector3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyThrust {
    pub Force: Vector3,
    pub Location: Vector3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BodyVelocity {
    pub MaxForce: Vector3,
    pub P: f32,
    pub Velocity: Vector3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Bone {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BoolValue {
    pub Value: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BoxHandleAdornment {
    pub Size: Vector3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Breakpoint {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BrickColorValue {
    pub Value: BrickColor,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BrowserService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BubbleChatConfiguration {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BubbleChatMessageProperties {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BugReporterService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BulkImportService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct BuoyancySensor {
    pub FullySubmerged: bool,
    pub TouchingSurface: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CFrameValue {
    pub Value: CFrame,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CSGDictionaryService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CacheableContentProvider {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CalloutService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Camera {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CanvasGroup {
    pub GroupColor3: Color3,
    pub GroupTransparency: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Capture {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CaptureService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CatalogPages {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChangeHistoryService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChannelSelectorSoundEffect {
    pub Channel: i32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChannelTabsConfiguration {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CharacterAppearance {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CharacterMesh {
    pub BaseTextureId: i64,
    pub BodyPart: enums::BodyPart,
    pub MeshId: i64,
    pub OverlayTextureId: i64,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Chat {
    pub BubbleChatEnabled: bool,
    pub IsAutoMigrated: bool,
    pub LoadDefaultChat: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChatInputBarConfiguration {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChatWindowConfiguration {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChatWindowMessageProperties {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChatbotUIService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ChorusSoundEffect {
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ClickDetector {
    pub CursorIcon: ContentId,
    pub MaxActivationDistance: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ClientReplicator {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ClimbController {
    pub AccelerationTime: f32,
    pub BalanceMaxTorque: f32,
    pub BalanceSpeed: f32,
    pub MoveMaxForce: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Clothing {
    pub Color3: Color3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CloudCRUDService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CloudLocalizationTable {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Clouds {
    pub Color: Color3,
    pub Cover: f32,
    pub Density: f32,
    pub Enabled: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ClusterPacketCache {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Collaborator {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CollaboratorsService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CollectionService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Color3Value {
    pub Value: Color3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ColorCorrectionEffect {
    pub Brightness: f32,
    pub Contrast: f32,
    pub Saturation: f32,
    pub TintColor: Color3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ColorGradingEffect {
    pub TonemapperPreset: enums::TonemapperPreset,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CommandInstance {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CommandService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CommerceService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CompressorSoundEffect {
    pub Attack: f32,
    pub GainMakeup: f32,
    pub Ratio: f32,
    pub Release: f32,
    pub SideChain: Ref,
    pub Threshold: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ConeHandleAdornment {
    pub Height: f32,
    pub Radius: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ConfigService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ConfigSnapshot {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Configuration {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ConfigureServerService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ConnectivityService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Constraint {
    pub Attachment0: Ref,
    pub Attachment1: Ref,
    pub Color: BrickColor,
    pub Enabled: bool,
    pub Visible: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ContentProvider {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ContextActionService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Controller {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ControllerBase {
    pub BalanceRigidityEnabled: bool,
    pub MoveSpeedFactor: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ControllerManager {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ControllerPartSensor {
    pub HitFrame: CFrame,
    pub HitNormal: Vector3,
    pub SearchDistance: f32,
    pub SensedPart: Ref,
    pub SensorMode: enums::SensorMode,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ControllerSensor {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ControllerService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ConversationalAIAcceptanceService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CookiesService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CoreGui {
    pub SelectionImageObject: Ref,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CorePackages {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CoreScript {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CoreScriptDebuggingManagerHelper {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CoreScriptSyncService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CornerWedgePart {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CreationDBService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CreatorStoreService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CrossDMScriptChangeListener {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CurveAnimation {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CustomEvent {
    pub PersistedCurrentValue: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CustomEventReceiver {
    pub Source: Ref,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CustomLog {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CustomSoundEffect {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CylinderHandleAdornment {
    pub Angle: f32,
    pub Height: f32,
    pub InnerRadius: f32,
    pub Radius: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CylinderMesh {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct CylindricalConstraint {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataModel {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataModelMesh {
    pub Offset: Vector3,
    pub Scale: Vector3,
    pub VertexColor: Vector3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataModelPatchService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataModelSession {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStore {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreGetOptions {
    pub UseCache: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreIncrementOptions {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreInfo {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreKey {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreKeyInfo {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreKeyPages {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreListingPages {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreObjectVersionInfo {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreOptions {
    pub AllScopes: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStorePages {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreService {
    pub AutomaticRetry: bool,
    pub LegacyNamingScheme: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreSetOptions {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DataStoreVersionPages {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Debris {
    pub MaxItems: i32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebugSettings {
    pub IsScriptStackTracingEnabled: bool,
    pub ReportSoundWarnings: bool,
    pub TickCountPreciseOverride: enums::TickCountSampleMethod,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggablePluginWatcher {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerBreakpoint {
    pub Condition: String,
    pub ContinueExecution: bool,
    pub IsContextDependentBreakpoint: bool,
    pub IsEnabled: bool,
    pub Line: i32,
    pub LogExpression: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerConnection {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerConnectionManager {
    pub Timeout: f64,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerLuaResponse {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerManager {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerUIService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerVariable {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DebuggerWatch {
    pub Expression: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Decal {
    pub Color3: Color3,
    pub TextureContent: Content,
    pub Transparency: f32,
    pub ZIndex: i32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DepthOfFieldEffect {
    pub FarIntensity: f32,
    pub FocusDistance: f32,
    pub InFocusRadius: f32,
    pub NearIntensity: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DeviceIdService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Dialog {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DialogChoice {
    pub GoodbyeChoiceActive: bool,
    pub GoodbyeDialog: String,
    pub ResponseDialog: String,
    pub UserDialog: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DistortionSoundEffect {
    pub Level: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DockWidgetPluginGui {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DoubleConstrainedValue {
    pub MaxValue: f64,
    pub MinValue: f64,
    pub Value: f64,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DraftsService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DragDetector {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Dragger {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DraggerService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct DynamicRotate {
    pub BaseAngle: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EchoSoundEffect {
    pub Delay: f32,
    pub DryLevel: f32,
    pub Feedback: f32,
    pub WetLevel: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EditableImage {
    pub ImageData: BinaryString,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EditableMesh {
    pub MeshData: SharedString,
    pub SkinningEnabled: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EditableService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EmotesPages {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EqualizerSoundEffect {
    pub HighGain: f32,
    pub LowGain: f32,
    pub MidGain: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EulerRotationCurve {
    pub RotationOrder: enums::RotationOrder,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct EventIngestService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ExampleService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ExperienceAuthService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ExperienceInviteOptions {
    pub InviteMessageId: String,
    pub InviteUser: i64,
    pub LaunchData: String,
    pub PromptMessage: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ExperienceNotificationService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ExperienceService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ExperienceStateCaptureService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ExplorerFilter {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ExplorerFilterAutocompleter {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ExplorerServiceVisibilityService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Explosion {
    pub BlastPressure: f32,
    pub BlastRadius: f32,
    pub DestroyJointRadiusPercent: f32,
    pub ExplosionType: enums::ExplosionType,
    pub Position: Vector3,
    pub TimeScale: f32,
    pub Visible: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FaceAnimatorService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FaceControls {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FaceInstance {
    pub Face: enums::NormalId,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FacialAgeEstimationService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FacialAnimationRecordingService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FacialAnimationStreamingServiceStats {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FacialAnimationStreamingServiceV2 {
    pub ServiceState: i32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FacialAnimationStreamingSubsessionStats {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FacsImportData {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Feature {
    pub FaceId: enums::NormalId,
    pub InOut: enums::InOut,
    pub LeftRight: enums::LeftRight,
    pub TopBottom: enums::TopBottom,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FeatureRestrictionManager {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FeedPages {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FeedService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct File {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FileMesh {
    pub MeshId: ContentId,
    pub TextureId: ContentId,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Fire {
    pub Color: Color3,
    pub Enabled: bool,
    pub SecondaryColor: Color3,
    pub TimeScale: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Flag {
    pub TeamColor: BrickColor,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FlagStand {
    pub TeamColor: BrickColor,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FlagStandService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FlangeSoundEffect {
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FloatCurve {
    pub ValuesAndTimes: BinaryString,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FloorWire {
    pub CycleOffset: f32,
    pub From: Ref,
    pub StudsBetweenTextures: f32,
    pub Texture: ContentId,
    pub TextureSize: Vector2,
    pub To: Ref,
    pub Velocity: f32,
    pub WireRadius: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FluidForceSensor {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FlyweightService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Folder {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ForceField {
    pub Visible: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FormFactorPart {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Frame {
    pub Style: enums::FrameStyle,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FriendPages {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FriendService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct FunctionalTest {
    pub Description: String,
    pub HasMigratedSettingsToTestService: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GamePassService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GameSettings {
    pub VideoCaptureEnabled: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GamepadService {
    pub GamepadCursorEnabled: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GenerationService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GenericChallengeService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GenericSettings {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Geometry {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GeometryService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GetTextBoundsParams {
    pub Font: Font,
    pub RichText: bool,
    pub Size: f32,
    pub Text: String,
    pub Width: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GlobalDataStore {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GlobalSettings {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Glue {
    pub F0: Vector3,
    pub F1: Vector3,
    pub F2: Vector3,
    pub F3: Vector3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GoogleAnalyticsConfiguration {
    pub GaId: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GroundController {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GroupImportData {
    pub Anchored: bool,
    pub ImportAsModelAsset: bool,
    pub InsertInWorkspace: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GroupService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiBase {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiBase2d {
    pub AutoLocalize: bool,
    pub RootLocalizationTable: Ref,
    pub SelectionBehaviorDown: enums::SelectionBehavior,
    pub SelectionBehaviorLeft: enums::SelectionBehavior,
    pub SelectionBehaviorRight: enums::SelectionBehavior,
    pub SelectionBehaviorUp: enums::SelectionBehavior,
    pub SelectionGroup: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiBase3d {
    pub Color3: Color3,
    pub Transparency: f32,
    pub Visible: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiButton {
    pub AutoButtonColor: bool,
    pub HoverHapticEffect: Ref,
    pub Modal: bool,
    pub PressHapticEffect: Ref,
    pub Selected: bool,
    pub Style: enums::ButtonStyle,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiLabel {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiMain {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiObject {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuiService {
    pub AutoSelectGuiEnabled: bool,
    pub GuiNavigationEnabled: bool,
    pub SelectedObject: Ref,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct GuidRegistryService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HSRDataContentProvider {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HandleAdornment {
    pub AdornCullingMode: enums::AdornCullingMode,
    pub AlwaysOnTop: bool,
    pub CFrame: CFrame,
    pub SizeRelativeOffset: Vector3,
    pub ZIndex: i32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Handles {
    pub Faces: Faces,
    pub Style: enums::HandlesStyle,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HandlesBase {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HapticEffect {
    pub Looped: bool,
    pub Position: Vector3,
    pub Radius: f32,
    pub Type: enums::HapticEffectType,
    pub Waveform: Ref,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HapticService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Hat {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HeapProfilerService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HeatmapService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HeightmapImporterService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HiddenSurfaceRemovalAsset {
    pub HsrData: BinaryString,
    pub HsrMeshIdData: BinaryString,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Highlight {
    pub Adornee: Ref,
    pub DepthMode: enums::HighlightDepthMode,
    pub Enabled: bool,
    pub FillColor: Color3,
    pub FillTransparency: f32,
    pub OutlineColor: Color3,
    pub OutlineTransparency: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HingeConstraint {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Hint {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Hole {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Hopper {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HopperBin {
    pub Active: bool,
    pub BinType: enums::BinType,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HttpRbxApiService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HttpRequest {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HttpService {
    pub HttpEnabled: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Humanoid {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HumanoidController {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HumanoidDescription {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct HumanoidRigDescription {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct IKControl {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ILegacyStudioBridge {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct IXPService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ImageButton {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ImageHandleAdornment {
    pub Image: ContentId,
    pub Size: Vector2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ImageLabel {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ImportSession {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct IncrementalPatchBuilder {
    pub AddPathsToBundle: bool,
    pub BuildDebouncePeriod: f64,
    pub HighCompression: bool,
    pub SerializePatch: bool,
    pub ZstdCompression: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InputAction {
    pub Enabled: bool,
    pub Type: enums::InputActionType,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InputBinding {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InputContext {
    pub Enabled: bool,
    pub Priority: i32,
    pub Sink: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InputObject {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InsertService {
    pub AllowClientInsertModels: bool,
    pub AllowInsertFreeModels: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Instance {
    pub Capabilities: SecurityCapabilities,
    pub HistoryId: UniqueId,
    pub Name: String,
    pub SourceAssetId: i64,
    pub Tags: Tags,
    pub UniqueId: UniqueId,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InstanceAdornment {
    pub Adornee: Ref,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct IntConstrainedValue {
    pub MaxValue: i64,
    pub MinValue: i64,
    pub Value: i64,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct IntValue {
    pub Value: i64,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InternalSyncItem {
    pub AutoSync: bool,
    pub Enabled: bool,
    pub Path: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InternalSyncService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct IntersectOperation {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct InventoryPages {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct JointImportData {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct JointInstance {
    pub C0: CFrame,
    pub C1: CFrame,
    pub Enabled: bool,
    pub Part0: Ref,
    pub Part1: Ref,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct JointsService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct KeyboardService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Keyframe {
    pub Time: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct KeyframeMarker {
    pub Value: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct KeyframeSequence {
    pub AuthoredHipHeight: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct KeyframeSequenceProvider {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LSPFileSyncService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LanguageService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LayerCollector {
    pub Enabled: bool,
    pub ResetOnSpawn: bool,
    pub ZIndexBehavior: enums::ZIndexBehavior,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LegacyStudioBridge {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Light {
    pub Brightness: f32,
    pub Color: Color3,
    pub Enabled: bool,
    pub Shadows: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Lighting {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LineForce {
    pub ApplyAtCenterOfMass: bool,
    pub InverseSquareLaw: bool,
    pub Magnitude: f32,
    pub MaxForce: f32,
    pub ReactionForceEnabled: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LineHandleAdornment {
    pub Length: f32,
    pub Thickness: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LinearVelocity {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LinkingService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LiveScriptingService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LiveSyncService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LocalDebuggerConnection {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LocalScript {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LocalStorageService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LocalizationService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LocalizationTable {
    pub Contents: String,
    pub SourceLocaleId: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LodDataEntity {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LodDataService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LogReporterService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LogService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LoginService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LuaSettings {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LuaSourceContainer {
    pub ScriptGuid: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LuaWebService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct LuauScriptAnalyzerService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MLModelDeliveryService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ManualGlue {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ManualSurfaceJointInstance {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ManualWeld {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MarkerCurve {
    pub ValuesAndTimes: BinaryString,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MarketplaceService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MatchmakingService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MaterialGenerationService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MaterialGenerationSession {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MaterialImportData {
    pub DiffuseFilePath: String,
    pub MetalnessFilePath: String,
    pub NormalFilePath: String,
    pub RoughnessFilePath: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MaterialService {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MaterialVariant {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MemStorageConnection {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MemStorageService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MemoryStoreHashMap {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MemoryStoreHashMapPages {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MemoryStoreQueue {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MemoryStoreService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MemoryStoreSortedMap {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MeshContentProvider {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MeshImportData {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MeshPart {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Message {
    pub Text: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MessageBusConnection {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MessageBusService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MessagingService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MetaBreakpoint {
    pub Condition: String,
    pub ContinueExecution: bool,
    pub Enabled: bool,
    pub Line: i32,
    pub LogMessage: String,
    pub RemoveOnHit: bool,
    pub Script: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MetaBreakpointContext {
    pub ContextDataInternal: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MetaBreakpointManager {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Model {
    pub LevelOfDetail: enums::ModelLevelOfDetail,
    pub ModelMeshCFrame: CFrame,
    pub ModelMeshData: SharedString,
    pub ModelMeshSize: Vector3,
    pub ModelStreamingMode: enums::ModelStreamingMode,
    pub NeedsPivotMigration: bool,
    pub PrimaryPart: Ref,
    pub WorldPivotData: Option<CFrame>,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ModuleScript {
    pub LinkedSource: ContentId,
    pub Source: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Motor {
    pub DesiredAngle: f32,
    pub MaxVelocity: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Motor6D {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MotorFeature {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Mouse {
    pub Icon: ContentId,
    pub TargetFilter: Ref,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MouseService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct MultipleDocumentInterfaceInstance {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NegateOperation {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NetworkClient {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NetworkMarker {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NetworkPeer {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NetworkReplicator {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NetworkServer {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NetworkSettings {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NoCollisionConstraint {
    pub Enabled: bool,
    pub Part0: Ref,
    pub Part1: Ref,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Noise {
    pub NoiseType: enums::NoiseType,
    pub Seed: i32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NonReplicatedCSGDictionaryService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NotificationService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NumberPose {
    pub Value: f64,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct NumberValue {
    pub Value: f64,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Object {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ObjectValue {
    pub Value: Ref,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct OmniRecommendationsService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct OpenCloudApiV1 {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct OpenCloudService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct OperationGraph {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct OrderedDataStore {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct OutfitPages {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PVAdornment {
    pub Adornee: Ref,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PVInstance {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PackageLink {
    pub AutoUpdate: bool,
    pub DefaultName: String,
    pub ModifiedState: i32,
    pub SerializedDefaultAttributes: BinaryString,
    pub VersionIdSerialize: i64,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PackageService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PackageUIService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Pages {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Pants {
    pub PantsTemplate: ContentId,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ParabolaAdornment {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Part {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PartAdornment {
    pub Adornee: Ref,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PartOperation {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PartOperationAsset {
    pub ChildData: BinaryString,
    pub MeshData: BinaryString,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ParticleEmitter {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PatchBundlerFileWatch {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PatchMapping {
    pub FlattenTree: bool,
    pub PatchId: String,
    pub TargetPath: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Path {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Path2D {
    pub Closed: bool,
    pub Color3: Color3,
    pub PropertiesSerialize: BinaryString,
    pub Thickness: f32,
    pub Transparency: f32,
    pub Visible: bool,
    pub ZIndex: i32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PathfindingLink {
    pub Attachment0: Ref,
    pub Attachment1: Ref,
    pub IsBidirectional: bool,
    pub Label: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PathfindingModifier {
    pub Label: String,
    pub PassThrough: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PathfindingService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PausedState {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PausedStateBreakpoint {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PausedStateException {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PerformanceControlService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PermissionsService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PhysicsService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PhysicsSettings {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PitchShiftSoundEffect {
    pub Octave: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlaceAssetIdsService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlaceStatsService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlacesService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Plane {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlaneConstraint {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Platform {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlatformCloudStorageService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlatformFriendsService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Player {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerData {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerDataRecord {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerDataRecordConfig {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerDataService {
    pub LoadFailureBehavior: enums::PlayerDataLoadFailureBehavior,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerEmulatorService {
    pub CustomPoliciesEnabled: bool,
    pub EmulatedCountryCode: String,
    pub EmulatedGameLocale: String,
    pub PlayerEmulationEnabled: bool,
    pub PseudolocalizationEnabled: bool,
    pub SerializedEmulatedPolicyInfo: BinaryString,
    pub TextElongationFactor: i32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerGui {
    pub ScreenOrientation: enums::ScreenOrientation,
    pub SelectionImageObject: Ref,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerHydrationService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerMouse {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerScripts {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PlayerViewService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Players {
    pub BanningEnabled: bool,
    pub CharacterAutoLoads: bool,
    pub RespawnTime: f32,
    pub UseStrafingAnimations: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Plugin {
    pub DisableUiDragDetectorDrags: bool,
    pub IsDebuggable: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginAction {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginCapabilities {
    pub Manifest: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginDebugService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginDragEvent {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginGui {
    pub Title: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginGuiService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginManagementService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginManager {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginManagerInterface {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginMenu {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginMouse {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginPolicyService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginToolbar {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PluginToolbarButton {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PointLight {
    pub Range: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PointsService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PolicyService {
    pub IsLuobuServer: enums::TriStateBoolean,
    pub LuobuWhitelisted: enums::TriStateBoolean,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Pose {
    pub CFrame: CFrame,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PoseBase {
    pub EasingDirection: enums::PoseEasingDirection,
    pub EasingStyle: enums::PoseEasingStyle,
    pub Weight: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PostEffect {
    pub Enabled: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PrismaticConstraint {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ProcessInstancePhysicsService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ProximityPrompt {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ProximityPromptService {
    pub Enabled: bool,
    pub MaxPromptsVisible: i32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct PublishService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct QWidgetPluginGui {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RTAnimationTracker {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RayValue {
    pub Value: Ray,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RbxAnalyticsService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadata {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataCallbacks {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataClass {
    pub ExplorerImageIndex: i32,
    pub ExplorerOrder: i32,
    pub Insertable: bool,
    pub PreferredParent: String,
    pub ServiceVisibility: enums::ServiceVisibility,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataClasses {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataEnum {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataEnumItem {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataEnums {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataEvents {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataFunctions {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataItem {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataMember {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataProperties {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionMetadataYieldFunctions {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReflectionService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RelativeGui {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RemoteCursorService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RemoteDebuggerServer {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RemoteEvent {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RemoteFunction {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RenderSettings {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RenderingTest {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReplicatedFirst {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReplicatedStorage {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ReverbSoundEffect {
    pub DecayTime: f32,
    pub Density: f32,
    pub Diffusion: f32,
    pub DryLevel: f32,
    pub WetLevel: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RibbonNotificationService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RigidConstraint {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RobloxPluginGuiService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RobloxReplicatedStorage {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RobloxSerializableInstance {
    pub Data: BinaryString,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RobloxServerStorage {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RocketPropulsion {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RodConstraint {
    pub Length: f32,
    pub LimitAngle0: f32,
    pub LimitAngle1: f32,
    pub LimitsEnabled: bool,
    pub Thickness: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RomarkRbxAnalyticsService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RomarkService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RootImportData {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RopeConstraint {
    pub Length: f32,
    pub Restitution: f32,
    pub Thickness: f32,
    pub WinchEnabled: bool,
    pub WinchForce: f32,
    pub WinchResponsiveness: f32,
    pub WinchSpeed: f32,
    pub WinchTarget: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Rotate {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RotateP {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RotateV {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RotationCurve {
    pub ValuesAndTimes: BinaryString,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RtMessagingService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RunService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RunningAverageItemDouble {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RunningAverageItemInt {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RunningAverageTimeIntervalItem {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct RuntimeScriptService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SafetyService {
    pub IsCaptureModeForReport: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScreenGui {
    pub ClipToDeviceSafeArea: bool,
    pub DisplayOrder: i32,
    pub SafeAreaCompatibility: enums::SafeAreaCompatibility,
    pub ScreenInsets: enums::ScreenInsets,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScreenshotCapture {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScreenshotHud {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Script {
    pub Source: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptBuilder {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptChangeService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptCloneWatcher {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptCloneWatcherHelper {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptCommitService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptContext {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptDebugger {
    pub CoreScriptIdentifier: String,
    pub ScriptGuid: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptDocument {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptEditorService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptProfilerService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptRegistrationService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptRuntime {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScriptService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ScrollingFrame {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Seat {
    pub Disabled: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Selection {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionBox {
    pub LineThickness: f32,
    pub StudioSelectionBox: bool,
    pub SurfaceColor3: Color3,
    pub SurfaceTransparency: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionHighlightManager {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionLasso {
    pub Humanoid: Ref,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionPartLasso {
    pub Part: Ref,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionPointLasso {
    pub Point: Vector3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SelectionSphere {
    pub SurfaceColor3: Color3,
    pub SurfaceTransparency: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SensorBase {
    pub UpdateType: enums::SensorUpdateType,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SerializationService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ServerReplicator {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ServerScriptService {
    pub LoadStringEnabled: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ServerStorage {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ServiceProvider {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ServiceVisibilityService {
    pub HiddenServices: BinaryString,
    pub VisibleServices: BinaryString,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SessionService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SharedTableRegistry {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Shirt {
    pub ShirtTemplate: ContentId,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ShirtGraphic {
    pub Color3: Color3,
    pub Graphic: ContentId,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SkateboardController {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SkateboardPlatform {
    pub Steer: i32,
    pub StickyWheels: bool,
    pub Throttle: i32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Skin {
    pub SkinColor: BrickColor,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Sky {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SlidingBallConstraint {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Smoke {
    pub Color: Color3,
    pub Enabled: bool,
    pub TimeScale: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SmoothVoxelsUpgraderService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Snap {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SnippetService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SocialService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SolidModelContentProvider {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Sound {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SoundEffect {
    pub Enabled: bool,
    pub Priority: i32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SoundGroup {
    pub Volume: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SoundService {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Sparkles {
    pub Enabled: bool,
    pub SparkleColor: Color3,
    pub TimeScale: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SpawnLocation {
    pub AllowTeamChangeOnTouch: bool,
    pub Duration: i32,
    pub Enabled: bool,
    pub Neutral: bool,
    pub TeamColor: BrickColor,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SpawnerService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SpecialMesh {
    pub MeshType: enums::MeshType,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SphereHandleAdornment {
    pub Radius: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SpotLight {
    pub Angle: f32,
    pub Face: enums::NormalId,
    pub Range: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SpringConstraint {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StackFrame {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StandalonePluginScripts {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StandardPages {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StartPageService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StarterCharacterScripts {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StarterGear {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StarterGui {
    pub ResetPlayerGuiOnSpawn: bool,
    pub RtlTextSupport: enums::RtlTextSupport,
    pub ScreenOrientation: enums::ScreenOrientation,
    pub ShowDevelopmentGui: bool,
    pub StudioDefaultStyleSheet: Ref,
    pub StudioInsertWidgetLayerCollectorAutoLinkStyleSheet: Ref,
    pub VirtualCursorMode: enums::VirtualCursorMode,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StarterPack {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StarterPlayer {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StarterPlayerScripts {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StartupMessageService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Stats {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StatsItem {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Status {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StopWatchReporter {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StreamingService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StringValue {
    pub Value: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Studio {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioAssetService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioAttachment {
    pub AutoHideParent: bool,
    pub IsArrowVisible: bool,
    pub Offset: Vector2,
    pub SourceAnchorPoint: Vector2,
    pub TargetAnchorPoint: Vector2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioCallout {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioCameraService {
    pub LockCameraSpeed: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioData {
    pub EnableScriptCollabByDefaultOnLoad: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioDeviceEmulatorService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioObjectBase {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioPublishService {
    pub PublishLocked: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioScriptDebugEventListener {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioSdkService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioService {
    pub Secrets: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioTheme {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioUserService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioWidget {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StudioWidgetsService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StyleBase {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StyleDerive {
    pub Index: i32,
    pub StyleSheet: Ref,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StyleLink {
    pub StyleSheet: Ref,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StyleRule {
    pub Priority: i32,
    pub PropertiesSerialize: BinaryString,
    pub Selector: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StyleSheet {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct StylingService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SunRaysEffect {
    pub Intensity: f32,
    pub Spread: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SurfaceAppearance {
    pub AlphaMode: enums::AlphaMode,
    pub Color: Color3,
    pub ColorMapContent: Content,
    pub MetalnessMapContent: Content,
    pub NormalMapContent: Content,
    pub RoughnessMapContent: Content,
    pub TexturePack: ContentId,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SurfaceGui {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SurfaceGuiBase {
    pub Active: bool,
    pub Adornee: Ref,
    pub Face: enums::NormalId,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SurfaceLight {
    pub Angle: f32,
    pub Face: enums::NormalId,
    pub Range: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SurfaceSelection {
    pub TargetSurface: enums::NormalId,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SwimController {
    pub AccelerationTime: f32,
    pub PitchMaxTorque: f32,
    pub PitchSpeedFactor: f32,
    pub RollMaxTorque: f32,
    pub RollSpeedFactor: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SyncScriptBuilder {
    pub CompileTarget: enums::CompileTarget,
    pub CoverageInfo: bool,
    pub DebugInfo: bool,
    pub PackAsSource: bool,
    pub RawBytecode: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct SystemThemeService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TaskScheduler {
    pub ThreadPoolConfig: enums::ThreadPoolConfig,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Team {
    pub AutoAssignable: bool,
    pub TeamColor: BrickColor,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TeamCreateData {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TeamCreatePublishService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TeamCreateService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Teams {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TelemetryService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TeleportAsyncResult {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TeleportOptions {
    pub ReservedServerAccessCode: String,
    pub ServerInstanceId: String,
    pub ShouldReserveServer: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TeleportService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TemporaryCageMeshProvider {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TemporaryScriptService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Terrain {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TerrainDetail {
    pub ColorMap: ContentId,
    pub Face: enums::TerrainFace,
    pub MaterialPattern: enums::MaterialPattern,
    pub MetalnessMap: ContentId,
    pub NormalMap: ContentId,
    pub RoughnessMap: ContentId,
    pub StudsPerTile: f32,
    pub TexturePack: ContentId,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TerrainRegion {
    pub ExtentsMax: Vector3int16,
    pub ExtentsMin: Vector3int16,
    pub SmoothGrid: BinaryString,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TestService {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextBox {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextBoxService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextButton {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextChannel {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextChatCommand {
    pub AutocompleteVisible: bool,
    pub Enabled: bool,
    pub PrimaryAlias: String,
    pub SecondaryAlias: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextChatConfigurations {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextChatMessage {
    pub BubbleChatMessageProperties: Ref,
    pub ChatWindowMessageProperties: Ref,
    pub TextChannel: Ref,
    pub TextSource: Ref,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextChatMessageProperties {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextChatService {
    pub ChatTranslationFtuxShown: bool,
    pub ChatTranslationToggleEnabled: bool,
    pub ChatVersion: enums::ChatVersion,
    pub CreateDefaultCommands: bool,
    pub CreateDefaultTextChannels: bool,
    pub HasSeenDeprecationDialog: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextFilterResult {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextFilterTranslatedResult {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextLabel {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextSource {
    pub CanSend: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Texture {
    pub OffsetStudsU: f32,
    pub OffsetStudsV: f32,
    pub StudsPerTileU: f32,
    pub StudsPerTileV: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextureGenerationPartGroup {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextureGenerationService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TextureGenerationUnwrappingRequest {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ThirdPartyUserService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ThreadState {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TimerService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ToastNotificationService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Tool {
    pub CanBeDropped: bool,
    pub Enabled: bool,
    pub Grip: CFrame,
    pub ManualActivationOnly: bool,
    pub RequiresHandle: bool,
    pub ToolTip: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Torque {
    pub RelativeTo: enums::ActuatorRelativeTo,
    pub Torque: Vector3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TorsionSpringConstraint {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TotalCountTimeIntervalItem {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TouchInputService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TouchTransmitter {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TracerService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TrackerLodController {
    pub AudioMode: enums::TrackerLodFlagMode,
    pub VideoExtrapolationMode: enums::TrackerExtrapolationFlagMode,
    pub VideoLodMode: enums::TrackerLodValueMode,
    pub VideoMode: enums::TrackerLodFlagMode,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TrackerStreamAnimation {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Trail {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Translator {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TremoloSoundEffect {
    pub Depth: f32,
    pub Duty: f32,
    pub Frequency: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TriangleMeshPart {
    pub AeroMeshData: SharedString,
    pub FluidFidelityInternal: enums::FluidFidelity,
    pub PhysicalConfigData: SharedString,
    pub UnscaledCofm: Vector3,
    pub UnscaledVolInertiaDiags: Vector3,
    pub UnscaledVolInertiaOffDiags: Vector3,
    pub UnscaledVolume: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TrussPart {
    pub Style: enums::Style,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TutorialService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Tween {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TweenBase {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct TweenService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UGCAvatarService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UGCValidationService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIAspectRatioConstraint {
    pub AspectRatio: f32,
    pub AspectType: enums::AspectType,
    pub DominantAxis: enums::DominantAxis,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIBase {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIComponent {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIConstraint {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UICorner {
    pub CornerRadius: UDim,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIDragDetector {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIDragDetectorService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIFlexItem {
    pub FlexMode: enums::UIFlexMode,
    pub GrowRatio: f32,
    pub ItemLineAlignment: enums::ItemLineAlignment,
    pub ShrinkRatio: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIGradient {
    pub Color: ColorSequence,
    pub Enabled: bool,
    pub Offset: Vector2,
    pub Rotation: f32,
    pub Transparency: NumberSequence,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIGridLayout {
    pub CellPadding: UDim2,
    pub CellSize: UDim2,
    pub FillDirectionMaxCells: i32,
    pub StartCorner: enums::StartCorner,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIGridStyleLayout {
    pub FillDirection: enums::FillDirection,
    pub HorizontalAlignment: enums::HorizontalAlignment,
    pub SortOrder: enums::SortOrder,
    pub VerticalAlignment: enums::VerticalAlignment,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UILayout {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIListLayout {
    pub HorizontalFlex: enums::UIFlexAlignment,
    pub ItemLineAlignment: enums::ItemLineAlignment,
    pub Padding: UDim,
    pub VerticalFlex: enums::UIFlexAlignment,
    pub Wraps: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIPadding {
    pub PaddingBottom: UDim,
    pub PaddingLeft: UDim,
    pub PaddingRight: UDim,
    pub PaddingTop: UDim,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIPageLayout {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIScale {
    pub Scale: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UISizeConstraint {
    pub MaxSize: Vector2,
    pub MinSize: Vector2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UIStroke {
    pub ApplyStrokeMode: enums::ApplyStrokeMode,
    pub Color: Color3,
    pub Enabled: bool,
    pub LineJoinMode: enums::LineJoinMode,
    pub Thickness: f32,
    pub Transparency: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UITableLayout {
    pub FillEmptySpaceColumns: bool,
    pub FillEmptySpaceRows: bool,
    pub MajorAxis: enums::TableMajorAxis,
    pub Padding: UDim2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UITextSizeConstraint {
    pub MaxTextSize: i32,
    pub MinTextSize: i32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UnionOperation {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UniqueIdLookupService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UniversalConstraint {
    pub LimitsEnabled: bool,
    pub MaxAngle: f32,
    pub Radius: f32,
    pub Restitution: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UnreliableRemoteEvent {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UnvalidatedAssetService {
    pub CachedData: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UserGameSettings {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UserInputService {
    pub LegacyInputEventsEnabled: bool,
    pub MouseBehavior: enums::MouseBehavior,
    pub MouseIcon: ContentId,
    pub MouseIconEnabled: bool,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UserService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UserSettings {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct UserStorageService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VRService {
    pub AutomaticScaling: enums::VRScaling,
    pub AvatarGestures: bool,
    pub ControllerModels: enums::VRControllerModelMode,
    pub FadeOutViewOnCollision: bool,
    pub LaserPointer: enums::VRLaserPointerMode,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VRStatusService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ValueBase {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Vector3Curve {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Vector3Value {
    pub Value: Vector3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VectorForce {
    pub ApplyAtCenterOfMass: bool,
    pub Force: Vector3,
    pub RelativeTo: enums::ActuatorRelativeTo,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VehicleController {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VehicleSeat {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VelocityMotor {
    pub CurrentAngle: f32,
    pub DesiredAngle: f32,
    pub Hole: Ref,
    pub MaxVelocity: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VersionControlService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VideoCaptureService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VideoDeviceInput {
    pub Active: bool,
    pub CameraId: String,
    pub CaptureQuality: enums::VideoDeviceCaptureQuality,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VideoDisplay {
    pub ResampleMode: enums::ResamplerMode,
    pub ScaleType: enums::ScaleType,
    pub TileSize: UDim2,
    pub VideoColor3: Color3,
    pub VideoRectOffset: Vector2,
    pub VideoRectSize: Vector2,
    pub VideoTransparency: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VideoFrame {
    pub Looped: bool,
    pub Playing: bool,
    pub TimePosition: f64,
    pub Video: ContentId,
    pub Volume: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VideoPlayer {
    pub Asset: ContentId,
    pub Looping: bool,
    pub PlaybackSpeed: f32,
    pub TimePosition: f64,
    pub Volume: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VideoService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct ViewportFrame {
    pub Ambient: Color3,
    pub CameraCFrame: CFrame,
    pub CameraFieldOfView: f32,
    pub ImageColor3: Color3,
    pub ImageTransparency: f32,
    pub LightColor: Color3,
    pub LightDirection: Vector3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VirtualInputManager {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VirtualUser {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VisibilityCheckDispatcher {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Visit {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VisualizationMode {
    pub Enabled: bool,
    pub Title: String,
    pub ToolTip: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VisualizationModeCategory {
    pub Enabled: bool,
    pub Title: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VisualizationModeService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VoiceChatInternal {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct VoiceChatService {
    pub DefaultDistanceAttenuation: enums::VoiceChatDistanceAttenuationType,
    pub EnableDefaultVoice: bool,
    pub UseAudioApi: enums::AudioApiRollout,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WebSocketClient {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WebSocketService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WebViewService {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WedgePart {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Weld {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WeldConstraint {
    pub CFrame0: CFrame,
    pub State: i32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Wire {
    pub SourceInstance: Ref,
    pub SourceName: String,
    pub TargetInstance: Ref,
    pub TargetName: String,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WireframeHandleAdornment {
    pub Scale: Vector3,
    pub Thickness: f32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct Workspace {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WorkspaceAnnotation {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WorldModel {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WorldRoot {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WrapDeformer {}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WrapLayer {
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
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub struct WrapTarget {
    pub Stiffness: f32,
}
