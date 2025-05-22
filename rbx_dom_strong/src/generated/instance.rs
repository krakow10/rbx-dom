use super::r#enum::*;
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
pub struct Accessory {
    pub AccessoryType: AccessoryType,
}
#[derive(Debug, Clone)]
pub struct AccessoryDescription {
    pub AccessoryType: AccessoryType,
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
pub struct AccountService {}
#[derive(Debug, Clone)]
pub struct Accoutrement {
    pub AttachmentForward: Vector3,
    pub AttachmentPoint: CFrame,
    pub AttachmentPos: Vector3,
    pub AttachmentRight: Vector3,
    pub AttachmentUp: Vector3,
    pub BackendAccoutrementState: i32,
}
#[derive(Debug, Clone)]
pub struct AchievementService {}
#[derive(Debug, Clone)]
pub struct ActivityHistoryEventService {}
#[derive(Debug, Clone)]
pub struct Actor {}
#[derive(Debug, Clone)]
pub struct AdGui {
    pub AdShape: AdShape,
    pub EnableVideoAds: bool,
    pub FallbackImage: ContentId,
    pub Status: AdUnitStatus,
}
#[derive(Debug, Clone)]
pub struct AdPortal {
    pub PortalInvalidReason: String,
    pub PortalVersion: i64,
    pub Status: AdUnitStatus,
}
#[derive(Debug, Clone)]
pub struct AdService {}
#[derive(Debug, Clone)]
pub struct AdvancedDragger {}
#[derive(Debug, Clone)]
pub struct AirController {
    pub BalanceMaxTorque: f32,
    pub BalanceSpeed: f32,
    pub LinearImpulse: Vector3,
    pub MaintainAngularMomentum: bool,
    pub MaintainLinearMomentum: bool,
    pub MoveMaxForce: f32,
    pub TurnMaxTorque: f32,
    pub TurnSpeedFactor: f32,
}
#[derive(Debug, Clone)]
pub struct AlignOrientation {
    pub AlignType: AlignType,
    pub CFrame: CFrame,
    pub LookAtPosition: Vector3,
    pub MaxAngularVelocity: f32,
    pub MaxTorque: f32,
    pub Mode: OrientationAlignmentMode,
    pub PrimaryAxis: Vector3,
    pub PrimaryAxisOnly: bool,
    pub ReactionTorqueEnabled: bool,
    pub Responsiveness: f32,
    pub RigidityEnabled: bool,
    pub SecondaryAxis: Vector3,
}
#[derive(Debug, Clone)]
pub struct AlignPosition {
    pub ApplyAtCenterOfMass: bool,
    pub ForceLimitMode: ForceLimitMode,
    pub ForceRelativeTo: ActuatorRelativeTo,
    pub MaxAxesForce: Vector3,
    pub MaxForce: f32,
    pub MaxVelocity: f32,
    pub Mode: PositionAlignmentMode,
    pub Position: Vector3,
    pub ReactionForceEnabled: bool,
    pub Responsiveness: f32,
    pub RigidityEnabled: bool,
}
#[derive(Debug, Clone)]
pub struct AnalysticsSettings {}
#[derive(Debug, Clone)]
pub struct AnalyticsService {
    pub ApiKey: String,
}
#[derive(Debug, Clone)]
pub struct AngularVelocity {
    pub AngularVelocity: Vector3,
    pub MaxTorque: f32,
    pub ReactionTorqueEnabled: bool,
    pub RelativeTo: ActuatorRelativeTo,
}
#[derive(Debug, Clone)]
pub struct Animation {
    pub AnimationId: ContentId,
}
#[derive(Debug, Clone)]
pub struct AnimationClip {
    pub Guid: String,
    pub GuidBinaryString: BinaryString,
    pub Loop: bool,
    pub Priority: AnimationPriority,
}
#[derive(Debug, Clone)]
pub struct AnimationClipProvider {}
#[derive(Debug, Clone)]
pub struct AnimationConstraint {
    pub C0: CFrame,
    pub C1: CFrame,
    pub IsKinematic: bool,
    pub MaxForce: f32,
    pub MaxTorque: f32,
    pub Part0: Ref,
    pub Part1: Ref,
    pub Transform: CFrame,
}
#[derive(Debug, Clone)]
pub struct AnimationController {}
#[derive(Debug, Clone)]
pub struct AnimationFromVideoCreatorService {}
#[derive(Debug, Clone)]
pub struct AnimationFromVideoCreatorStudioService {}
#[derive(Debug, Clone)]
pub struct AnimationImportData {}
#[derive(Debug, Clone)]
pub struct AnimationRigData {
    pub Label: BinaryString,
    pub Name: BinaryString,
    pub Parent: BinaryString,
    pub PostTransform: BinaryString,
    pub PreTransform: BinaryString,
    pub Transform: BinaryString,
}
#[derive(Debug, Clone)]
pub struct AnimationStreamTrack {
    pub Animation: Ref,
    pub FacsDataLod: FACSDataLod,
    pub IsPlaying: bool,
    pub Priority: AnimationPriority,
    pub WeightCurrent: f32,
    pub WeightTarget: f32,
}
#[derive(Debug, Clone)]
pub struct AnimationTrack {
    pub Animation: Ref,
    pub IsPlaying: bool,
    pub Length: f32,
    pub Looped: bool,
    pub Priority: AnimationPriority,
    pub Speed: f32,
    pub TimePosition: f32,
    pub WeightCurrent: f32,
    pub WeightTarget: f32,
}
#[derive(Debug, Clone)]
pub struct Animator {
    pub EvaluationThrottled: bool,
    pub PreferLodEnabled: bool,
    pub RootMotion: CFrame,
    pub RootMotionWeight: f32,
}
#[derive(Debug, Clone)]
pub struct Annotation {
    pub AuthorColor3: Color3,
    pub AuthorId: i64,
    pub ChannelId: String,
    pub Contents: String,
    pub CreationTimeUnix: i64,
    pub LastModifiedTimeUnix: i64,
    pub LoadingReplies: bool,
    pub MessageId: String,
    pub ReplyCount: i64,
    pub Resolved: bool,
    pub TaggedUsers: String,
}
#[derive(Debug, Clone)]
pub struct AnnotationsService {
    pub AnnotationsLoadingStatus: AnnotationRequestStatus,
    pub AnnotationsVisible: bool,
    pub Hovered: Ref,
    pub Mode: AnnotationEditingMode,
    pub ResolvedLoadingStatus: AnnotationRequestStatus,
    pub Selected: Ref,
}
#[derive(Debug, Clone)]
pub struct AppLifecycleObserverService {}
#[derive(Debug, Clone)]
pub struct AppStorageService {}
#[derive(Debug, Clone)]
pub struct AppUpdateService {}
#[derive(Debug, Clone)]
pub struct ArcHandles {
    pub Axes: Axes,
    pub MouseButton1DownConnectionCount: i32,
    pub MouseButton1UpConnectionCount: i32,
    pub MouseDragConnectionCount: i32,
    pub MouseEnterConnectionCount: i32,
    pub MouseLeaveConnectionCount: i32,
}
#[derive(Debug, Clone)]
pub struct AssetCounterService {}
#[derive(Debug, Clone)]
pub struct AssetDeliveryProxy {
    pub Interface: String,
    pub Port: i32,
    pub StartServer: bool,
}
#[derive(Debug, Clone)]
pub struct AssetImportService {}
#[derive(Debug, Clone)]
pub struct AssetImportSession {}
#[derive(Debug, Clone)]
pub struct AssetManagerService {}
#[derive(Debug, Clone)]
pub struct AssetPatchSettings {
    pub ContentId: String,
    pub OutputPath: String,
    pub PatchId: String,
}
#[derive(Debug, Clone)]
pub struct AssetService {}
#[derive(Debug, Clone)]
pub struct AssetSoundEffect {}
#[derive(Debug, Clone)]
pub struct Atmosphere {
    pub Color: Color3,
    pub Decay: Color3,
    pub Density: f32,
    pub Glare: f32,
    pub Haze: f32,
    pub Offset: f32,
}
#[derive(Debug, Clone)]
pub struct AtmosphereSensor {
    pub AirDensity: f32,
    pub RelativeWindVelocity: Vector3,
}
#[derive(Debug, Clone)]
pub struct Attachment {
    pub Axis: Vector3,
    pub CFrame: CFrame,
    pub Orientation: Vector3,
    pub Position: Vector3,
    pub Rotation: Vector3,
    pub SecondaryAxis: Vector3,
    pub Visible: bool,
    pub WorldAxis: Vector3,
    pub WorldCFrame: CFrame,
    pub WorldOrientation: Vector3,
    pub WorldPosition: Vector3,
    pub WorldRotation: Vector3,
    pub WorldSecondaryAxis: Vector3,
}
#[derive(Debug, Clone)]
pub struct AudioAnalyzer {
    pub PeakLevel: f32,
    pub RmsLevel: f32,
    pub SpectrumEnabled: bool,
    pub WindowSize: AudioWindowSize,
}
#[derive(Debug, Clone)]
pub struct AudioChannelMixer {
    pub Layout: AudioChannelLayout,
}
#[derive(Debug, Clone)]
pub struct AudioChannelSplitter {
    pub Layout: AudioChannelLayout,
}
#[derive(Debug, Clone)]
pub struct AudioChorus {
    pub Bypass: bool,
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
#[derive(Debug, Clone)]
pub struct AudioCompressor {
    pub Attack: f32,
    pub Bypass: bool,
    pub Editor: bool,
    pub MakeupGain: f32,
    pub Ratio: f32,
    pub Release: f32,
    pub Threshold: f32,
}
#[derive(Debug, Clone)]
pub struct AudioDeviceInput {
    pub AccessList: BinaryString,
    pub AccessType: AccessModifierType,
    pub Active: bool,
    pub IsReady: bool,
    pub Muted: bool,
    pub MutedByLocalUser: bool,
    pub Player: Ref,
    pub Volume: f32,
}
#[derive(Debug, Clone)]
pub struct AudioDeviceOutput {
    pub Player: Ref,
}
#[derive(Debug, Clone)]
pub struct AudioDistortion {
    pub Bypass: bool,
    pub Level: f32,
}
#[derive(Debug, Clone)]
pub struct AudioEcho {
    pub Bypass: bool,
    pub DelayTime: f32,
    pub DryLevel: f32,
    pub Feedback: f32,
    pub RampTime: f32,
    pub WetLevel: f32,
}
#[derive(Debug, Clone)]
pub struct AudioEmitter {
    pub AngleAttenuation: BinaryString,
    pub AudioInteractionGroup: String,
    pub DistanceAttenuation: BinaryString,
    pub SimulationFidelity: AudioSimulationFidelity,
}
#[derive(Debug, Clone)]
pub struct AudioEqualizer {
    pub Bypass: bool,
    pub Editor: bool,
    pub HighGain: f32,
    pub LowGain: f32,
    pub MidGain: f32,
    pub MidRange: NumberRange,
}
#[derive(Debug, Clone)]
pub struct AudioFader {
    pub Bypass: bool,
    pub Volume: f32,
}
#[derive(Debug, Clone)]
pub struct AudioFilter {
    pub Bypass: bool,
    pub Editor: bool,
    pub FilterType: AudioFilterType,
    pub Frequency: f32,
    pub Gain: f32,
    pub Q: f32,
}
#[derive(Debug, Clone)]
pub struct AudioFlanger {
    pub Bypass: bool,
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
#[derive(Debug, Clone)]
pub struct AudioFocusService {}
#[derive(Debug, Clone)]
pub struct AudioLimiter {
    pub Bypass: bool,
    pub Editor: bool,
    pub MaxLevel: f32,
    pub Release: f32,
}
#[derive(Debug, Clone)]
pub struct AudioListener {
    pub AngleAttenuation: BinaryString,
    pub AudioInteractionGroup: String,
    pub DistanceAttenuation: BinaryString,
    pub SimulationFidelity: AudioSimulationFidelity,
}
#[derive(Debug, Clone)]
pub struct AudioPages {}
#[derive(Debug, Clone)]
pub struct AudioPitchShifter {
    pub Bypass: bool,
    pub Pitch: f32,
    pub WindowSize: AudioWindowSize,
}
#[derive(Debug, Clone)]
pub struct AudioPlayer {
    pub Asset: ContentId,
    pub AssetId: String,
    pub AutoLoad: bool,
    pub IsPlaying: bool,
    pub IsReady: bool,
    pub LoopRegion: NumberRange,
    pub Looping: bool,
    pub PlaybackRegion: NumberRange,
    pub PlaybackSpeed: f64,
    pub TimeLength: f64,
    pub TimePosition: f64,
    pub Volume: f32,
}
#[derive(Debug, Clone)]
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
pub struct AudioSearchParams {
    pub Album: String,
    pub Artist: String,
    pub AudioSubType: AudioSubType,
    pub AudioSubtype: AudioSubType,
    pub MaxDuration: i32,
    pub MinDuration: i32,
    pub SearchKeyword: String,
    pub Tag: String,
    pub Title: String,
}
#[derive(Debug, Clone)]
pub struct AudioTextToSpeech {
    pub IsLoaded: bool,
    pub IsPlaying: bool,
    pub Looping: bool,
    pub Pitch: f32,
    pub PlaybackSpeed: f32,
    pub Speed: f32,
    pub Text: String,
    pub TimeLength: f64,
    pub TimePosition: f64,
    pub VoiceId: String,
    pub Volume: f32,
}
#[derive(Debug, Clone)]
pub struct AuroraScript {
    pub EnableCulling: bool,
    pub EnableLod: bool,
    pub LodCriticality: i32,
    pub Priority: i32,
    pub Source: String,
    pub Tag: String,
}
#[derive(Debug, Clone)]
pub struct AuroraScriptService {
    pub BufferSize: i32,
}
#[derive(Debug, Clone)]
pub struct AuroraService {}
#[derive(Debug, Clone)]
pub struct AvatarChatService {
    pub ClientFeatures: i32,
    pub ClientFeaturesInitialized: bool,
    pub ServerFeatures: i32,
}
#[derive(Debug, Clone)]
pub struct AvatarCreationService {}
#[derive(Debug, Clone)]
pub struct AvatarEditorService {}
#[derive(Debug, Clone)]
pub struct AvatarImportService {}
#[derive(Debug, Clone)]
pub struct AvatarPreloader {}
#[derive(Debug, Clone)]
pub struct Backpack {}
#[derive(Debug, Clone)]
pub struct BackpackItem {
    pub TextureId: ContentId,
}
#[derive(Debug, Clone)]
pub struct BadgeService {}
#[derive(Debug, Clone)]
pub struct BallSocketConstraint {
    pub LimitsEnabled: bool,
    pub MaxFrictionTorque: f32,
    pub MaxFrictionTorqueXml: f32,
    pub Radius: f32,
    pub Restitution: f32,
    pub TwistLimitsEnabled: bool,
    pub TwistLowerAngle: f32,
    pub TwistUpperAngle: f32,
    pub UpperAngle: f32,
}
#[derive(Debug, Clone)]
pub struct BanHistoryPages {}
#[derive(Debug, Clone)]
pub struct BaseImportData {
    pub Id: String,
    pub ImportName: String,
    pub ShouldImport: bool,
}
#[derive(Debug, Clone)]
pub struct BasePart {
    pub Anchored: bool,
    pub AssemblyAngularVelocity: Vector3,
    pub AssemblyCenterOfMass: Vector3,
    pub AssemblyLinearVelocity: Vector3,
    pub AssemblyMass: f32,
    pub AssemblyRootPart: Ref,
    pub AudioCanCollide: bool,
    pub BackParamA: f32,
    pub BackParamB: f32,
    pub BackSurface: SurfaceType,
    pub BackSurfaceInput: InputType,
    pub BottomParamA: f32,
    pub BottomParamB: f32,
    pub BottomSurface: SurfaceType,
    pub BottomSurfaceInput: InputType,
    pub BrickColor: BrickColor,
    pub BrickColor: BrickColor,
    pub CFrame: CFrame,
    pub CanCollide: bool,
    pub CanQuery: bool,
    pub CanTouch: bool,
    pub CastShadow: bool,
    pub CenterOfMass: Vector3,
    pub CollisionGroup: String,
    pub CollisionGroupId: i32,
    pub CollisionGroupReplicate: String,
    pub Color: Color3,
    pub Color3uint8: Color3uint8,
    pub CurrentPhysicalProperties: PhysicalProperties,
    pub CustomPhysicalProperties: PhysicalProperties,
    pub DraggingV1: bool,
    pub Elasticity: f32,
    pub EnableFluidForces: bool,
    pub ExtentsCFrame: CFrame,
    pub ExtentsSize: Vector3,
    pub Friction: f32,
    pub FrontParamA: f32,
    pub FrontParamB: f32,
    pub FrontSurface: SurfaceType,
    pub FrontSurfaceInput: InputType,
    pub LeftParamA: f32,
    pub LeftParamB: f32,
    pub LeftSurface: SurfaceType,
    pub LeftSurfaceInput: InputType,
    pub LocalSimulationValidation: i32,
    pub LocalTransparencyModifier: f32,
    pub Locked: bool,
    pub Mass: f32,
    pub Massless: bool,
    pub Material: Material,
    pub MaterialVariant: String,
    pub MaterialVariantSerialized: String,
    pub NetworkIsSleeping: bool,
    pub NetworkOwnershipRule: NetworkOwnership,
    pub Orientation: Vector3,
    pub PhysicsRepRootPart: Ref,
    pub PivotOffset: CFrame,
    pub Position: Vector3,
    pub ReceiveAge: f32,
    pub Reflectance: f32,
    pub ResizeIncrement: i32,
    pub ResizeableFaces: Faces,
    pub RightParamA: f32,
    pub RightParamB: f32,
    pub RightSurface: SurfaceType,
    pub RightSurfaceInput: InputType,
    pub RootPriority: i32,
    pub RotVelocity: Vector3,
    pub Rotation: Vector3,
    pub Siz: Vector3,
    pub Size: Vector3,
    pub Size: Vector3,
    pub SpecificGravity: f32,
    pub TopParamA: f32,
    pub TopParamB: f32,
    pub TopSurface: SurfaceType,
    pub TopSurfaceInput: InputType,
    pub Transparency: f32,
    pub Velocity: Vector3,
}
#[derive(Debug, Clone)]
pub struct BasePlayerGui {}
#[derive(Debug, Clone)]
pub struct BaseRemoteEvent {}
#[derive(Debug, Clone)]
pub struct BaseScript {
    pub Disabled: bool,
    pub Enabled: bool,
    pub LinkedSource: ContentId,
    pub RunContext: RunContext,
}
#[derive(Debug, Clone)]
pub struct BaseWrap {
    pub CageMeshContent: Content,
    pub CageMeshId: ContentId,
    pub CageOrigin: CFrame,
    pub CageOriginWorld: CFrame,
    pub HsrAssetId: ContentId,
    pub HsrData: SharedString,
    pub HsrMeshIdData: SharedString,
    pub ImportInProcess: bool,
    pub ImportOrigin: CFrame,
    pub ImportOriginWorld: CFrame,
    pub TemporaryCageMeshId: ContentId,
}
#[derive(Debug, Clone)]
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
    pub LocalTransparencyModifier: f32,
    pub Segments: i32,
    pub Texture: ContentId,
    pub TextureLength: f32,
    pub TextureMode: TextureMode,
    pub TextureSpeed: f32,
    pub Transparency: NumberSequence,
    pub Width0: f32,
    pub Width1: f32,
    pub ZOffset: f32,
}
#[derive(Debug, Clone)]
pub struct BevelMesh {
    pub Bevel: f32,
    pub BevelRoundness: f32,
    pub Bulge: f32,
}
#[derive(Debug, Clone)]
pub struct BillboardGui {
    pub Active: bool,
    pub Adornee: Ref,
    pub AlwaysOnTop: bool,
    pub Brightness: f32,
    pub ClipsDescendants: bool,
    pub CurrentDistance: f32,
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
pub struct BinaryStringValue {
    pub Value: BinaryString,
}
#[derive(Debug, Clone)]
pub struct BindableEvent {}
#[derive(Debug, Clone)]
pub struct BindableFunction {}
#[derive(Debug, Clone)]
pub struct BlockMesh {}
#[derive(Debug, Clone)]
pub struct BloomEffect {
    pub Intensity: f32,
    pub Size: f32,
    pub Threshold: f32,
}
#[derive(Debug, Clone)]
pub struct BlurEffect {
    pub Size: f32,
}
#[derive(Debug, Clone)]
pub struct BodyAngularVelocity {
    pub AngularVelocity: Vector3,
    pub Angularvelocity: Vector3,
    pub MaxTorque: Vector3,
    pub MaxTorque: Vector3,
    pub P: f32,
}
#[derive(Debug, Clone)]
pub struct BodyColors {
    pub HeadColor: BrickColor,
    pub HeadColor3: Color3,
    pub LeftArmColor: BrickColor,
    pub LeftArmColor3: Color3,
    pub LeftLegColor: BrickColor,
    pub LeftLegColor3: Color3,
    pub RightArmColor: BrickColor,
    pub RightArmColor3: Color3,
    pub RightLegColor: BrickColor,
    pub RightLegColor3: Color3,
    pub TorsoColor: BrickColor,
    pub TorsoColor3: Color3,
}
#[derive(Debug, Clone)]
pub struct BodyForce {
    pub Force: Vector3,
    pub Force: Vector3,
}
#[derive(Debug, Clone)]
pub struct BodyGyro {
    pub CFrame: CFrame,
    pub Cframe: CFrame,
    pub D: f32,
    pub MaxTorque: Vector3,
    pub MaxTorque: Vector3,
    pub P: f32,
}
#[derive(Debug, Clone)]
pub struct BodyMover {}
#[derive(Debug, Clone)]
pub struct BodyPartDescription {
    pub AssetId: i64,
    pub BodyPart: BodyPart,
    pub Color: Color3,
    pub Instance: Ref,
}
#[derive(Debug, Clone)]
pub struct BodyPosition {
    pub D: f32,
    pub MaxForce: Vector3,
    pub MaxForce: Vector3,
    pub P: f32,
    pub Position: Vector3,
    pub Position: Vector3,
}
#[derive(Debug, Clone)]
pub struct BodyThrust {
    pub Force: Vector3,
    pub Force: Vector3,
    pub Location: Vector3,
    pub Location: Vector3,
}
#[derive(Debug, Clone)]
pub struct BodyVelocity {
    pub MaxForce: Vector3,
    pub MaxForce: Vector3,
    pub P: f32,
    pub Velocity: Vector3,
    pub Velocity: Vector3,
}
#[derive(Debug, Clone)]
pub struct Bone {
    pub Transform: CFrame,
    pub TransformedCFrame: CFrame,
    pub TransformedWorldCFrame: CFrame,
}
#[derive(Debug, Clone)]
pub struct BoolValue {
    pub Value: bool,
}
#[derive(Debug, Clone)]
pub struct BoxHandleAdornment {
    pub Size: Vector3,
}
#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub Condition: String,
    pub ContinueExecution: bool,
    pub Enabled: bool,
    pub Id: i32,
    pub Line: i32,
    pub LogMessage: String,
    pub MetaBreakpointId: i32,
    pub RemoveOnHit: bool,
    pub Script: String,
    pub Valid: bool,
    pub Verified: bool,
}
#[derive(Debug, Clone)]
pub struct BrickColorValue {
    pub Value: BrickColor,
}
#[derive(Debug, Clone)]
pub struct BrowserService {}
#[derive(Debug, Clone)]
pub struct BubbleChatConfiguration {
    pub AdorneeName: String,
    pub BackgroundColor3: Color3,
    pub BackgroundTransparency: f64,
    pub BubbleDuration: f32,
    pub BubblesSpacing: f32,
    pub Enabled: bool,
    pub Font: Font,
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
pub struct BubbleChatMessageProperties {
    pub BackgroundColor3: Color3,
    pub BackgroundTransparency: f64,
    pub FontFace: Font,
    pub TailVisible: bool,
    pub TextColor3: Color3,
    pub TextSize: i64,
}
#[derive(Debug, Clone)]
pub struct BugReporterService {}
#[derive(Debug, Clone)]
pub struct BulkImportService {}
#[derive(Debug, Clone)]
pub struct BuoyancySensor {
    pub FullySubmerged: bool,
    pub TouchingSurface: bool,
}
#[derive(Debug, Clone)]
pub struct CFrameValue {
    pub Value: CFrame,
}
#[derive(Debug, Clone)]
pub struct CSGDictionaryService {}
#[derive(Debug, Clone)]
pub struct CacheableContentProvider {}
#[derive(Debug, Clone)]
pub struct CalloutService {}
#[derive(Debug, Clone)]
pub struct Camera {
    pub CFrame: CFrame,
    pub CameraSubject: Ref,
    pub CameraType: CameraType,
    pub CoordinateFrame: CFrame,
    pub DiagonalFieldOfView: f32,
    pub FieldOfView: f32,
    pub FieldOfViewMode: FieldOfViewMode,
    pub Focus: CFrame,
    pub Focus: CFrame,
    pub HeadLocked: bool,
    pub HeadScale: f32,
    pub MaxAxisFieldOfView: f32,
    pub NearPlaneZ: f32,
    pub ViewportSize: Vector2,
    pub VrTiltAndRollEnabled: bool,
}
#[derive(Debug, Clone)]
pub struct CanvasGroup {
    pub GroupColor3: Color3,
    pub GroupTransparency: f32,
    pub ResolutionScale: f32,
}
#[derive(Debug, Clone)]
pub struct Capture {}
#[derive(Debug, Clone)]
pub struct CaptureService {}
#[derive(Debug, Clone)]
pub struct CatalogPages {}
#[derive(Debug, Clone)]
pub struct ChangeHistoryService {}
#[derive(Debug, Clone)]
pub struct ChannelSelectorSoundEffect {
    pub Channel: i32,
}
#[derive(Debug, Clone)]
pub struct ChannelTabsConfiguration {
    pub AbsolutePosition: Vector2,
    pub AbsoluteSize: Vector2,
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
pub struct CharacterAppearance {}
#[derive(Debug, Clone)]
pub struct CharacterMesh {
    pub BaseTextureId: i64,
    pub BodyPart: BodyPart,
    pub MeshId: i64,
    pub OverlayTextureId: i64,
}
#[derive(Debug, Clone)]
pub struct Chat {
    pub BubbleChatEnabled: bool,
    pub IsAutoMigrated: bool,
    pub LoadDefaultChat: bool,
}
#[derive(Debug, Clone)]
pub struct ChatInputBarConfiguration {
    pub AbsolutePosition: Vector2,
    pub AbsolutePositionWrite: Vector2,
    pub AbsoluteSize: Vector2,
    pub AbsoluteSizeWrite: Vector2,
    pub AutocompleteEnabled: bool,
    pub BackgroundColor3: Color3,
    pub BackgroundTransparency: f64,
    pub Enabled: bool,
    pub FontFace: Font,
    pub IsFocused: bool,
    pub IsFocusedWrite: bool,
    pub KeyboardKeyCode: KeyCode,
    pub PlaceholderColor3: Color3,
    pub TargetTextChannel: Ref,
    pub TextBox: Ref,
    pub TextColor3: Color3,
    pub TextSize: i64,
    pub TextStrokeColor3: Color3,
    pub TextStrokeTransparency: f64,
}
#[derive(Debug, Clone)]
pub struct ChatWindowConfiguration {
    pub AbsolutePosition: Vector2,
    pub AbsolutePositionWrite: Vector2,
    pub AbsoluteSize: Vector2,
    pub AbsoluteSizeWrite: Vector2,
    pub BackgroundColor3: Color3,
    pub BackgroundTransparency: f64,
    pub Enabled: bool,
    pub FontFace: Font,
    pub HeightScale: f32,
    pub HorizontalAlignment: HorizontalAlignment,
    pub TextColor3: Color3,
    pub TextSize: i64,
    pub TextStrokeColor3: Color3,
    pub TextStrokeTransparency: f64,
    pub VerticalAlignment: VerticalAlignment,
    pub WidthScale: f32,
}
#[derive(Debug, Clone)]
pub struct ChatWindowMessageProperties {
    pub FontFace: Font,
    pub PrefixTextProperties: Ref,
    pub TextColor3: Color3,
    pub TextSize: i32,
    pub TextStrokeColor3: Color3,
    pub TextStrokeTransparency: f64,
}
#[derive(Debug, Clone)]
pub struct ChatbotUIService {}
#[derive(Debug, Clone)]
pub struct ChorusSoundEffect {
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
#[derive(Debug, Clone)]
pub struct ClickDetector {
    pub CursorIcon: ContentId,
    pub MaxActivationDistance: f32,
}
#[derive(Debug, Clone)]
pub struct ClientReplicator {}
#[derive(Debug, Clone)]
pub struct ClimbController {
    pub AccelerationTime: f32,
    pub BalanceMaxTorque: f32,
    pub BalanceSpeed: f32,
    pub MoveMaxForce: f32,
}
#[derive(Debug, Clone)]
pub struct Clothing {
    pub Color3: Color3,
    pub Outfit1: ContentId,
    pub Outfit2: ContentId,
}
#[derive(Debug, Clone)]
pub struct CloudCRUDService {}
#[derive(Debug, Clone)]
pub struct CloudLocalizationTable {}
#[derive(Debug, Clone)]
pub struct Clouds {
    pub Color: Color3,
    pub Cover: f32,
    pub Density: f32,
    pub Enabled: bool,
}
#[derive(Debug, Clone)]
pub struct ClusterPacketCache {}
#[derive(Debug, Clone)]
pub struct Collaborator {
    pub CFrame: CFrame,
    pub CollaboratorColor: i32,
    pub CollaboratorColor3: Color3,
    pub CurDocGuid: String,
    pub CurScriptLineNumber: i32,
    pub IsIdle: bool,
    pub Status: CollaboratorStatus,
    pub UserId: i64,
    pub Username: String,
}
#[derive(Debug, Clone)]
pub struct CollaboratorsService {}
#[derive(Debug, Clone)]
pub struct CollectionService {}
#[derive(Debug, Clone)]
pub struct Color3Value {
    pub Value: Color3,
}
#[derive(Debug, Clone)]
pub struct ColorCorrectionEffect {
    pub Brightness: f32,
    pub Contrast: f32,
    pub Saturation: f32,
    pub TintColor: Color3,
}
#[derive(Debug, Clone)]
pub struct ColorGradingEffect {
    pub TonemapperPreset: TonemapperPreset,
}
#[derive(Debug, Clone)]
pub struct CommandInstance {
    pub AllowGuiAccessPoints: bool,
    pub Checked: bool,
    pub DefaultShortcut: String,
    pub DisplayName: String,
    pub Enabled: bool,
    pub Icon: String,
    pub Name: String,
    pub Permission: CommandPermission,
    pub StatusTip: String,
}
#[derive(Debug, Clone)]
pub struct CommandService {}
#[derive(Debug, Clone)]
pub struct CommerceService {}
#[derive(Debug, Clone)]
pub struct CompressorSoundEffect {
    pub Attack: f32,
    pub GainMakeup: f32,
    pub Ratio: f32,
    pub Release: f32,
    pub SideChain: Ref,
    pub Threshold: f32,
}
#[derive(Debug, Clone)]
pub struct ConeHandleAdornment {
    pub Height: f32,
    pub Radius: f32,
}
#[derive(Debug, Clone)]
pub struct ConfigService {}
#[derive(Debug, Clone)]
pub struct ConfigSnapshot {
    pub Error: ConfigSnapshotErrorState,
    pub Outdated: bool,
}
#[derive(Debug, Clone)]
pub struct Configuration {}
#[derive(Debug, Clone)]
pub struct ConfigureServerService {}
#[derive(Debug, Clone)]
pub struct ConnectivityService {
    pub NetworkStatus: NetworkStatus,
}
#[derive(Debug, Clone)]
pub struct Constraint {
    pub Active: bool,
    pub Attachment0: Ref,
    pub Attachment1: Ref,
    pub Color: BrickColor,
    pub Enabled: bool,
    pub Visible: bool,
}
#[derive(Debug, Clone)]
pub struct ContentProvider {
    pub BaseUrl: String,
    pub RequestQueueSize: i32,
}
#[derive(Debug, Clone)]
pub struct ContextActionService {}
#[derive(Debug, Clone)]
pub struct Controller {}
#[derive(Debug, Clone)]
pub struct ControllerBase {
    pub Active: bool,
    pub BalanceRigidityEnabled: bool,
    pub MoveSpeedFactor: f32,
}
#[derive(Debug, Clone)]
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
pub struct ControllerPartSensor {
    pub HitFrame: CFrame,
    pub HitNormal: Vector3,
    pub SearchDistance: f32,
    pub SensedPart: Ref,
    pub SensorMode: SensorMode,
}
#[derive(Debug, Clone)]
pub struct ControllerSensor {}
#[derive(Debug, Clone)]
pub struct ControllerService {}
#[derive(Debug, Clone)]
pub struct ConversationalAIAcceptanceService {}
#[derive(Debug, Clone)]
pub struct CookiesService {}
#[derive(Debug, Clone)]
pub struct CoreGui {
    pub SelectionImageObject: Ref,
    pub Version: i32,
}
#[derive(Debug, Clone)]
pub struct CorePackages {}
#[derive(Debug, Clone)]
pub struct CoreScript {}
#[derive(Debug, Clone)]
pub struct CoreScriptDebuggingManagerHelper {}
#[derive(Debug, Clone)]
pub struct CoreScriptSyncService {}
#[derive(Debug, Clone)]
pub struct CornerWedgePart {}
#[derive(Debug, Clone)]
pub struct CreationDBService {}
#[derive(Debug, Clone)]
pub struct CreatorStoreService {}
#[derive(Debug, Clone)]
pub struct CrossDMScriptChangeListener {}
#[derive(Debug, Clone)]
pub struct CurveAnimation {}
#[derive(Debug, Clone)]
pub struct CustomEvent {
    pub PersistedCurrentValue: f32,
}
#[derive(Debug, Clone)]
pub struct CustomEventReceiver {
    pub Source: Ref,
}
#[derive(Debug, Clone)]
pub struct CustomLog {}
#[derive(Debug, Clone)]
pub struct CustomSoundEffect {}
#[derive(Debug, Clone)]
pub struct CylinderHandleAdornment {
    pub Angle: f32,
    pub Height: f32,
    pub InnerRadius: f32,
    pub Radius: f32,
}
#[derive(Debug, Clone)]
pub struct CylinderMesh {}
#[derive(Debug, Clone)]
pub struct CylindricalConstraint {
    pub AngularActuatorType: ActuatorType,
    pub AngularLimitsEnabled: bool,
    pub AngularResponsiveness: f32,
    pub AngularRestitution: f32,
    pub AngularSpeed: f32,
    pub AngularVelocity: f32,
    pub CurrentAngle: f32,
    pub InclinationAngle: f32,
    pub LowerAngle: f32,
    pub MotorMaxAngularAcceleration: f32,
    pub MotorMaxTorque: f32,
    pub RotationAxisVisible: bool,
    pub ServoMaxTorque: f32,
    pub SoftlockAngularServoUponReachingTarget: bool,
    pub TargetAngle: f32,
    pub UpperAngle: f32,
    pub WorldRotationAxis: Vector3,
}
#[derive(Debug, Clone)]
pub struct DataModel {
    pub CreatorId: i64,
    pub CreatorType: CreatorType,
    pub Environment: String,
    pub ForceR15: bool,
    pub GameAvatarType: GameAvatarType,
    pub GameId: i64,
    pub GearGenreSetting: GearGenreSetting,
    pub Genre: Genre,
    pub IsSfFlagsLoaded: bool,
    pub JobId: String,
    pub Lighting: Ref,
    pub MatchmakingType: MatchmakingType,
    pub PlaceId: i64,
    pub PlaceVersion: i32,
    pub PrivateServerId: String,
    pub PrivateServerOwnerId: i64,
    pub R15CollisionType: R15CollisionType,
    pub VipServerId: String,
    pub VipServerOwnerId: i64,
    pub Workspace: Ref,
    pub Workspace: Ref,
}
#[derive(Debug, Clone)]
pub struct DataModelMesh {
    pub Offset: Vector3,
    pub Scale: Vector3,
    pub VertexColor: Vector3,
}
#[derive(Debug, Clone)]
pub struct DataModelPatchService {}
#[derive(Debug, Clone)]
pub struct DataModelSession {
    pub CurrentDataModelType: StudioDataModelType,
    pub SessionId: String,
}
#[derive(Debug, Clone)]
pub struct DataStore {}
#[derive(Debug, Clone)]
pub struct DataStoreGetOptions {
    pub UseCache: bool,
}
#[derive(Debug, Clone)]
pub struct DataStoreIncrementOptions {}
#[derive(Debug, Clone)]
pub struct DataStoreInfo {
    pub CreatedTime: i64,
    pub DataStoreName: String,
    pub UpdatedTime: i64,
}
#[derive(Debug, Clone)]
pub struct DataStoreKey {
    pub KeyName: String,
}
#[derive(Debug, Clone)]
pub struct DataStoreKeyInfo {
    pub CreatedTime: i64,
    pub UpdatedTime: i64,
    pub Version: String,
}
#[derive(Debug, Clone)]
pub struct DataStoreKeyPages {
    pub Cursor: String,
}
#[derive(Debug, Clone)]
pub struct DataStoreListingPages {
    pub Cursor: String,
}
#[derive(Debug, Clone)]
pub struct DataStoreObjectVersionInfo {
    pub CreatedTime: i64,
    pub IsDeleted: bool,
    pub Version: String,
}
#[derive(Debug, Clone)]
pub struct DataStoreOptions {
    pub AllScopes: bool,
}
#[derive(Debug, Clone)]
pub struct DataStorePages {}
#[derive(Debug, Clone)]
pub struct DataStoreService {
    pub AutomaticRetry: bool,
    pub LegacyNamingScheme: bool,
}
#[derive(Debug, Clone)]
pub struct DataStoreSetOptions {}
#[derive(Debug, Clone)]
pub struct DataStoreVersionPages {}
#[derive(Debug, Clone)]
pub struct Debris {
    pub MaxItems: i32,
}
#[derive(Debug, Clone)]
pub struct DebugSettings {
    pub DataModel: i32,
    pub InstanceCount: i32,
    pub IsScriptStackTracingEnabled: bool,
    pub JobCount: i32,
    pub PlayerCount: i32,
    pub ReportSoundWarnings: bool,
    pub RobloxVersion: String,
    pub TickCountPreciseOverride: TickCountSampleMethod,
}
#[derive(Debug, Clone)]
pub struct DebuggablePluginWatcher {}
#[derive(Debug, Clone)]
pub struct DebuggerBreakpoint {
    pub Condition: String,
    pub ContinueExecution: bool,
    pub IsContextDependentBreakpoint: bool,
    pub IsEnabled: bool,
    pub Line: i32,
    pub Line: i32,
    pub LogExpression: String,
}
#[derive(Debug, Clone)]
pub struct DebuggerConnection {
    pub ErrorMessage: String,
    pub HasError: bool,
    pub Id: i32,
    pub IsPaused: bool,
}
#[derive(Debug, Clone)]
pub struct DebuggerConnectionManager {
    pub Timeout: f64,
}
#[derive(Debug, Clone)]
pub struct DebuggerLuaResponse {
    pub IsError: bool,
    pub IsSuccess: bool,
    pub Message: String,
    pub RequestId: i32,
    pub Status: DebuggerStatus,
}
#[derive(Debug, Clone)]
pub struct DebuggerManager {
    pub DebuggingEnabled: bool,
}
#[derive(Debug, Clone)]
pub struct DebuggerUIService {}
#[derive(Debug, Clone)]
pub struct DebuggerVariable {
    pub Name: String,
    pub Populated: bool,
    pub Type: String,
    pub Value: String,
    pub VariableId: i32,
    pub VariablesCount: i32,
}
#[derive(Debug, Clone)]
pub struct DebuggerWatch {
    pub Expression: String,
}
#[derive(Debug, Clone)]
pub struct Decal {
    pub Color3: Color3,
    pub LocalTransparencyModifier: f32,
    pub Shiny: f32,
    pub Specular: f32,
    pub Texture: ContentId,
    pub TextureContent: Content,
    pub Transparency: f32,
    pub ZIndex: i32,
}
#[derive(Debug, Clone)]
pub struct DepthOfFieldEffect {
    pub FarIntensity: f32,
    pub FocusDistance: f32,
    pub InFocusRadius: f32,
    pub NearIntensity: f32,
}
#[derive(Debug, Clone)]
pub struct DeviceIdService {}
#[derive(Debug, Clone)]
pub struct Dialog {
    pub BehaviorType: DialogBehaviorType,
    pub ConversationDistance: f32,
    pub GoodbyeChoiceActive: bool,
    pub GoodbyeDialog: String,
    pub InUse: bool,
    pub InitialPrompt: String,
    pub Purpose: DialogPurpose,
    pub Tone: DialogTone,
    pub TriggerDistance: f32,
    pub TriggerOffset: Vector3,
}
#[derive(Debug, Clone)]
pub struct DialogChoice {
    pub GoodbyeChoiceActive: bool,
    pub GoodbyeDialog: String,
    pub ResponseDialog: String,
    pub UserDialog: String,
}
#[derive(Debug, Clone)]
pub struct DistortionSoundEffect {
    pub Level: f32,
}
#[derive(Debug, Clone)]
pub struct DockWidgetPluginGui {
    pub HostWidgetWasRestored: bool,
}
#[derive(Debug, Clone)]
pub struct DoubleConstrainedValue {
    pub ConstrainedValue: f64,
    pub MaxValue: f64,
    pub MinValue: f64,
    pub Value: f64,
    pub Value: f64,
}
#[derive(Debug, Clone)]
pub struct DraftsService {}
#[derive(Debug, Clone)]
pub struct DragDetector {
    pub ActivatedCursorIcon: ContentId,
    pub ApplyAtCenterOfMass: bool,
    pub Axis: Vector3,
    pub DragFrame: CFrame,
    pub DragStyle: DragDetectorDragStyle,
    pub Enabled: bool,
    pub GamepadModeSwitchKeyCode: KeyCode,
    pub KeyboardModeSwitchKeyCode: KeyCode,
    pub MaxDragAngle: f32,
    pub MaxDragTranslation: Vector3,
    pub MaxForce: f32,
    pub MaxTorque: f32,
    pub MinDragAngle: f32,
    pub MinDragTranslation: Vector3,
    pub Orientation: Vector3,
    pub PermissionPolicy: DragDetectorPermissionPolicy,
    pub PhysicalDragClickedPart: Ref,
    pub PhysicalDragHitPoint: Vector3,
    pub PhysicalDragIsInVr: bool,
    pub PhysicalDragTargetFrame: CFrame,
    pub ReferenceInstance: Ref,
    pub ResponseStyle: DragDetectorResponseStyle,
    pub Responsiveness: f32,
    pub RunLocally: bool,
    pub SecondaryAxis: Vector3,
    pub TrackballRadialPullFactor: f32,
    pub TrackballRollFactor: f32,
    pub VrSwitchKeyCode: KeyCode,
    pub WorldAxis: Vector3,
    pub WorldSecondaryAxis: Vector3,
}
#[derive(Debug, Clone)]
pub struct Dragger {}
#[derive(Debug, Clone)]
pub struct DraggerService {
    pub AlignDraggedObjects: bool,
    pub AngleSnapEnabled: bool,
    pub AngleSnapIncrement: f32,
    pub AnimateHover: bool,
    pub CollisionsEnabled: bool,
    pub DraggerCoordinateSpace: DraggerCoordinateSpace,
    pub DraggerMovementMode: DraggerMovementMode,
    pub GeometrySnapColor: Color3,
    pub HoverAnimateFrequency: f32,
    pub HoverLineThickness: i32,
    pub HoverThickness: f32,
    pub JointsEnabled: bool,
    pub LinearSnapEnabled: bool,
    pub LinearSnapIncrement: f32,
    pub PartSnapEnabled: bool,
    pub PivotSnapToGeometry: bool,
    pub ShowHover: bool,
    pub ShowPivotIndicator: bool,
}
#[derive(Debug, Clone)]
pub struct DynamicRotate {
    pub BaseAngle: f32,
}
#[derive(Debug, Clone)]
pub struct EchoSoundEffect {
    pub Delay: f32,
    pub DryLevel: f32,
    pub Feedback: f32,
    pub WetLevel: f32,
}
#[derive(Debug, Clone)]
pub struct EditableImage {
    pub ImageData: BinaryString,
    pub IsReplicatedCopy: bool,
    pub Size: Vector2,
}
#[derive(Debug, Clone)]
pub struct EditableMesh {
    pub FixedSize: bool,
    pub IsReplicatedCopy: bool,
    pub MeshData: SharedString,
    pub SkinningEnabled: bool,
}
#[derive(Debug, Clone)]
pub struct EditableService {
    pub EditableStatus: EditableStatus,
}
#[derive(Debug, Clone)]
pub struct EmotesPages {}
#[derive(Debug, Clone)]
pub struct EqualizerSoundEffect {
    pub HighGain: f32,
    pub LowGain: f32,
    pub MidGain: f32,
}
#[derive(Debug, Clone)]
pub struct EulerRotationCurve {
    pub RotationOrder: RotationOrder,
}
#[derive(Debug, Clone)]
pub struct EventIngestService {}
#[derive(Debug, Clone)]
pub struct ExampleService {}
#[derive(Debug, Clone)]
pub struct ExperienceAuthService {}
#[derive(Debug, Clone)]
pub struct ExperienceInviteOptions {
    pub InviteMessageId: String,
    pub InviteUser: i64,
    pub LaunchData: String,
    pub PromptMessage: String,
}
#[derive(Debug, Clone)]
pub struct ExperienceNotificationService {}
#[derive(Debug, Clone)]
pub struct ExperienceService {}
#[derive(Debug, Clone)]
pub struct ExperienceStateCaptureService {
    pub HiddenSelectionEnabled: bool,
    pub IsInBackground: bool,
    pub IsInCaptureMode: bool,
}
#[derive(Debug, Clone)]
pub struct ExplorerFilter {}
#[derive(Debug, Clone)]
pub struct ExplorerFilterAutocompleter {
    pub ReplaceRange: Vector2,
    pub RequiresOutsideContext: bool,
}
#[derive(Debug, Clone)]
pub struct ExplorerServiceVisibilityService {}
#[derive(Debug, Clone)]
pub struct Explosion {
    pub BlastPressure: f32,
    pub BlastRadius: f32,
    pub DestroyJointRadiusPercent: f32,
    pub ExplosionType: ExplosionType,
    pub LocalTransparencyModifier: f32,
    pub Position: Vector3,
    pub TimeScale: f32,
    pub Visible: bool,
}
#[derive(Debug, Clone)]
pub struct FaceAnimatorService {
    pub AudioAnimationEnabled: bool,
    pub FaceTrackingStatusEnum: TrackerFaceTrackingStatus,
    pub FlipHeadOrientation: bool,
    pub VideoAnimationEnabled: bool,
}
#[derive(Debug, Clone)]
pub struct FaceControls {
    pub ChinRaiser: f32,
    pub ChinRaiserUpperLip: f32,
    pub Corrugator: f32,
    pub EyesLookDown: f32,
    pub EyesLookLeft: f32,
    pub EyesLookRight: f32,
    pub EyesLookUp: f32,
    pub FlatPucker: f32,
    pub Funneler: f32,
    pub JawDrop: f32,
    pub JawLeft: f32,
    pub JawRight: f32,
    pub LeftBrowLowerer: f32,
    pub LeftCheekPuff: f32,
    pub LeftCheekRaiser: f32,
    pub LeftDimpler: f32,
    pub LeftEyeClosed: f32,
    pub LeftEyeUpperLidRaiser: f32,
    pub LeftInnerBrowRaiser: f32,
    pub LeftLipCornerDown: f32,
    pub LeftLipCornerPuller: f32,
    pub LeftLipStretcher: f32,
    pub LeftLowerLipDepressor: f32,
    pub LeftNoseWrinkler: f32,
    pub LeftOuterBrowRaiser: f32,
    pub LeftUpperLipRaiser: f32,
    pub LipPresser: f32,
    pub LipsTogether: f32,
    pub LowerLipSuck: f32,
    pub MouthLeft: f32,
    pub MouthRight: f32,
    pub Pucker: f32,
    pub RightBrowLowerer: f32,
    pub RightCheekPuff: f32,
    pub RightCheekRaiser: f32,
    pub RightDimpler: f32,
    pub RightEyeClosed: f32,
    pub RightEyeUpperLidRaiser: f32,
    pub RightInnerBrowRaiser: f32,
    pub RightLipCornerDown: f32,
    pub RightLipCornerPuller: f32,
    pub RightLipStretcher: f32,
    pub RightLowerLipDepressor: f32,
    pub RightNoseWrinkler: f32,
    pub RightOuterBrowRaiser: f32,
    pub RightUpperLipRaiser: f32,
    pub TongueDown: f32,
    pub TongueOut: f32,
    pub TongueUp: f32,
    pub UpperLipSuck: f32,
}
#[derive(Debug, Clone)]
pub struct FaceInstance {
    pub Face: NormalId,
}
#[derive(Debug, Clone)]
pub struct FacialAgeEstimationService {}
#[derive(Debug, Clone)]
pub struct FacialAnimationRecordingService {
    pub BiometricDataConsent: bool,
}
#[derive(Debug, Clone)]
pub struct FacialAnimationStreamingServiceStats {}
#[derive(Debug, Clone)]
pub struct FacialAnimationStreamingServiceV2 {
    pub ServiceState: i32,
}
#[derive(Debug, Clone)]
pub struct FacialAnimationStreamingSubsessionStats {}
#[derive(Debug, Clone)]
pub struct FacsImportData {}
#[derive(Debug, Clone)]
pub struct Feature {
    pub FaceId: NormalId,
    pub InOut: InOut,
    pub LeftRight: LeftRight,
    pub TopBottom: TopBottom,
}
#[derive(Debug, Clone)]
pub struct FeatureRestrictionManager {}
#[derive(Debug, Clone)]
pub struct FeedPages {}
#[derive(Debug, Clone)]
pub struct FeedService {}
#[derive(Debug, Clone)]
pub struct File {
    pub Size: i64,
}
#[derive(Debug, Clone)]
pub struct FileMesh {
    pub MeshId: ContentId,
    pub TextureId: ContentId,
}
#[derive(Debug, Clone)]
pub struct Fire {
    pub Color: Color3,
    pub Enabled: bool,
    pub Heat: f32,
    pub HeatXml: f32,
    pub LocalTransparencyModifier: f32,
    pub SecondaryColor: Color3,
    pub Size: f32,
    pub Size: f32,
    pub SizeXml: f32,
    pub TimeScale: f32,
}
#[derive(Debug, Clone)]
pub struct Flag {
    pub TeamColor: BrickColor,
}
#[derive(Debug, Clone)]
pub struct FlagStand {
    pub TeamColor: BrickColor,
}
#[derive(Debug, Clone)]
pub struct FlagStandService {}
#[derive(Debug, Clone)]
pub struct FlangeSoundEffect {
    pub Depth: f32,
    pub Mix: f32,
    pub Rate: f32,
}
#[derive(Debug, Clone)]
pub struct FloatCurve {
    pub Length: i32,
    pub ValuesAndTimes: BinaryString,
}
#[derive(Debug, Clone)]
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
pub struct FluidForceSensor {
    pub CenterOfPressure: Vector3,
    pub Force: Vector3,
    pub Torque: Vector3,
}
#[derive(Debug, Clone)]
pub struct FlyweightService {}
#[derive(Debug, Clone)]
pub struct Folder {
    pub ReplicatedGuiInsertionOrder: i32,
}
#[derive(Debug, Clone)]
pub struct ForceField {
    pub Visible: bool,
}
#[derive(Debug, Clone)]
pub struct FormFactorPart {
    pub FormFactor: FormFactor,
    pub FormFactor: FormFactor,
    pub FormFactorRaw: FormFactor,
}
#[derive(Debug, Clone)]
pub struct Frame {
    pub Style: FrameStyle,
}
#[derive(Debug, Clone)]
pub struct FriendPages {}
#[derive(Debug, Clone)]
pub struct FriendService {}
#[derive(Debug, Clone)]
pub struct FunctionalTest {
    pub AllowSleep: bool,
    pub Description: String,
    pub HasMigratedSettingsToTestService: bool,
    pub Is30FpsThrottleEnabled: bool,
    pub PhysicsEnvironmentalThrottle: bool,
    pub Timeout: f64,
}
#[derive(Debug, Clone)]
pub struct GamePassService {}
#[derive(Debug, Clone)]
pub struct GameSettings {
    pub VideoCaptureEnabled: bool,
    pub VideoRecording: bool,
}
#[derive(Debug, Clone)]
pub struct GamepadService {
    pub GamepadCursorEnabled: bool,
}
#[derive(Debug, Clone)]
pub struct GenerationService {}
#[derive(Debug, Clone)]
pub struct GenericChallengeService {}
#[derive(Debug, Clone)]
pub struct GenericSettings {}
#[derive(Debug, Clone)]
pub struct Geometry {}
#[derive(Debug, Clone)]
pub struct GeometryService {}
#[derive(Debug, Clone)]
pub struct GetTextBoundsParams {
    pub Font: Font,
    pub RichText: bool,
    pub Size: f32,
    pub Text: String,
    pub Width: f32,
}
#[derive(Debug, Clone)]
pub struct GlobalDataStore {}
#[derive(Debug, Clone)]
pub struct GlobalSettings {}
#[derive(Debug, Clone)]
pub struct Glue {
    pub F0: Vector3,
    pub F1: Vector3,
    pub F2: Vector3,
    pub F3: Vector3,
}
#[derive(Debug, Clone)]
pub struct GoogleAnalyticsConfiguration {
    pub GaId: String,
}
#[derive(Debug, Clone)]
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
pub struct GroupImportData {
    pub Anchored: bool,
    pub ImportAsModelAsset: bool,
    pub InsertInWorkspace: bool,
}
#[derive(Debug, Clone)]
pub struct GroupService {}
#[derive(Debug, Clone)]
pub struct GuiBase {}
#[derive(Debug, Clone)]
pub struct GuiBase2d {
    pub AbsolutePosition: Vector2,
    pub AbsoluteRotation: f32,
    pub AbsoluteSize: Vector2,
    pub AutoLocalize: bool,
    pub ClippedRect: Rect,
    pub IsNotOccluded: bool,
    pub Localize: bool,
    pub RawRect2D: Rect,
    pub ReplicatedInsertionOrder: i32,
    pub RootLocalizationTable: Ref,
    pub SelectionBehaviorDown: SelectionBehavior,
    pub SelectionBehaviorLeft: SelectionBehavior,
    pub SelectionBehaviorRight: SelectionBehavior,
    pub SelectionBehaviorUp: SelectionBehavior,
    pub SelectionGroup: bool,
    pub TotalGroupScale: f32,
}
#[derive(Debug, Clone)]
pub struct GuiBase3d {
    pub Color: BrickColor,
    pub Color3: Color3,
    pub Transparency: f32,
    pub Visible: bool,
}
#[derive(Debug, Clone)]
pub struct GuiButton {
    pub AutoButtonColor: bool,
    pub HoverHapticEffect: Ref,
    pub Modal: bool,
    pub MouseButton1ClickConnectionCount: i32,
    pub MouseButton1DownConnectionCount: i32,
    pub MouseButton1UpConnectionCount: i32,
    pub MouseButton2ClickConnectionCount: i32,
    pub MouseButton2DownConnectionCount: i32,
    pub MouseButton2UpConnectionCount: i32,
    pub PressHapticEffect: Ref,
    pub Selected: bool,
    pub Style: ButtonStyle,
}
#[derive(Debug, Clone)]
pub struct GuiLabel {}
#[derive(Debug, Clone)]
pub struct GuiMain {}
#[derive(Debug, Clone)]
pub struct GuiObject {
    pub Active: bool,
    pub AnchorPoint: Vector2,
    pub AutomaticSize: AutomaticSize,
    pub BackgroundColor: BrickColor,
    pub BackgroundColor3: Color3,
    pub BackgroundTransparency: f32,
    pub BorderColor: BrickColor,
    pub BorderColor3: Color3,
    pub BorderMode: BorderMode,
    pub BorderSizePixel: i32,
    pub ClipsDescendants: bool,
    pub DragBeginConnectionCount: i32,
    pub DragStoppedConnectionCount: i32,
    pub Draggable: bool,
    pub GuiState: GuiState,
    pub Interactable: bool,
    pub LayoutOrder: i32,
    pub MouseEnterConnectionCount: i32,
    pub MouseLeaveConnectionCount: i32,
    pub MouseMovedConnectionCount: i32,
    pub MouseWheelBackwardConnectionCount: i32,
    pub MouseWheelForwardConnectionCount: i32,
    pub NextSelectionDown: Ref,
    pub NextSelectionLeft: Ref,
    pub NextSelectionRight: Ref,
    pub NextSelectionUp: Ref,
    pub Position: UDim2,
    pub Rotation: f32,
    pub Selectable: bool,
    pub SelectionImageObject: Ref,
    pub SelectionOrder: i32,
    pub SelectionRect2D: Rect,
    pub Size: UDim2,
    pub SizeConstraint: SizeConstraint,
    pub Transparency: f32,
    pub Visible: bool,
    pub ZIndex: i32,
}
#[derive(Debug, Clone)]
pub struct GuiService {
    pub AutoSelectGuiEnabled: bool,
    pub CoreEffectFolder: Ref,
    pub CoreGuiFolder: Ref,
    pub CoreGuiNavigationEnabled: bool,
    pub GuiNavigationEnabled: bool,
    pub IsModalDialog: bool,
    pub IsWindows: bool,
    pub MenuIsOpen: bool,
    pub PreferredTextSize: PreferredTextSize,
    pub PreferredTransparency: f32,
    pub ReducedMotionEnabled: bool,
    pub SelectedCoreObject: Ref,
    pub SelectedObject: Ref,
    pub TopbarInset: Rect,
    pub TouchControlsEnabled: bool,
}
#[derive(Debug, Clone)]
pub struct GuidRegistryService {}
#[derive(Debug, Clone)]
pub struct HSRDataContentProvider {}
#[derive(Debug, Clone)]
pub struct HandleAdornment {
    pub AdornCullingMode: AdornCullingMode,
    pub AlwaysOnTop: bool,
    pub CFrame: CFrame,
    pub SizeRelativeOffset: Vector3,
    pub ZIndex: i32,
}
#[derive(Debug, Clone)]
pub struct Handles {
    pub Faces: Faces,
    pub MouseButton1DownConnectionCount: i32,
    pub MouseButton1UpConnectionCount: i32,
    pub MouseDragConnectionCount: i32,
    pub MouseEnterConnectionCount: i32,
    pub MouseLeaveConnectionCount: i32,
    pub Style: HandlesStyle,
}
#[derive(Debug, Clone)]
pub struct HandlesBase {}
#[derive(Debug, Clone)]
pub struct HapticEffect {
    pub Looped: bool,
    pub Position: Vector3,
    pub Radius: f32,
    pub Type: HapticEffectType,
    pub Waveform: Ref,
}
#[derive(Debug, Clone)]
pub struct HapticService {}
#[derive(Debug, Clone)]
pub struct Hat {}
#[derive(Debug, Clone)]
pub struct HeapProfilerService {}
#[derive(Debug, Clone)]
pub struct HeatmapService {}
#[derive(Debug, Clone)]
pub struct HeightmapImporterService {}
#[derive(Debug, Clone)]
pub struct HiddenSurfaceRemovalAsset {
    pub HsrData: BinaryString,
    pub HsrMeshIdData: BinaryString,
}
#[derive(Debug, Clone)]
pub struct Highlight {
    pub Adornee: Ref,
    pub DepthMode: HighlightDepthMode,
    pub Enabled: bool,
    pub FillColor: Color3,
    pub FillTransparency: f32,
    pub LineThickness: i32,
    pub OutlineColor: Color3,
    pub OutlineTransparency: f32,
    pub ReservedId: ReservedHighlightId,
}
#[derive(Debug, Clone)]
pub struct HingeConstraint {
    pub ActuatorType: ActuatorType,
    pub AngularResponsiveness: f32,
    pub AngularSpeed: f32,
    pub AngularVelocity: f32,
    pub CurrentAngle: f32,
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
pub struct Hint {}
#[derive(Debug, Clone)]
pub struct Hole {}
#[derive(Debug, Clone)]
pub struct Hopper {}
#[derive(Debug, Clone)]
pub struct HopperBin {
    pub Active: bool,
    pub BinType: BinType,
    pub Command: String,
    pub TextureName: String,
}
#[derive(Debug, Clone)]
pub struct HttpRbxApiService {}
#[derive(Debug, Clone)]
pub struct HttpRequest {}
#[derive(Debug, Clone)]
pub struct HttpService {
    pub HttpEnabled: bool,
}
#[derive(Debug, Clone)]
pub struct Humanoid {
    pub AutoJumpEnabled: bool,
    pub AutoRotate: bool,
    pub AutomaticScalingEnabled: bool,
    pub BreakJointsOnDeath: bool,
    pub CameraMaxDistance: f32,
    pub CameraMinDistance: f32,
    pub CameraMode: CameraMode,
    pub CameraOffset: Vector3,
    pub CollisionType: HumanoidCollisionType,
    pub DisplayDistanceType: HumanoidDisplayDistanceType,
    pub DisplayName: String,
    pub EvaluateStateMachine: bool,
    pub FloorMaterial: Material,
    pub Health: f32,
    pub HealthDisplayDistance: f32,
    pub HealthDisplayType: HumanoidHealthDisplayType,
    pub HealthXml: f32,
    pub HipHeight: f32,
    pub InternalBodyScale: Vector3,
    pub InternalDisplayName: String,
    pub InternalHeadScale: f32,
    pub InternalOriginalHipHeight: f32,
    pub Jump: bool,
    pub JumpHeight: f32,
    pub JumpPower: f32,
    pub JumpReplicate: bool,
    pub LeftLeg: Ref,
    pub MaxHealth: f32,
    pub MaxHealth: f32,
    pub MaxSlopeAngle: f32,
    pub MoveDirection: Vector3,
    pub MoveDirectionInternal: Vector3,
    pub NameDisplayDistance: f32,
    pub NameOcclusion: NameOcclusion,
    pub NetworkHumanoidState: HumanoidStateType,
    pub PlatformStand: bool,
    pub RequiresNeck: bool,
    pub RigType: HumanoidRigType,
    pub RightLeg: Ref,
    pub RootPart: Ref,
    pub SeatPart: Ref,
    pub Sit: bool,
    pub Strafe: bool,
    pub TargetPoint: Vector3,
    pub Torso: Ref,
    pub UseJumpPower: bool,
    pub WalkAngleError: f32,
    pub WalkDirection: Vector3,
    pub WalkSpeed: f32,
    pub WalkToPart: Ref,
    pub WalkToPoint: Vector3,
}
#[derive(Debug, Clone)]
pub struct HumanoidController {}
#[derive(Debug, Clone)]
pub struct HumanoidDescription {
    pub AccessoryBlob: String,
    pub BackAccessory: String,
    pub BodyTypeScale: f32,
    pub ClimbAnimation: i64,
    pub DepthScale: f32,
    pub EmotesDataInternal: String,
    pub EquippedEmotesDataInternal: String,
    pub Face: i64,
    pub FaceAccessory: String,
    pub FallAnimation: i64,
    pub FrontAccessory: String,
    pub GraphicTShirt: i64,
    pub HairAccessory: String,
    pub HatAccessory: String,
    pub Head: i64,
    pub HeadColor: Color3,
    pub HeadScale: f32,
    pub HeightScale: f32,
    pub IdleAnimation: i64,
    pub JumpAnimation: i64,
    pub LeftArm: i64,
    pub LeftArmColor: Color3,
    pub LeftLeg: i64,
    pub LeftLegColor: Color3,
    pub MoodAnimation: i64,
    pub NeckAccessory: String,
    pub NumberEmotesLoaded: i32,
    pub Pants: i64,
    pub ProportionScale: f32,
    pub ResetIncludesBodyParts: bool,
    pub RightArm: i64,
    pub RightArmColor: Color3,
    pub RightLeg: i64,
    pub RightLegColor: Color3,
    pub RunAnimation: i64,
    pub Shirt: i64,
    pub ShouldersAccessory: String,
    pub SwimAnimation: i64,
    pub Torso: i64,
    pub TorsoColor: Color3,
    pub WaistAccessory: String,
    pub WalkAnimation: i64,
    pub WidthScale: f32,
}
#[derive(Debug, Clone)]
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
    pub Type: IKControlType,
    pub Weight: f32,
}
#[derive(Debug, Clone)]
pub struct ILegacyStudioBridge {}
#[derive(Debug, Clone)]
pub struct IXPService {}
#[derive(Debug, Clone)]
pub struct ImageButton {
    pub ContentImageSize: Vector2,
    pub HoverImage: ContentId,
    pub HoverImageContent: Content,
    pub Image: ContentId,
    pub ImageColor3: Color3,
    pub ImageContent: Content,
    pub ImageRectOffset: Vector2,
    pub ImageRectSize: Vector2,
    pub ImageTransparency: f32,
    pub IsLoaded: bool,
    pub PressedImage: ContentId,
    pub PressedImageContent: Content,
    pub ResampleMode: ResamplerMode,
    pub ScaleType: ScaleType,
    pub SliceCenter: Rect,
    pub SliceScale: f32,
    pub TileSize: UDim2,
}
#[derive(Debug, Clone)]
pub struct ImageHandleAdornment {
    pub Image: ContentId,
    pub Size: Vector2,
}
#[derive(Debug, Clone)]
pub struct ImageLabel {
    pub ContentImageSize: Vector2,
    pub Image: ContentId,
    pub ImageColor3: Color3,
    pub ImageContent: Content,
    pub ImageRectOffset: Vector2,
    pub ImageRectSize: Vector2,
    pub ImageTransparency: f32,
    pub IsLoaded: bool,
    pub ResampleMode: ResamplerMode,
    pub ScaleType: ScaleType,
    pub SliceCenter: Rect,
    pub SliceScale: f32,
    pub TileSize: UDim2,
}
#[derive(Debug, Clone)]
pub struct ImportSession {}
#[derive(Debug, Clone)]
pub struct IncrementalPatchBuilder {
    pub AddPathsToBundle: bool,
    pub BuildDebouncePeriod: f64,
    pub HighCompression: bool,
    pub SerializePatch: bool,
    pub ZstdCompression: bool,
}
#[derive(Debug, Clone)]
pub struct InputAction {
    pub Enabled: bool,
    pub Type: InputActionType,
}
#[derive(Debug, Clone)]
pub struct InputBinding {
    pub Down: KeyCode,
    pub KeyCode: KeyCode,
    pub Left: KeyCode,
    pub PressedThreshold: f32,
    pub ReleasedThreshold: f32,
    pub Right: KeyCode,
    pub Scale: f32,
    pub UiButton: Ref,
    pub Up: KeyCode,
    pub Vector2Scale: Vector2,
}
#[derive(Debug, Clone)]
pub struct InputContext {
    pub Enabled: bool,
    pub Priority: i32,
    pub Sink: bool,
}
#[derive(Debug, Clone)]
pub struct InputObject {
    pub Delta: Vector3,
    pub KeyCode: KeyCode,
    pub Position: Vector3,
    pub UserInputState: UserInputState,
    pub UserInputType: UserInputType,
}
#[derive(Debug, Clone)]
pub struct InsertService {
    pub AllowClientInsertModels: bool,
    pub AllowInsertFreeModels: bool,
}
#[derive(Debug, Clone)]
pub struct Instance {
    pub Archivable: bool,
    pub Archivable: bool,
    pub Attributes: Attributes,
    pub AttributesReplicate: String,
    pub AttributesSerialize: BinaryString,
    pub Capabilities: SecurityCapabilities,
    pub DataCost: i32,
    pub DefinesCapabilities: bool,
    pub HistoryId: UniqueId,
    pub Name: String,
    pub NumExpectedDirectChildren: i32,
    pub Parent: Ref,
    pub PropertyStatusStudio: PropertyStatus,
    pub RobloxLocked: bool,
    pub Sandboxed: bool,
    pub SourceAssetId: i64,
    pub Tags: Tags,
    pub UniqueId: UniqueId,
}
#[derive(Debug, Clone)]
pub struct InstanceAdornment {
    pub Adornee: Ref,
}
#[derive(Debug, Clone)]
pub struct IntConstrainedValue {
    pub ConstrainedValue: i64,
    pub MaxValue: i64,
    pub MinValue: i64,
    pub Value: i64,
    pub Value: i64,
}
#[derive(Debug, Clone)]
pub struct IntValue {
    pub Value: i64,
}
#[derive(Debug, Clone)]
pub struct InternalSyncItem {
    pub AutoSync: bool,
    pub Enabled: bool,
    pub Path: String,
    pub Target: Ref,
}
#[derive(Debug, Clone)]
pub struct InternalSyncService {}
#[derive(Debug, Clone)]
pub struct IntersectOperation {}
#[derive(Debug, Clone)]
pub struct InventoryPages {}
#[derive(Debug, Clone)]
pub struct JointImportData {}
#[derive(Debug, Clone)]
pub struct JointInstance {
    pub Active: bool,
    pub C0: CFrame,
    pub C1: CFrame,
    pub Enabled: bool,
    pub Part0: Ref,
    pub Part1: Ref,
    pub Part1: Ref,
}
#[derive(Debug, Clone)]
pub struct JointsService {}
#[derive(Debug, Clone)]
pub struct KeyboardService {}
#[derive(Debug, Clone)]
pub struct Keyframe {
    pub Time: f32,
}
#[derive(Debug, Clone)]
pub struct KeyframeMarker {
    pub Value: String,
}
#[derive(Debug, Clone)]
pub struct KeyframeSequence {
    pub AuthoredHipHeight: f32,
}
#[derive(Debug, Clone)]
pub struct KeyframeSequenceProvider {}
#[derive(Debug, Clone)]
pub struct LSPFileSyncService {}
#[derive(Debug, Clone)]
pub struct LanguageService {}
#[derive(Debug, Clone)]
pub struct LayerCollector {
    pub Enabled: bool,
    pub ResetOnSpawn: bool,
    pub ZIndexBehavior: ZIndexBehavior,
}
#[derive(Debug, Clone)]
pub struct LegacyStudioBridge {}
#[derive(Debug, Clone)]
pub struct Light {
    pub Brightness: f32,
    pub Color: Color3,
    pub Enabled: bool,
    pub Shadows: bool,
}
#[derive(Debug, Clone)]
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
    pub LightingStyle: LightingStyle,
    pub OutdoorAmbient: Color3,
    pub Outlines: bool,
    pub PrioritizeLightingQuality: bool,
    pub ShadowColor: Color3,
    pub ShadowSoftness: f32,
    pub Technology: Technology,
    pub TimeOfDay: String,
}
#[derive(Debug, Clone)]
pub struct LineForce {
    pub ApplyAtCenterOfMass: bool,
    pub InverseSquareLaw: bool,
    pub Magnitude: f32,
    pub MaxForce: f32,
    pub ReactionForceEnabled: bool,
}
#[derive(Debug, Clone)]
pub struct LineHandleAdornment {
    pub Length: f32,
    pub Thickness: f32,
}
#[derive(Debug, Clone)]
pub struct LinearVelocity {
    pub ForceLimitMode: ForceLimitMode,
    pub ForceLimitsEnabled: bool,
    pub LineDirection: Vector3,
    pub LineVelocity: f32,
    pub MaxAxesForce: Vector3,
    pub MaxForce: f32,
    pub MaxPlanarAxesForce: Vector2,
    pub PlaneVelocity: Vector2,
    pub PrimaryTangentAxis: Vector3,
    pub RelativeTo: ActuatorRelativeTo,
    pub SecondaryTangentAxis: Vector3,
    pub VectorVelocity: Vector3,
    pub VelocityConstraintMode: VelocityConstraintMode,
}
#[derive(Debug, Clone)]
pub struct LinkingService {}
#[derive(Debug, Clone)]
pub struct LiveScriptingService {
    pub ServerLiveEditingMode: ServerLiveEditingMode,
}
#[derive(Debug, Clone)]
pub struct LiveSyncService {
    pub HasSyncedInstances: bool,
}
#[derive(Debug, Clone)]
pub struct LocalDebuggerConnection {}
#[derive(Debug, Clone)]
pub struct LocalScript {}
#[derive(Debug, Clone)]
pub struct LocalStorageService {}
#[derive(Debug, Clone)]
pub struct LocalizationService {
    pub ForcePlayModeGameLocaleId: String,
    pub ForcePlayModeRobloxLocaleId: String,
    pub GameSourceLanguageId: String,
    pub IsTextScraperRunning: bool,
    pub LocaleManifest: String,
    pub RobloxForcePlayModeGameLocaleId: String,
    pub RobloxForcePlayModeRobloxLocaleId: String,
    pub RobloxLocaleId: String,
    pub ShouldUseCloudTable: bool,
    pub SystemLocaleId: String,
}
#[derive(Debug, Clone)]
pub struct LocalizationTable {
    pub Contents: String,
    pub DevelopmentLanguage: String,
    pub IsExemptFromUgcAnalytics: bool,
    pub Root: Ref,
    pub SourceLocaleId: String,
}
#[derive(Debug, Clone)]
pub struct LodDataEntity {
    pub EntityData: SharedString,
    pub EntityLodEnabled: bool,
    pub EntityPosition: CFrame,
    pub EntityScale: Vector3,
    pub EntitySource: Ref,
}
#[derive(Debug, Clone)]
pub struct LodDataService {}
#[derive(Debug, Clone)]
pub struct LogReporterService {}
#[derive(Debug, Clone)]
pub struct LogService {}
#[derive(Debug, Clone)]
pub struct LoginService {}
#[derive(Debug, Clone)]
pub struct LuaSettings {}
#[derive(Debug, Clone)]
pub struct LuaSourceContainer {
    pub CachedRemoteSource: String,
    pub CachedRemoteSourceLoadState: i32,
    pub HasAssociatedDrafts: bool,
    pub IsDifferentFromFileSystem: bool,
    pub IsPlayerScript: bool,
    pub SandboxedSource: String,
    pub ScriptGuid: String,
}
#[derive(Debug, Clone)]
pub struct LuaWebService {}
#[derive(Debug, Clone)]
pub struct LuauScriptAnalyzerService {}
#[derive(Debug, Clone)]
pub struct MLModelDeliveryService {}
#[derive(Debug, Clone)]
pub struct ManualGlue {}
#[derive(Debug, Clone)]
pub struct ManualSurfaceJointInstance {}
#[derive(Debug, Clone)]
pub struct ManualWeld {}
#[derive(Debug, Clone)]
pub struct MarkerCurve {
    pub Length: i32,
    pub ValuesAndTimes: BinaryString,
}
#[derive(Debug, Clone)]
pub struct MarketplaceService {}
#[derive(Debug, Clone)]
pub struct MatchmakingService {}
#[derive(Debug, Clone)]
pub struct MaterialGenerationService {}
#[derive(Debug, Clone)]
pub struct MaterialGenerationSession {}
#[derive(Debug, Clone)]
pub struct MaterialImportData {
    pub DiffuseFilePath: String,
    pub IsPbr: bool,
    pub MetalnessFilePath: String,
    pub NormalFilePath: String,
    pub RoughnessFilePath: String,
}
#[derive(Debug, Clone)]
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
    pub Use2022Materials: bool,
    pub Use2022MaterialsXml: bool,
    pub WoodName: String,
    pub WoodPlanksName: String,
}
#[derive(Debug, Clone)]
pub struct MaterialVariant {
    pub AvgMetalness: i32,
    pub AvgRoughness: i32,
    pub BaseMaterial: Material,
    pub ColorMap: ContentId,
    pub CustomPhysicalProperties: PhysicalProperties,
    pub MaterialPattern: MaterialPattern,
    pub MetalnessMap: ContentId,
    pub NormalMap: ContentId,
    pub RoughnessMap: ContentId,
    pub StudsPerTile: f32,
    pub TexturePack: ContentId,
}
#[derive(Debug, Clone)]
pub struct MemStorageConnection {}
#[derive(Debug, Clone)]
pub struct MemStorageService {}
#[derive(Debug, Clone)]
pub struct MemoryStoreHashMap {}
#[derive(Debug, Clone)]
pub struct MemoryStoreHashMapPages {}
#[derive(Debug, Clone)]
pub struct MemoryStoreQueue {}
#[derive(Debug, Clone)]
pub struct MemoryStoreService {}
#[derive(Debug, Clone)]
pub struct MemoryStoreSortedMap {}
#[derive(Debug, Clone)]
pub struct MeshContentProvider {}
#[derive(Debug, Clone)]
pub struct MeshImportData {
    pub Anchored: bool,
    pub CageManifold: bool,
    pub CageMeshIntersectedPreview: bool,
    pub CageMeshNotIntersected: bool,
    pub CageNoOverlappingVertices: bool,
    pub CageNonManifoldPreview: bool,
    pub CageOverlappingVerticesPreview: bool,
    pub CageUvMatched: bool,
    pub CageUvMisMatchedPreview: bool,
    pub Dimensions: Vector3,
    pub DoubleSided: bool,
    pub IgnoreVertexColors: bool,
    pub IrrelevantCageModifiedPreview: bool,
    pub MeshHoleDetectedPreview: bool,
    pub MeshNoHoleDetected: bool,
    pub NoIrrelevantCageModified: bool,
    pub NoOuterCageFarExtendedFromMesh: bool,
    pub OuterCageFarExtendedFromMeshPreview: bool,
    pub PolygonCount: f32,
    pub UseImportedPivot: bool,
}
#[derive(Debug, Clone)]
pub struct MeshPart {
    pub AlternateMeshHash: i64,
    pub DoubleSided: bool,
    pub EditableMeshString: SharedString,
    pub HasJointOffset: bool,
    pub HasSkinnedMesh: bool,
    pub InitialSize: Vector3,
    pub JointOffset: Vector3,
    pub MeshContent: Content,
    pub MeshId: ContentId,
    pub MeshId: ContentId,
    pub PhysicsData: BinaryString,
    pub RenderFidelity: RenderFidelity,
    pub RenderFidelityReplicate: RenderFidelity,
    pub TextureContent: Content,
    pub TextureId: ContentId,
    pub VertexCount: i32,
}
#[derive(Debug, Clone)]
pub struct Message {
    pub Text: String,
}
#[derive(Debug, Clone)]
pub struct MessageBusConnection {}
#[derive(Debug, Clone)]
pub struct MessageBusService {}
#[derive(Debug, Clone)]
pub struct MessagingService {}
#[derive(Debug, Clone)]
pub struct MetaBreakpoint {
    pub Condition: String,
    pub ContinueExecution: bool,
    pub Enabled: bool,
    pub Id: i32,
    pub IsLogpoint: bool,
    pub Line: i32,
    pub LogMessage: String,
    pub RemoveOnHit: bool,
    pub Script: String,
    pub Valid: bool,
}
#[derive(Debug, Clone)]
pub struct MetaBreakpointContext {
    pub ContextDataInternal: String,
}
#[derive(Debug, Clone)]
pub struct MetaBreakpointManager {}
#[derive(Debug, Clone)]
pub struct Model {
    pub LevelOfDetail: ModelLevelOfDetail,
    pub LodEntity: Ref,
    pub ModelMeshCFrame: CFrame,
    pub ModelMeshData: SharedString,
    pub ModelMeshSize: Vector3,
    pub ModelStreamingMode: ModelStreamingMode,
    pub NeedsPivotMigration: bool,
    pub PrimaryPart: Ref,
    pub Scale: f32,
    pub ScaleFactor: f32,
    pub WorldPivot: CFrame,
    pub WorldPivotData: Option<CFrame>,
}
#[derive(Debug, Clone)]
pub struct ModuleScript {
    pub Confidential: bool,
    pub LinkedSource: ContentId,
    pub Source: String,
}
#[derive(Debug, Clone)]
pub struct Motor {
    pub CurrentAngle: f32,
    pub DesiredAngle: f32,
    pub MaxVelocity: f32,
    pub ReplicateCurrentAngle: f32,
}
#[derive(Debug, Clone)]
pub struct Motor6D {
    pub ChildName: String,
    pub ParentName: String,
    pub ReplicateCurrentAngle6D: Vector3,
    pub ReplicateCurrentOffset6D: Vector3,
    pub Transform: CFrame,
}
#[derive(Debug, Clone)]
pub struct MotorFeature {}
#[derive(Debug, Clone)]
pub struct Mouse {
    pub Hit: CFrame,
    pub Hit: CFrame,
    pub Icon: ContentId,
    pub Origin: CFrame,
    pub Target: Ref,
    pub Target: Ref,
    pub TargetFilter: Ref,
    pub TargetSurface: NormalId,
    pub UnitRay: Ray,
    pub ViewSizeX: i32,
    pub ViewSizeY: i32,
    pub X: i32,
    pub Y: i32,
}
#[derive(Debug, Clone)]
pub struct MouseService {}
#[derive(Debug, Clone)]
pub struct MultipleDocumentInterfaceInstance {
    pub FocusedDataModelSession: Ref,
}
#[derive(Debug, Clone)]
pub struct NegateOperation {}
#[derive(Debug, Clone)]
pub struct NetworkClient {}
#[derive(Debug, Clone)]
pub struct NetworkMarker {}
#[derive(Debug, Clone)]
pub struct NetworkPeer {}
#[derive(Debug, Clone)]
pub struct NetworkReplicator {}
#[derive(Debug, Clone)]
pub struct NetworkServer {}
#[derive(Debug, Clone)]
pub struct NetworkSettings {
    pub EmulatedTotalMemoryInMb: i32,
    pub FreeMemoryMBytes: f32,
    pub HttpProxyEnabled: bool,
    pub HttpProxyUrl: String,
    pub IncomingReplicationLag: f64,
    pub OpenCertManagerDialog: i32,
    pub PrintJoinSizeBreakdown: bool,
    pub PrintPhysicsErrors: bool,
    pub PrintStreamInstanceQuota: bool,
    pub RandomizeJoinInstanceOrder: bool,
    pub RenderStreamedRegions: bool,
    pub ShowActiveAnimationAsset: bool,
}
#[derive(Debug, Clone)]
pub struct NoCollisionConstraint {
    pub Enabled: bool,
    pub Part0: Ref,
    pub Part1: Ref,
}
#[derive(Debug, Clone)]
pub struct Noise {
    pub NoiseType: NoiseType,
    pub Seed: i32,
}
#[derive(Debug, Clone)]
pub struct NonReplicatedCSGDictionaryService {}
#[derive(Debug, Clone)]
pub struct NotificationService {
    pub IsConnected: bool,
    pub IsLuaChatEnabled: bool,
    pub IsLuaGameDetailsEnabled: bool,
    pub SelectedTheme: String,
}
#[derive(Debug, Clone)]
pub struct NumberPose {
    pub Value: f64,
}
#[derive(Debug, Clone)]
pub struct NumberValue {
    pub Value: f64,
}
#[derive(Debug, Clone)]
pub struct Object {
    pub ClassName: String,
    pub ClassName: String,
}
#[derive(Debug, Clone)]
pub struct ObjectValue {
    pub Value: Ref,
}
#[derive(Debug, Clone)]
pub struct OmniRecommendationsService {}
#[derive(Debug, Clone)]
pub struct OpenCloudApiV1 {}
#[derive(Debug, Clone)]
pub struct OpenCloudService {}
#[derive(Debug, Clone)]
pub struct OperationGraph {}
#[derive(Debug, Clone)]
pub struct OrderedDataStore {}
#[derive(Debug, Clone)]
pub struct OutfitPages {}
#[derive(Debug, Clone)]
pub struct PVAdornment {
    pub Adornee: Ref,
}
#[derive(Debug, Clone)]
pub struct PVInstance {
    pub Origin: CFrame,
    pub PivotOffset: CFrame,
}
#[derive(Debug, Clone)]
pub struct PackageLink {
    pub AutoUpdate: bool,
    pub CanAutoUpdate: bool,
    pub Creator: String,
    pub DefaultName: String,
    pub HasNewVersion: bool,
    pub ModifiedState: i32,
    pub PackageAssetName: String,
    pub PackageGuid: i64,
    pub PackageId: ContentId,
    pub PackageIdSerialize: ContentId,
    pub PermissionLevel: PackagePermission,
    pub SerializedDefaultAttributes: BinaryString,
    pub Status: String,
    pub VersionIdSerialize: i64,
    pub VersionNumber: i64,
}
#[derive(Debug, Clone)]
pub struct PackageService {}
#[derive(Debug, Clone)]
pub struct PackageUIService {}
#[derive(Debug, Clone)]
pub struct Pages {
    pub IsFinished: bool,
}
#[derive(Debug, Clone)]
pub struct Pants {
    pub PantsTemplate: ContentId,
}
#[derive(Debug, Clone)]
pub struct ParabolaAdornment {
    pub A: f32,
    pub B: f32,
    pub C: f32,
    pub Range: f32,
    pub Thickness: f32,
}
#[derive(Debug, Clone)]
pub struct Part {
    pub Shap: PartType,
    pub Shape: PartType,
    pub Shape: PartType,
}
#[derive(Debug, Clone)]
pub struct PartAdornment {
    pub Adornee: Ref,
}
#[derive(Debug, Clone)]
pub struct PartOperation {
    pub AssetId: ContentId,
    pub ChildData: BinaryString,
    pub ChildData2: SharedString,
    pub ComponentIndex: i32,
    pub CsgMesh2KernelMap: SharedString,
    pub FormFactor: FormFactor,
    pub InitialSize: Vector3,
    pub ManifoldMesh: SharedString,
    pub MeshData: BinaryString,
    pub MeshData2: SharedString,
    pub PhysicsData: BinaryString,
    pub RenderFidelity: RenderFidelity,
    pub SerializedCsgTree: SharedString,
    pub SerializedOperationGraph: SharedString,
    pub SmoothingAngle: f32,
    pub TriangleCount: i32,
    pub UsePartColor: bool,
}
#[derive(Debug, Clone)]
pub struct PartOperationAsset {
    pub ChildData: BinaryString,
    pub MeshData: BinaryString,
}
#[derive(Debug, Clone)]
pub struct ParticleEmitter {
    pub Acceleration: Vector3,
    pub Brightness: f32,
    pub Color: ColorSequence,
    pub Drag: f32,
    pub EmissionDirection: NormalId,
    pub Enabled: bool,
    pub FlipbookFramerate: NumberRange,
    pub FlipbookIncompatible: String,
    pub FlipbookLayout: ParticleFlipbookLayout,
    pub FlipbookMode: ParticleFlipbookMode,
    pub FlipbookStartRandom: bool,
    pub Lifetime: NumberRange,
    pub LightEmission: f32,
    pub LightInfluence: f32,
    pub LocalTransparencyModifier: f32,
    pub LockedToPart: bool,
    pub Orientation: ParticleOrientation,
    pub Rate: f32,
    pub RotSpeed: NumberRange,
    pub Rotation: NumberRange,
    pub Shape: ParticleEmitterShape,
    pub ShapeInOut: ParticleEmitterShapeInOut,
    pub ShapePartial: f32,
    pub ShapeStyle: ParticleEmitterShapeStyle,
    pub Size: NumberSequence,
    pub Speed: NumberRange,
    pub SpreadAngle: Vector2,
    pub Squash: NumberSequence,
    pub Texture: ContentId,
    pub TimeScale: f32,
    pub Transparency: NumberSequence,
    pub VelocityInheritance: f32,
    pub VelocitySpread: f32,
    pub WindAffectsDrag: bool,
    pub ZOffset: f32,
}
#[derive(Debug, Clone)]
pub struct PatchBundlerFileWatch {}
#[derive(Debug, Clone)]
pub struct PatchMapping {
    pub FlattenTree: bool,
    pub PatchId: String,
    pub TargetPath: String,
}
#[derive(Debug, Clone)]
pub struct Path {
    pub Status: PathStatus,
}
#[derive(Debug, Clone)]
pub struct Path2D {
    pub Closed: bool,
    pub Color3: Color3,
    pub PropertiesSerialize: BinaryString,
    pub SelectedControlPoint: i32,
    pub Thickness: f32,
    pub Transparency: f32,
    pub Visible: bool,
    pub ZIndex: i32,
}
#[derive(Debug, Clone)]
pub struct PathfindingLink {
    pub Attachment0: Ref,
    pub Attachment1: Ref,
    pub IsBidirectional: bool,
    pub Label: String,
}
#[derive(Debug, Clone)]
pub struct PathfindingModifier {
    pub Label: String,
    pub PassThrough: bool,
}
#[derive(Debug, Clone)]
pub struct PathfindingService {
    pub EmptyCutoff: f32,
}
#[derive(Debug, Clone)]
pub struct PausedState {
    pub AllThreadsPaused: bool,
    pub Reason: DebuggerPauseReason,
    pub ThreadId: i32,
}
#[derive(Debug, Clone)]
pub struct PausedStateBreakpoint {
    pub Breakpoint: Ref,
}
#[derive(Debug, Clone)]
pub struct PausedStateException {
    pub ExceptionText: String,
}
#[derive(Debug, Clone)]
pub struct PerformanceControlService {}
#[derive(Debug, Clone)]
pub struct PermissionsService {}
#[derive(Debug, Clone)]
pub struct PhysicsService {}
#[derive(Debug, Clone)]
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
    pub PhysicsEnvironmentalThrottle: EnviromentalPhysicsThrottle,
    pub ShowDecompositionGeometry: bool,
    pub ShowFluidForcesForSelectedOrHoveredMechanisms: bool,
    pub ShowInstanceNamesForDrawnForcesAndTorques: bool,
    pub SolverConvergenceMetricType: SolverConvergenceMetricType,
    pub SolverConvergenceVisualizationMode: SolverConvergenceVisualizationMode,
    pub ThrottleAdjustTime: f64,
    pub TorqueDrawScale: f32,
    pub UseCsGv2: bool,
}
#[derive(Debug, Clone)]
pub struct PitchShiftSoundEffect {
    pub Octave: f32,
}
#[derive(Debug, Clone)]
pub struct PlaceAssetIdsService {}
#[derive(Debug, Clone)]
pub struct PlaceStatsService {}
#[derive(Debug, Clone)]
pub struct PlacesService {}
#[derive(Debug, Clone)]
pub struct Plane {}
#[derive(Debug, Clone)]
pub struct PlaneConstraint {}
#[derive(Debug, Clone)]
pub struct Platform {}
#[derive(Debug, Clone)]
pub struct PlatformCloudStorageService {}
#[derive(Debug, Clone)]
pub struct PlatformFriendsService {}
#[derive(Debug, Clone)]
pub struct Player {
    pub AccountAge: i32,
    pub AccountAgeReplicate: i32,
    pub AppearanceDidLoad: bool,
    pub AutoJumpEnabled: bool,
    pub CameraMaxZoomDistance: f32,
    pub CameraMinZoomDistance: f32,
    pub CameraMode: CameraMode,
    pub CanLoadCharacterAppearance: bool,
    pub Character: Ref,
    pub CharacterAppearance: String,
    pub CharacterAppearanceId: i64,
    pub ChararacterRegionId: Vector3,
    pub ChatMode: ChatMode,
    pub ChatPrivacyMode: ChatPrivacyMode,
    pub CloudEditCameraCoordinateFrame: CFrame,
    pub CloudEditPlayerActive: bool,
    pub CountryRegionCodeReplicate: String,
    pub DataComplexity: i32,
    pub DataComplexityLimit: i32,
    pub DataReady: bool,
    pub DevCameraOcclusionMode: DevCameraOcclusionMode,
    pub DevComputerCameraMode: DevComputerCameraMovementMode,
    pub DevComputerMovementMode: DevComputerMovementMode,
    pub DevEnableMouseLock: bool,
    pub DevTouchCameraMode: DevTouchCameraMovementMode,
    pub DevTouchMovementMode: DevTouchMovementMode,
    pub DisplayName: String,
    pub FollowUserId: i64,
    pub FollowUserIdReplicated: i64,
    pub GameplayPaused: bool,
    pub Guest: bool,
    pub HasVerifiedBadge: bool,
    pub HealthDisplayDistance: f32,
    pub InternalCharacterAppearanceLoaded: bool,
    pub LocaleId: String,
    pub MaxSimulationRadius: f32,
    pub MaximumSimulationRadius: f32,
    pub MembershipType: MembershipType,
    pub MembershipTypeReplicate: MembershipType,
    pub NameDisplayDistance: f32,
    pub Neutral: bool,
    pub OsPlatform: String,
    pub PartyId: String,
    pub PlatformName: String,
    pub RawJoinData: BinaryString,
    pub ReplicationFocus: Ref,
    pub RespawnLocation: Ref,
    pub SimulationRadius: f32,
    pub StepIdOffset: i32,
    pub SuperSafeChatReplicate: bool,
    pub Team: Ref,
    pub TeamColor: BrickColor,
    pub Teleported: bool,
    pub TeleportedIn: bool,
    pub ThirdPartyTextChatRestrictionStatus: ChatRestrictionStatus,
    pub UnfilteredChat: bool,
    pub UserId: i64,
    pub UserId: i64,
    pub VrDevice: String,
    pub VrEnabled: bool,
}
#[derive(Debug, Clone)]
pub struct PlayerData {}
#[derive(Debug, Clone)]
pub struct PlayerDataRecord {
    pub CreatedTime: i64,
    pub DefaultRecordName: bool,
    pub Dirty: bool,
    pub Error: PlayerDataErrorState,
    pub FlushedTime: i64,
    pub LoadedTime: i64,
    pub ModifiedTime: i64,
    pub NewRecord: bool,
    pub Readable: bool,
    pub RecordName: String,
    pub Writable: bool,
}
#[derive(Debug, Clone)]
pub struct PlayerDataRecordConfig {
    pub RecordName: String,
}
#[derive(Debug, Clone)]
pub struct PlayerDataService {
    pub LoadFailureBehavior: PlayerDataLoadFailureBehavior,
}
#[derive(Debug, Clone)]
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
pub struct PlayerGui {
    pub CurrentScreenOrientation: ScreenOrientation,
    pub ScreenOrientation: ScreenOrientation,
    pub SelectionImageObject: Ref,
}
#[derive(Debug, Clone)]
pub struct PlayerHydrationService {}
#[derive(Debug, Clone)]
pub struct PlayerMouse {}
#[derive(Debug, Clone)]
pub struct PlayerScripts {}
#[derive(Debug, Clone)]
pub struct PlayerViewService {}
#[derive(Debug, Clone)]
pub struct Players {
    pub BanningEnabled: bool,
    pub BubbleChat: bool,
    pub CharacterAutoLoads: bool,
    pub ClassicChat: bool,
    pub LocalPlayer: Ref,
    pub LocalPlayer: Ref,
    pub MaxPlayers: i32,
    pub MaxPlayersInternal: i32,
    pub NumPlayers: i32,
    pub NumPlayers: i32,
    pub PreferredPlayers: i32,
    pub PreferredPlayersInternal: i32,
    pub RespawnTime: f32,
    pub ServerGitHash: String,
    pub ServerLogPrefix: String,
    pub UseStrafingAnimations: bool,
}
#[derive(Debug, Clone)]
pub struct Plugin {
    pub CollisionEnabled: bool,
    pub DisableUiDragDetectorDrags: bool,
    pub GridSize: f32,
    pub HostDataModelType: StudioDataModelType,
    pub HostDataModelTypeIsCurrent: bool,
    pub IsDebuggable: bool,
    pub MultipleDocumentInterfaceInstance: Ref,
    pub UsesAssetInsertionDrag: bool,
}
#[derive(Debug, Clone)]
pub struct PluginAction {
    pub ActionId: String,
    pub AllowBinding: bool,
    pub Checked: bool,
    pub DefaultShortcut: String,
    pub Enabled: bool,
    pub StatusTip: String,
    pub Text: String,
}
#[derive(Debug, Clone)]
pub struct PluginCapabilities {
    pub Manifest: String,
}
#[derive(Debug, Clone)]
pub struct PluginDebugService {}
#[derive(Debug, Clone)]
pub struct PluginDragEvent {
    pub Data: String,
    pub MimeType: String,
    pub Position: Vector2,
    pub Sender: String,
}
#[derive(Debug, Clone)]
pub struct PluginGui {
    pub Title: String,
}
#[derive(Debug, Clone)]
pub struct PluginGuiService {}
#[derive(Debug, Clone)]
pub struct PluginManagementService {}
#[derive(Debug, Clone)]
pub struct PluginManager {}
#[derive(Debug, Clone)]
pub struct PluginManagerInterface {}
#[derive(Debug, Clone)]
pub struct PluginMenu {
    pub Icon: String,
    pub Title: String,
}
#[derive(Debug, Clone)]
pub struct PluginMouse {}
#[derive(Debug, Clone)]
pub struct PluginPolicyService {}
#[derive(Debug, Clone)]
pub struct PluginToolbar {}
#[derive(Debug, Clone)]
pub struct PluginToolbarButton {
    pub ClickableWhenViewportHidden: bool,
    pub Enabled: bool,
    pub Icon: ContentId,
}
#[derive(Debug, Clone)]
pub struct PointLight {
    pub Range: f32,
}
#[derive(Debug, Clone)]
pub struct PointsService {}
#[derive(Debug, Clone)]
pub struct PolicyService {
    pub IsLuobuServer: TriStateBoolean,
    pub LuobuWhitelisted: TriStateBoolean,
}
#[derive(Debug, Clone)]
pub struct Pose {
    pub CFrame: CFrame,
    pub MaskWeight: f32,
}
#[derive(Debug, Clone)]
pub struct PoseBase {
    pub EasingDirection: PoseEasingDirection,
    pub EasingStyle: PoseEasingStyle,
    pub Weight: f32,
}
#[derive(Debug, Clone)]
pub struct PostEffect {
    pub Enabled: bool,
}
#[derive(Debug, Clone)]
pub struct PrismaticConstraint {}
#[derive(Debug, Clone)]
pub struct ProcessInstancePhysicsService {}
#[derive(Debug, Clone)]
pub struct ProximityPrompt {
    pub ActionText: String,
    pub AutoLocalize: bool,
    pub ClickablePrompt: bool,
    pub Enabled: bool,
    pub Exclusivity: ProximityPromptExclusivity,
    pub GamepadKeyCode: KeyCode,
    pub HoldDuration: f32,
    pub KeyboardKeyCode: KeyCode,
    pub MaxActivationDistance: f32,
    pub ObjectText: String,
    pub RequiresLineOfSight: bool,
    pub RootLocalizationTable: Ref,
    pub Style: ProximityPromptStyle,
    pub UiOffset: Vector2,
}
#[derive(Debug, Clone)]
pub struct ProximityPromptService {
    pub Enabled: bool,
    pub MaxPromptsVisible: i32,
}
#[derive(Debug, Clone)]
pub struct PublishService {}
#[derive(Debug, Clone)]
pub struct QWidgetPluginGui {}
#[derive(Debug, Clone)]
pub struct RTAnimationTracker {
    pub Active: bool,
    pub EnableFallbackAudioInput: bool,
    pub SessionName: String,
    pub TrackerMode: TrackerMode,
    pub TrackerType: TrackerType,
}
#[derive(Debug, Clone)]
pub struct RayValue {
    pub Value: Ray,
}
#[derive(Debug, Clone)]
pub struct RbxAnalyticsService {}
#[derive(Debug, Clone)]
pub struct ReflectionMetadata {}
#[derive(Debug, Clone)]
pub struct ReflectionMetadataCallbacks {}
#[derive(Debug, Clone)]
pub struct ReflectionMetadataClass {
    pub ExplorerImageIndex: i32,
    pub ExplorerOrder: i32,
    pub Insertable: bool,
    pub PreferredParent: String,
    pub ServiceVisibility: ServiceVisibility,
}
#[derive(Debug, Clone)]
pub struct ReflectionMetadataClasses {}
#[derive(Debug, Clone)]
pub struct ReflectionMetadataEnum {}
#[derive(Debug, Clone)]
pub struct ReflectionMetadataEnumItem {}
#[derive(Debug, Clone)]
pub struct ReflectionMetadataEnums {}
#[derive(Debug, Clone)]
pub struct ReflectionMetadataEvents {}
#[derive(Debug, Clone)]
pub struct ReflectionMetadataFunctions {}
#[derive(Debug, Clone)]
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
pub struct ReflectionMetadataMember {}
#[derive(Debug, Clone)]
pub struct ReflectionMetadataProperties {}
#[derive(Debug, Clone)]
pub struct ReflectionMetadataYieldFunctions {}
#[derive(Debug, Clone)]
pub struct ReflectionService {}
#[derive(Debug, Clone)]
pub struct RelativeGui {}
#[derive(Debug, Clone)]
pub struct RemoteCursorService {}
#[derive(Debug, Clone)]
pub struct RemoteDebuggerServer {}
#[derive(Debug, Clone)]
pub struct RemoteEvent {}
#[derive(Debug, Clone)]
pub struct RemoteFunction {}
#[derive(Debug, Clone)]
pub struct RenderSettings {
    pub AutoFrmLevel: i32,
    pub EagerBulkExecution: bool,
    pub EditQualityLevel: QualityLevel,
    pub EnableFrm: bool,
    pub EnableVrMode: bool,
    pub ExportMergeByMaterial: bool,
    pub FrameRateManager: FramerateManagerMode,
    pub GraphicsMode: GraphicsMode,
    pub MeshCacheSize: i32,
    pub MeshPartDetailLevel: MeshPartDetailLevel,
    pub QualityLevel: QualityLevel,
    pub ReloadAssets: bool,
    pub RenderCsgTrianglesDebug: bool,
    pub ShowBoundingBoxes: bool,
    pub ViewMode: ViewMode,
}
#[derive(Debug, Clone)]
pub struct RenderingTest {
    pub CFrame: CFrame,
    pub ComparisonDiffThreshold: i32,
    pub ComparisonMethod: RenderingTestComparisonMethod,
    pub ComparisonPsnrThreshold: f32,
    pub Description: String,
    pub FieldOfView: f32,
    pub Orientation: Vector3,
    pub PerfTest: bool,
    pub Position: Vector3,
    pub QualityAuto: bool,
    pub QualityLevel: i32,
    pub RenderingTestFrameCount: i32,
    pub ShouldSkip: bool,
    pub Ticket: String,
    pub Timeout: i32,
}
#[derive(Debug, Clone)]
pub struct ReplicatedFirst {}
#[derive(Debug, Clone)]
pub struct ReplicatedStorage {}
#[derive(Debug, Clone)]
pub struct ReverbSoundEffect {
    pub DecayTime: f32,
    pub Density: f32,
    pub Diffusion: f32,
    pub DryLevel: f32,
    pub WetLevel: f32,
}
#[derive(Debug, Clone)]
pub struct RibbonNotificationService {}
#[derive(Debug, Clone)]
pub struct RigidConstraint {}
#[derive(Debug, Clone)]
pub struct RobloxPluginGuiService {}
#[derive(Debug, Clone)]
pub struct RobloxReplicatedStorage {}
#[derive(Debug, Clone)]
pub struct RobloxSerializableInstance {
    pub Data: BinaryString,
}
#[derive(Debug, Clone)]
pub struct RobloxServerStorage {}
#[derive(Debug, Clone)]
pub struct RocketPropulsion {
    pub Active: bool,
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
pub struct RodConstraint {
    pub CurrentDistance: f32,
    pub Length: f32,
    pub LimitAngle0: f32,
    pub LimitAngle1: f32,
    pub LimitsEnabled: bool,
    pub Thickness: f32,
}
#[derive(Debug, Clone)]
pub struct RomarkRbxAnalyticsService {}
#[derive(Debug, Clone)]
pub struct RomarkService {}
#[derive(Debug, Clone)]
pub struct RootImportData {
    pub AddModelToInventory: bool,
    pub Anchored: bool,
    pub AnimationIdForRestPose: f32,
    pub ExistingPackageId: String,
    pub FileDimensions: Vector3,
    pub ImportAsModelAsset: bool,
    pub ImportAsPackage: bool,
    pub InsertInWorkspace: bool,
    pub InsertWithScenePosition: bool,
    pub InvertNegativeFaces: bool,
    pub KeepZeroInfluenceBones: bool,
    pub MergeMeshes: bool,
    pub PolygonCount: f32,
    pub PreferredUploadId: i64,
    pub RestPose: RestPose,
    pub RigScale: RigScale,
    pub RigType: RigType,
    pub RigVisualization: bool,
    pub ScaleUnit: MeshScaleUnit,
    pub UseSceneOriginAsPivot: bool,
    pub UsesCages: bool,
    pub ValidateUgcBody: bool,
    pub WorldForward: NormalId,
    pub WorldUp: NormalId,
}
#[derive(Debug, Clone)]
pub struct RopeConstraint {
    pub CurrentDistance: f32,
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
pub struct Rotate {}
#[derive(Debug, Clone)]
pub struct RotateP {}
#[derive(Debug, Clone)]
pub struct RotateV {}
#[derive(Debug, Clone)]
pub struct RotationCurve {
    pub Length: i32,
    pub ValuesAndTimes: BinaryString,
}
#[derive(Debug, Clone)]
pub struct RtMessagingService {}
#[derive(Debug, Clone)]
pub struct RunService {
    pub ClientGitHash: String,
    pub RunState: RunState,
}
#[derive(Debug, Clone)]
pub struct RunningAverageItemDouble {}
#[derive(Debug, Clone)]
pub struct RunningAverageItemInt {}
#[derive(Debug, Clone)]
pub struct RunningAverageTimeIntervalItem {}
#[derive(Debug, Clone)]
pub struct RuntimeScriptService {}
#[derive(Debug, Clone)]
pub struct SafetyService {
    pub IsCaptureModeForReport: bool,
}
#[derive(Debug, Clone)]
pub struct ScreenGui {
    pub ClipToDeviceSafeArea: bool,
    pub DisplayOrder: i32,
    pub IgnoreGuiInset: bool,
    pub OnTopOfCoreBlur: bool,
    pub SafeAreaCompatibility: SafeAreaCompatibility,
    pub ScreenInsets: ScreenInsets,
}
#[derive(Debug, Clone)]
pub struct ScreenshotCapture {}
#[derive(Debug, Clone)]
pub struct ScreenshotHud {
    pub CameraButtonIcon: ContentId,
    pub CameraButtonPosition: UDim2,
    pub CloseButtonPosition: UDim2,
    pub CloseWhenScreenshotTaken: bool,
    pub ExperienceNameOverlayEnabled: bool,
    pub HideCoreGuiForCaptures: bool,
    pub HidePlayerGuiForCaptures: bool,
    pub OverlayFont: Font,
    pub UsernameOverlayEnabled: bool,
    pub Visible: bool,
}
#[derive(Debug, Clone)]
pub struct Script {
    pub Source: String,
}
#[derive(Debug, Clone)]
pub struct ScriptBuilder {}
#[derive(Debug, Clone)]
pub struct ScriptChangeService {}
#[derive(Debug, Clone)]
pub struct ScriptCloneWatcher {}
#[derive(Debug, Clone)]
pub struct ScriptCloneWatcherHelper {}
#[derive(Debug, Clone)]
pub struct ScriptCommitService {}
#[derive(Debug, Clone)]
pub struct ScriptContext {
    pub ScriptsDisabled: bool,
}
#[derive(Debug, Clone)]
pub struct ScriptDebugger {
    pub CoreScriptIdentifier: String,
    pub CurrentLine: i32,
    pub IsDebugging: bool,
    pub IsPaused: bool,
    pub Script: Ref,
    pub ScriptGuid: String,
}
#[derive(Debug, Clone)]
pub struct ScriptDocument {}
#[derive(Debug, Clone)]
pub struct ScriptEditorService {}
#[derive(Debug, Clone)]
pub struct ScriptProfilerService {}
#[derive(Debug, Clone)]
pub struct ScriptRegistrationService {}
#[derive(Debug, Clone)]
pub struct ScriptRuntime {}
#[derive(Debug, Clone)]
pub struct ScriptService {}
#[derive(Debug, Clone)]
pub struct ScrollingFrame {
    pub AbsoluteCanvasSize: Vector2,
    pub AbsoluteWindowSize: Vector2,
    pub AutomaticCanvasSize: AutomaticSize,
    pub BottomImage: ContentId,
    pub CanvasPosition: Vector2,
    pub CanvasSize: UDim2,
    pub DraggingScrollBar: DraggingScrollBar,
    pub ElasticBehavior: ElasticBehavior,
    pub HorizontalBarRect: Rect,
    pub HorizontalScrollBarInset: ScrollBarInset,
    pub MaxCanvasPosition: Vector2,
    pub MidImage: ContentId,
    pub ScrollBarImageColor3: Color3,
    pub ScrollBarImageTransparency: f32,
    pub ScrollBarThickness: i32,
    pub ScrollRate: f32,
    pub ScrollVelocity: Vector2,
    pub ScrollingDirection: ScrollingDirection,
    pub ScrollingEnabled: bool,
    pub SmoothScroll: bool,
    pub TopImage: ContentId,
    pub VerticalBarRect: Rect,
    pub VerticalScrollBarInset: ScrollBarInset,
    pub VerticalScrollBarPosition: VerticalScrollBarPosition,
}
#[derive(Debug, Clone)]
pub struct Seat {
    pub Disabled: bool,
    pub Occupant: Ref,
}
#[derive(Debug, Clone)]
pub struct Selection {
    pub ActiveInstance: Ref,
    pub RenderMode: SelectionRenderMode,
    pub SelectionBoxThickness: f32,
    pub SelectionLineThickness: i32,
    pub SelectionThickness: f32,
    pub ShowActiveInstanceHighlight: bool,
}
#[derive(Debug, Clone)]
pub struct SelectionBox {
    pub LineThickness: f32,
    pub StudioSelectionBox: bool,
    pub SurfaceColor: BrickColor,
    pub SurfaceColor3: Color3,
    pub SurfaceTransparency: f32,
}
#[derive(Debug, Clone)]
pub struct SelectionHighlightManager {}
#[derive(Debug, Clone)]
pub struct SelectionLasso {
    pub Humanoid: Ref,
}
#[derive(Debug, Clone)]
pub struct SelectionPartLasso {
    pub Part: Ref,
}
#[derive(Debug, Clone)]
pub struct SelectionPointLasso {
    pub Point: Vector3,
}
#[derive(Debug, Clone)]
pub struct SelectionSphere {
    pub SurfaceColor: BrickColor,
    pub SurfaceColor3: Color3,
    pub SurfaceTransparency: f32,
}
#[derive(Debug, Clone)]
pub struct SensorBase {
    pub UpdateType: SensorUpdateType,
}
#[derive(Debug, Clone)]
pub struct SerializationService {}
#[derive(Debug, Clone)]
pub struct ServerReplicator {}
#[derive(Debug, Clone)]
pub struct ServerScriptService {
    pub LoadStringEnabled: bool,
}
#[derive(Debug, Clone)]
pub struct ServerStorage {}
#[derive(Debug, Clone)]
pub struct ServiceProvider {}
#[derive(Debug, Clone)]
pub struct ServiceVisibilityService {
    pub HiddenServices: BinaryString,
    pub VisibleServices: BinaryString,
}
#[derive(Debug, Clone)]
pub struct SessionService {}
#[derive(Debug, Clone)]
pub struct SharedTableRegistry {}
#[derive(Debug, Clone)]
pub struct Shirt {
    pub ShirtTemplate: ContentId,
}
#[derive(Debug, Clone)]
pub struct ShirtGraphic {
    pub Color3: Color3,
    pub Graphic: ContentId,
}
#[derive(Debug, Clone)]
pub struct SkateboardController {
    pub Steer: f32,
    pub Throttle: f32,
}
#[derive(Debug, Clone)]
pub struct SkateboardPlatform {
    pub Controller: Ref,
    pub ControllingHumanoid: Ref,
    pub MoveState: MoveState,
    pub Steer: i32,
    pub StickyWheels: bool,
    pub Throttle: i32,
}
#[derive(Debug, Clone)]
pub struct Skin {
    pub SkinColor: BrickColor,
}
#[derive(Debug, Clone)]
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
pub struct SlidingBallConstraint {
    pub ActuatorType: ActuatorType,
    pub CurrentPosition: f32,
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
pub struct Smoke {
    pub Color: Color3,
    pub Enabled: bool,
    pub LocalTransparencyModifier: f32,
    pub Opacity: f32,
    pub OpacityXml: f32,
    pub RiseVelocity: f32,
    pub RiseVelocityXml: f32,
    pub Size: f32,
    pub SizeXml: f32,
    pub TimeScale: f32,
}
#[derive(Debug, Clone)]
pub struct SmoothVoxelsUpgraderService {}
#[derive(Debug, Clone)]
pub struct Snap {}
#[derive(Debug, Clone)]
pub struct SnippetService {}
#[derive(Debug, Clone)]
pub struct SocialService {}
#[derive(Debug, Clone)]
pub struct SolidModelContentProvider {}
#[derive(Debug, Clone)]
pub struct Sound {
    pub ChannelCount: i32,
    pub EmitterSize: f32,
    pub IsLoaded: bool,
    pub IsPaused: bool,
    pub IsPlaying: bool,
    pub IsPlaying: bool,
    pub IsSpatial: bool,
    pub LoopRegion: NumberRange,
    pub Looped: bool,
    pub MaxDistance: f32,
    pub MinDistance: f32,
    pub Pitch: f32,
    pub PlayOnRemove: bool,
    pub PlaybackLoudness: f64,
    pub PlaybackRegion: NumberRange,
    pub PlaybackRegionsEnabled: bool,
    pub PlaybackSpeed: f32,
    pub Playing: bool,
    pub PlayingReplicator: bool,
    pub RollOffGain: f32,
    pub RollOffMaxDistance: f32,
    pub RollOffMinDistance: f32,
    pub RollOffMode: RollOffMode,
    pub SoundGroup: Ref,
    pub SoundId: ContentId,
    pub TimeLength: f64,
    pub TimePosition: f64,
    pub TimePositionReplicator: f64,
    pub UsageContextPermission: UsageContext,
    pub Volume: f32,
    pub XmlReadMaxDistance3: f32,
    pub XmlReadMinDistance3: f32,
}
#[derive(Debug, Clone)]
pub struct SoundEffect {
    pub Enabled: bool,
    pub Priority: i32,
}
#[derive(Debug, Clone)]
pub struct SoundGroup {
    pub Volume: f32,
}
#[derive(Debug, Clone)]
pub struct SoundService {
    pub AmbientReverb: ReverbType,
    pub AudioApiByDefault: RolloutState,
    pub CharacterSoundsUseNewApi: RolloutState,
    pub DefaultListenerLocation: ListenerLocation,
    pub DistanceFactor: f32,
    pub DopplerScale: f32,
    pub IsNewExpForAudioApiByDefault: bool,
    pub RespectFilteringEnabled: bool,
    pub RolloffScale: f32,
    pub VolumetricAudio: VolumetricAudio,
}
#[derive(Debug, Clone)]
pub struct Sparkles {
    pub Color: Color3,
    pub Enabled: bool,
    pub LocalTransparencyModifier: f32,
    pub SparkleColor: Color3,
    pub TimeScale: f32,
}
#[derive(Debug, Clone)]
pub struct SpawnLocation {
    pub AllowTeamChangeOnTouch: bool,
    pub Duration: i32,
    pub Enabled: bool,
    pub Neutral: bool,
    pub TeamColor: BrickColor,
}
#[derive(Debug, Clone)]
pub struct SpawnerService {}
#[derive(Debug, Clone)]
pub struct SpecialMesh {
    pub MeshType: MeshType,
}
#[derive(Debug, Clone)]
pub struct SphereHandleAdornment {
    pub Radius: f32,
}
#[derive(Debug, Clone)]
pub struct SpotLight {
    pub Angle: f32,
    pub Face: NormalId,
    pub Range: f32,
}
#[derive(Debug, Clone)]
pub struct SpringConstraint {
    pub Coils: f32,
    pub CurrentLength: f32,
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
pub struct StackFrame {
    pub FrameId: i32,
    pub FrameName: String,
    pub FrameType: DebuggerFrameType,
    pub Globals: Ref,
    pub Line: i32,
    pub Locals: Ref,
    pub Populated: bool,
    pub Script: String,
    pub Upvalues: Ref,
}
#[derive(Debug, Clone)]
pub struct StandalonePluginScripts {}
#[derive(Debug, Clone)]
pub struct StandardPages {}
#[derive(Debug, Clone)]
pub struct StartPageService {}
#[derive(Debug, Clone)]
pub struct StarterCharacterScripts {}
#[derive(Debug, Clone)]
pub struct StarterGear {}
#[derive(Debug, Clone)]
pub struct StarterGui {
    pub ProcessUserInput: bool,
    pub ResetPlayerGuiOnSpawn: bool,
    pub RtlTextSupport: RtlTextSupport,
    pub ScreenOrientation: ScreenOrientation,
    pub ShowDevelopmentGui: bool,
    pub StudioDefaultStyleSheet: Ref,
    pub StudioInsertWidgetLayerCollectorAutoLinkStyleSheet: Ref,
    pub VirtualCursorMode: VirtualCursorMode,
}
#[derive(Debug, Clone)]
pub struct StarterPack {}
#[derive(Debug, Clone)]
pub struct StarterPlayer {
    pub AllowCustomAnimations: bool,
    pub AutoJumpEnabled: bool,
    pub AvatarJointUpgrade: RolloutState,
    pub AvatarJointUpgradeSerializedRollout: RolloutState,
    pub CameraMaxZoomDistance: f32,
    pub CameraMinZoomDistance: f32,
    pub CameraMode: CameraMode,
    pub CharacterJumpHeight: f32,
    pub CharacterJumpPower: f32,
    pub CharacterMaxSlopeAngle: f32,
    pub CharacterUseJumpPower: bool,
    pub CharacterWalkSpeed: f32,
    pub DevCameraOcclusionMode: DevCameraOcclusionMode,
    pub DevComputerCameraMovementMode: DevComputerCameraMovementMode,
    pub DevComputerMovementMode: DevComputerMovementMode,
    pub DevTouchCameraMovementMode: DevTouchCameraMovementMode,
    pub DevTouchMovementMode: DevTouchMovementMode,
    pub EnableDynamicHeads: LoadDynamicHeads,
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
    pub GameSettingsAvatar: GameAvatarType,
    pub GameSettingsR15Collision: R15CollisionType,
    pub GameSettingsScaleRangeBodyType: NumberRange,
    pub GameSettingsScaleRangeHead: NumberRange,
    pub GameSettingsScaleRangeHeight: NumberRange,
    pub GameSettingsScaleRangeProportion: NumberRange,
    pub GameSettingsScaleRangeWidth: NumberRange,
    pub HealthDisplayDistance: f32,
    pub LoadCharacterAppearance: bool,
    pub LoadCharacterLayeredClothing: LoadCharacterLayeredClothing,
    pub LoadCharacterLayeredClothing: LoadCharacterLayeredClothing,
    pub LuaCharacterController: CharacterControlMode,
    pub NameDisplayDistance: f32,
    pub RagdollDeath: bool,
    pub UserEmotesEnabled: bool,
}
#[derive(Debug, Clone)]
pub struct StarterPlayerScripts {}
#[derive(Debug, Clone)]
pub struct StartupMessageService {}
#[derive(Debug, Clone)]
pub struct Stats {
    pub ContactsCount: i32,
    pub DataReceiveKbps: f32,
    pub DataSendKbps: f32,
    pub FrameTime: f32,
    pub HeartbeatTime: f32,
    pub HeartbeatTimeMs: f32,
    pub InstanceCount: i32,
    pub MovingPrimitivesCount: i32,
    pub PhysicsReceiveKbps: f32,
    pub PhysicsSendKbps: f32,
    pub PhysicsStepTime: f32,
    pub PhysicsStepTimeMs: f32,
    pub PrimitivesCount: i32,
    pub RenderCpuFrameTime: f32,
    pub RenderGpuFrameTime: f32,
    pub SceneDrawcallCount: i32,
    pub SceneTriangleCount: i32,
    pub ShadowsDrawcallCount: i32,
    pub ShadowsTriangleCount: i32,
    pub Ui2dDrawcallCount: i32,
    pub Ui2dTriangleCount: i32,
    pub Ui3dDrawcallCount: i32,
    pub Ui3dTriangleCount: i32,
}
#[derive(Debug, Clone)]
pub struct StatsItem {
    pub DisplayName: String,
}
#[derive(Debug, Clone)]
pub struct Status {}
#[derive(Debug, Clone)]
pub struct StopWatchReporter {}
#[derive(Debug, Clone)]
pub struct StreamingService {}
#[derive(Debug, Clone)]
pub struct StringValue {
    pub Value: String,
}
#[derive(Debug, Clone)]
pub struct Studio {
    pub ActionOnAutoResumeSync: ActionOnAutoResumeSync,
    pub ActionOnStopSync: ActionOnStopSync,
    pub ActiveColor: Color3,
    pub ActiveHoverOverColor: Color3,
    pub AlwaysSaveScriptChanges: bool,
    pub AnimateHoverOver: bool,
    pub AutoCleanEmptyLine: bool,
    pub AutoClosingBrackets: bool,
    pub AutoClosingQuotes: bool,
    pub AutoDeleteClosingBracketsAndQuotes: bool,
    pub AutoIndentRule: AutoIndentRule,
    pub AutoRecoveryEnabled: bool,
    pub AutoRecoveryIntervalMinutes: i32,
    pub AutoResumeSyncOnPlaceOpen: bool,
    pub AutocompleteAcceptanceBehavior: CompletionAcceptanceBehavior,
    pub AutomaticallyTriggerAiCodeCompletion: bool,
    pub BackgroundColor: Color3,
    pub BasicObjectsDisplayMode: ListDisplayMode,
    pub BoolColor: Color3,
    pub BracketColor: Color3,
    pub BuiltInFunctionColor: Color3,
    pub CameraAdaptiveSpeed: bool,
    pub CameraMouseWheelSpeed: f32,
    pub CameraOrbitSensitivity: f32,
    pub CameraPanSensitivity: f32,
    pub CameraPanSpeed: f32,
    pub CameraShiftFactor: f32,
    pub CameraShiftSpeed: f32,
    pub CameraSpeed: f32,
    pub CameraSpeedAdjustBinding: CameraSpeedAdjustBinding,
    pub CameraTweenFocus: bool,
    pub CameraZoomSpeed: f32,
    pub CameraZoomToMousePosition: bool,
    pub ClearOutputOnStart: bool,
    pub CommandBarLocalState: bool,
    pub CommentColor: Color3,
    pub CurrentLineHighlightColor: Color3,
    pub DebuggerCurrentLineColor: Color3,
    pub DebuggerErrorLineColor: Color3,
    pub DeprecatedObjectsShown: bool,
    pub DisplayLanguage: String,
    pub DocViewCodeBackgroundColor: Color3,
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
    pub ErrorColor: Color3,
    pub FindSelectionBackgroundColor: Color3,
    pub FormatOnPaste: bool,
    pub FormatOnType: bool,
    pub FreeCameraSpeedScroll: bool,
    pub FunctionColor: Color3,
    pub FunctionNameColor: Color3,
    pub HighlightCurrentLine: bool,
    pub HighlightOccurances: bool,
    pub HintColor: Color3,
    pub HoverAnimateSpeed: HoverAnimateSpeed,
    pub HoverBoxThickness: f32,
    pub HoverLineThickness: i32,
    pub HoverOverColor: Color3,
    pub IndentUsingSpaces: bool,
    pub IndentationRulerColor: Color3,
    pub InformationColor: Color3,
    pub KeywordColor: Color3,
    pub LargeFileLineCountThreshold: i32,
    pub LargeFileThreshold: i32,
    pub LineThickness: f32,
    pub LoadAllBuiltinPluginsInRunModes: bool,
    pub LoadUserPluginsInRunModes: bool,
    pub LocalColor: Color3,
    pub LuaDebuggerEnabled: bool,
    pub LuaDebuggerEnabledAtStartup: bool,
    pub LuauKeywordColor: Color3,
    pub MainVolume: f32,
    pub MatchingWordBackgroundColor: Color3,
    pub MaximumOutputLines: i32,
    pub MenuItemBackgroundColor: Color3,
    pub MethodColor: Color3,
    pub NilColor: Color3,
    pub NumberColor: Color3,
    pub OnlyPlayAudioFromWindowInFocus: bool,
    pub OperatorColor: Color3,
    pub OutputLayoutMode: OutputLayoutMode,
    pub PermissionLevelShown: PermissionLevelShown,
    pub PhysicalDraggersSelectScopeByDefault: bool,
    pub PivotSnapToGeometryColor: Color3,
    pub PluginDebuggingEnabled: bool,
    pub PrimaryTextColor: Color3,
    pub PropertyColor: Color3,
    pub ReloadBuiltinPluginsOnChange: bool,
    pub ReloadLocalPluginsOnChange: bool,
    pub RespectStudioShortcutsWhenGameHasFocus: bool,
    pub RulerColor: Color3,
    pub Rulers: String,
    pub RuntimeUndoBehavior: RuntimeUndoBehavior,
    pub ScriptEditorColorPreset: StudioScriptEditorColorPresets,
    pub ScriptEditorMenuBorderColor: Color3,
    pub ScriptEditorScrollbarBackgroundColor: Color3,
    pub ScriptEditorScrollbarHandleColor: Color3,
    pub ScriptEditorShouldShowPluginMethods: bool,
    pub ScriptTimeoutLength: i32,
    pub ScrollPastLastLine: bool,
    pub SecondaryTextColor: Color3,
    pub SelectColor: Color3,
    pub SelectHoverColor: Color3,
    pub SelectedMenuItemBackgroundColor: Color3,
    pub SelectedTextColor: Color3,
    pub SelectionBackgroundColor: Color3,
    pub SelectionBoxThickness: f32,
    pub SelectionColor: Color3,
    pub SelectionLineThickness: i32,
    pub SelfColor: Color3,
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
    pub StringColor: Color3,
    pub TabWidth: i32,
    pub TextColor: Color3,
    pub TextWrapping: bool,
    pub Theme: Ref,
    pub TodoColor: Color3,
    pub TypeColor: Color3,
    pub UiTheme: UITheme,
    pub UseBoundingBoxMoveHandles: bool,
    pub WarningColor: Color3,
    pub WhitespaceColor: Color3,
}
#[derive(Debug, Clone)]
pub struct StudioAssetService {}
#[derive(Debug, Clone)]
pub struct StudioAttachment {
    pub AutoHideParent: bool,
    pub IsArrowVisible: bool,
    pub Offset: Vector2,
    pub SourceAnchorPoint: Vector2,
    pub TargetAnchorPoint: Vector2,
}
#[derive(Debug, Clone)]
pub struct StudioCallout {
    pub AnchorPoint: Vector2,
    pub IsArrowVisible: bool,
    pub IsNextVisible: bool,
    pub RowName: String,
    pub Text: String,
    pub Title: String,
}
#[derive(Debug, Clone)]
pub struct StudioCameraService {
    pub LockCameraSpeed: bool,
}
#[derive(Debug, Clone)]
pub struct StudioData {
    pub EnableScriptCollabByDefaultOnLoad: bool,
}
#[derive(Debug, Clone)]
pub struct StudioDeviceEmulatorService {
    pub HasMultiTouchStarted: bool,
    pub IsMultiTouchEmulationOn: bool,
    pub IsMultiTouchEnabled: bool,
    pub PivotPosition: Vector2,
}
#[derive(Debug, Clone)]
pub struct StudioObjectBase {}
#[derive(Debug, Clone)]
pub struct StudioPublishService {
    pub PublishLocked: bool,
}
#[derive(Debug, Clone)]
pub struct StudioScriptDebugEventListener {}
#[derive(Debug, Clone)]
pub struct StudioSdkService {}
#[derive(Debug, Clone)]
pub struct StudioService {
    pub ActiveScript: Ref,
    pub AlignDraggedObjects: bool,
    pub DraggerSolveConstraints: bool,
    pub DrawConstraintsOnTop: bool,
    pub GridSize: f32,
    pub HoverInstance: Ref,
    pub InstalledPluginData: String,
    pub PivotSnapToGeometry: bool,
    pub RotateIncrement: f32,
    pub Secrets: String,
    pub ShowConstraintDetails: bool,
    pub ShowWeldDetails: bool,
    pub StudioLocaleId: String,
    pub UseLocalSpace: bool,
}
#[derive(Debug, Clone)]
pub struct StudioTheme {}
#[derive(Debug, Clone)]
pub struct StudioUserService {
    pub IsLoggedIn: bool,
}
#[derive(Debug, Clone)]
pub struct StudioWidget {}
#[derive(Debug, Clone)]
pub struct StudioWidgetsService {}
#[derive(Debug, Clone)]
pub struct StyleBase {}
#[derive(Debug, Clone)]
pub struct StyleDerive {
    pub Index: i32,
    pub StyleSheet: Ref,
}
#[derive(Debug, Clone)]
pub struct StyleLink {
    pub StyleSheet: Ref,
}
#[derive(Debug, Clone)]
pub struct StyleRule {
    pub Index: i32,
    pub Priority: i32,
    pub PropertiesSerialize: BinaryString,
    pub Selector: String,
    pub SelectorError: String,
}
#[derive(Debug, Clone)]
pub struct StyleSheet {}
#[derive(Debug, Clone)]
pub struct StylingService {}
#[derive(Debug, Clone)]
pub struct SunRaysEffect {
    pub Intensity: f32,
    pub Spread: f32,
}
#[derive(Debug, Clone)]
pub struct SurfaceAppearance {
    pub AlphaMode: AlphaMode,
    pub Color: Color3,
    pub ColorMap: ContentId,
    pub ColorMapContent: Content,
    pub MetalnessMap: ContentId,
    pub MetalnessMapContent: Content,
    pub NormalMap: ContentId,
    pub NormalMapContent: Content,
    pub RoughnessMap: ContentId,
    pub RoughnessMapContent: Content,
    pub TexturePack: ContentId,
}
#[derive(Debug, Clone)]
pub struct SurfaceGui {
    pub AlwaysOnTop: bool,
    pub Brightness: f32,
    pub CanvasSize: Vector2,
    pub ClipsDescendants: bool,
    pub HorizontalCurvature: f32,
    pub LightInfluence: f32,
    pub MaxDistance: f32,
    pub PixelsPerStud: f32,
    pub Shape: SurfaceGuiShape,
    pub SizingMode: SurfaceGuiSizingMode,
    pub ToolPunchThroughDistance: f32,
    pub ZOffset: f32,
}
#[derive(Debug, Clone)]
pub struct SurfaceGuiBase {
    pub Active: bool,
    pub Adornee: Ref,
    pub Face: NormalId,
}
#[derive(Debug, Clone)]
pub struct SurfaceLight {
    pub Angle: f32,
    pub Face: NormalId,
    pub Range: f32,
}
#[derive(Debug, Clone)]
pub struct SurfaceSelection {
    pub TargetSurface: NormalId,
}
#[derive(Debug, Clone)]
pub struct SwimController {
    pub AccelerationTime: f32,
    pub PitchMaxTorque: f32,
    pub PitchSpeedFactor: f32,
    pub RollMaxTorque: f32,
    pub RollSpeedFactor: f32,
}
#[derive(Debug, Clone)]
pub struct SyncScriptBuilder {
    pub CompileTarget: CompileTarget,
    pub CoverageInfo: bool,
    pub DebugInfo: bool,
    pub PackAsSource: bool,
    pub RawBytecode: bool,
}
#[derive(Debug, Clone)]
pub struct SystemThemeService {}
#[derive(Debug, Clone)]
pub struct TaskScheduler {
    pub SchedulerDutyCycle: f64,
    pub SchedulerRate: f64,
    pub ThreadPoolConfig: ThreadPoolConfig,
    pub ThreadPoolSize: i32,
}
#[derive(Debug, Clone)]
pub struct Team {
    pub AutoAssignable: bool,
    pub AutoColorCharacters: bool,
    pub ChildOrder: i32,
    pub Score: i32,
    pub TeamColor: BrickColor,
}
#[derive(Debug, Clone)]
pub struct TeamCreateData {
    pub InitialCameraCFrame: CFrame,
}
#[derive(Debug, Clone)]
pub struct TeamCreatePublishService {}
#[derive(Debug, Clone)]
pub struct TeamCreateService {}
#[derive(Debug, Clone)]
pub struct Teams {}
#[derive(Debug, Clone)]
pub struct TelemetryService {}
#[derive(Debug, Clone)]
pub struct TeleportAsyncResult {
    pub PrivateServerId: String,
    pub ReservedServerAccessCode: String,
}
#[derive(Debug, Clone)]
pub struct TeleportOptions {
    pub ReservedServerAccessCode: String,
    pub ServerInstanceId: String,
    pub ShouldReserveServer: bool,
}
#[derive(Debug, Clone)]
pub struct TeleportService {
    pub CustomizedTeleportUi: bool,
}
#[derive(Debug, Clone)]
pub struct TemporaryCageMeshProvider {}
#[derive(Debug, Clone)]
pub struct TemporaryScriptService {}
#[derive(Debug, Clone)]
pub struct Terrain {
    pub AcquisitionMethod: TerrainAcquisitionMethod,
    pub ClusterGrid: String,
    pub ClusterGridV2: String,
    pub ClusterGridV3: BinaryString,
    pub Decoration: bool,
    pub GrassLength: f32,
    pub IsSmooth: bool,
    pub LastUsedModificationMethod: TerrainAcquisitionMethod,
    pub MaterialColors: MaterialColors,
    pub MaxExtents: Region3int16,
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
pub struct TerrainDetail {
    pub ColorMap: ContentId,
    pub Face: TerrainFace,
    pub MaterialPattern: MaterialPattern,
    pub MetalnessMap: ContentId,
    pub NormalMap: ContentId,
    pub RoughnessMap: ContentId,
    pub StudsPerTile: f32,
    pub TexturePack: ContentId,
}
#[derive(Debug, Clone)]
pub struct TerrainRegion {
    pub ExtentsMax: Vector3int16,
    pub ExtentsMin: Vector3int16,
    pub GridV3: BinaryString,
    pub IsSmooth: bool,
    pub SizeInCells: Vector3,
    pub SmoothGrid: BinaryString,
}
#[derive(Debug, Clone)]
pub struct TestService {
    pub AutoRuns: bool,
    pub Description: String,
    pub ErrorCount: i32,
    pub ExecuteWithStudioRun: bool,
    pub Is30FpsThrottleEnabled: bool,
    pub IsPhysicsEnvironmentalThrottled: bool,
    pub IsSleepAllowed: bool,
    pub NumberOfPlayers: i32,
    pub SimulateSecondsLag: f64,
    pub TestCount: i32,
    pub ThrottlePhysicsToRealtime: bool,
    pub Timeout: f64,
    pub WarnCount: i32,
}
#[derive(Debug, Clone)]
pub struct TextBox {
    pub ClearTextOnFocus: bool,
    pub Confidential: bool,
    pub ContentText: String,
    pub CursorPosition: i32,
    pub Font: Font,
    pub FontFace: Font,
    pub FontSize: FontSize,
    pub LineHeight: f32,
    pub LocalizationMatchIdentifier: String,
    pub LocalizationMatchedSourceText: String,
    pub ManualFocusRelease: bool,
    pub MaxVisibleGraphemes: i32,
    pub MultiLine: bool,
    pub OpenTypeFeatures: String,
    pub OpenTypeFeaturesError: String,
    pub OverlayNativeInput: bool,
    pub PlaceholderColor3: Color3,
    pub PlaceholderText: String,
    pub ReturnKeyType: ReturnKeyType,
    pub RichText: bool,
    pub SelectionStart: i32,
    pub ShouldEmitReturnEvents: bool,
    pub ShouldEmitUpAndDownArrowEvents: bool,
    pub ShowNativeInput: bool,
    pub Text: String,
    pub TextBounds: Vector2,
    pub TextColor: BrickColor,
    pub TextColor3: Color3,
    pub TextDirection: TextDirection,
    pub TextEditable: bool,
    pub TextFits: bool,
    pub TextInputType: TextInputType,
    pub TextScaled: bool,
    pub TextSize: f32,
    pub TextStrokeColor3: Color3,
    pub TextStrokeTransparency: f32,
    pub TextTransparency: f32,
    pub TextTruncate: TextTruncate,
    pub TextWrap: bool,
    pub TextWrapped: bool,
    pub TextXAlignment: TextXAlignment,
    pub TextYAlignment: TextYAlignment,
}
#[derive(Debug, Clone)]
pub struct TextBoxService {}
#[derive(Debug, Clone)]
pub struct TextButton {
    pub Confidential: bool,
    pub ContentText: String,
    pub Font: Font,
    pub FontFace: Font,
    pub FontSize: FontSize,
    pub LineHeight: f32,
    pub LocalizationMatchIdentifier: String,
    pub LocalizationMatchedSourceText: String,
    pub LocalizedText: String,
    pub MaxVisibleGraphemes: i32,
    pub OpenTypeFeatures: String,
    pub OpenTypeFeaturesError: String,
    pub RichText: bool,
    pub Text: String,
    pub TextBounds: Vector2,
    pub TextColor: BrickColor,
    pub TextColor3: Color3,
    pub TextDirection: TextDirection,
    pub TextFits: bool,
    pub TextScaled: bool,
    pub TextSize: f32,
    pub TextStrokeColor3: Color3,
    pub TextStrokeTransparency: f32,
    pub TextTransparency: f32,
    pub TextTruncate: TextTruncate,
    pub TextWrap: bool,
    pub TextWrapped: bool,
    pub TextXAlignment: TextXAlignment,
    pub TextYAlignment: TextYAlignment,
}
#[derive(Debug, Clone)]
pub struct TextChannel {
    pub DirectChatRequester: Ref,
}
#[derive(Debug, Clone)]
pub struct TextChatCommand {
    pub AutocompleteVisible: bool,
    pub Enabled: bool,
    pub PrimaryAlias: String,
    pub SecondaryAlias: String,
}
#[derive(Debug, Clone)]
pub struct TextChatConfigurations {}
#[derive(Debug, Clone)]
pub struct TextChatMessage {
    pub BubbleChatMessageProperties: Ref,
    pub ChatWindowMessageProperties: Ref,
    pub MessageId: String,
    pub Metadata: String,
    pub PrefixText: String,
    pub Status: TextChatMessageStatus,
    pub Text: String,
    pub TextChannel: Ref,
    pub TextSource: Ref,
    pub Translation: String,
    pub Verified: bool,
}
#[derive(Debug, Clone)]
pub struct TextChatMessageProperties {
    pub PrefixText: String,
    pub Text: String,
    pub Translation: String,
}
#[derive(Debug, Clone)]
pub struct TextChatService {
    pub ChatTranslationEnabled: bool,
    pub ChatTranslationFtuxShown: bool,
    pub ChatTranslationToggleEnabled: bool,
    pub ChatVersion: ChatVersion,
    pub CreateDefaultCommands: bool,
    pub CreateDefaultTextChannels: bool,
    pub HasSeenDeprecationDialog: bool,
}
#[derive(Debug, Clone)]
pub struct TextFilterResult {}
#[derive(Debug, Clone)]
pub struct TextFilterTranslatedResult {
    pub SourceLanguage: String,
    pub SourceText: Ref,
}
#[derive(Debug, Clone)]
pub struct TextLabel {
    pub Confidential: bool,
    pub ContentText: String,
    pub Font: Font,
    pub FontFace: Font,
    pub FontSize: FontSize,
    pub LineHeight: f32,
    pub LocalizationMatchIdentifier: String,
    pub LocalizationMatchedSourceText: String,
    pub LocalizedText: String,
    pub MaxVisibleGraphemes: i32,
    pub OpenTypeFeatures: String,
    pub OpenTypeFeaturesError: String,
    pub RichText: bool,
    pub Text: String,
    pub TextBounds: Vector2,
    pub TextColor: BrickColor,
    pub TextColor3: Color3,
    pub TextDirection: TextDirection,
    pub TextFits: bool,
    pub TextScaled: bool,
    pub TextSize: f32,
    pub TextStrokeColor3: Color3,
    pub TextStrokeTransparency: f32,
    pub TextTransparency: f32,
    pub TextTruncate: TextTruncate,
    pub TextWrap: bool,
    pub TextWrapped: bool,
    pub TextXAlignment: TextXAlignment,
    pub TextYAlignment: TextYAlignment,
}
#[derive(Debug, Clone)]
pub struct TextService {}
#[derive(Debug, Clone)]
pub struct TextSource {
    pub CanSend: bool,
    pub UserId: i64,
    pub UserIdReplicated: i64,
}
#[derive(Debug, Clone)]
pub struct Texture {
    pub OffsetStudsU: f32,
    pub OffsetStudsV: f32,
    pub StudsPerTileU: f32,
    pub StudsPerTileV: f32,
}
#[derive(Debug, Clone)]
pub struct TextureGenerationPartGroup {}
#[derive(Debug, Clone)]
pub struct TextureGenerationService {}
#[derive(Debug, Clone)]
pub struct TextureGenerationUnwrappingRequest {}
#[derive(Debug, Clone)]
pub struct ThirdPartyUserService {}
#[derive(Debug, Clone)]
pub struct ThreadState {
    pub FrameCount: i32,
    pub Populated: bool,
    pub ThreadId: i32,
    pub ThreadName: String,
}
#[derive(Debug, Clone)]
pub struct TimerService {}
#[derive(Debug, Clone)]
pub struct ToastNotificationService {}
#[derive(Debug, Clone)]
pub struct Tool {
    pub CanBeDropped: bool,
    pub Enabled: bool,
    pub Grip: CFrame,
    pub GripForward: Vector3,
    pub GripPos: Vector3,
    pub GripRight: Vector3,
    pub GripUp: Vector3,
    pub ManualActivationOnly: bool,
    pub RequiresHandle: bool,
    pub ToolTip: String,
}
#[derive(Debug, Clone)]
pub struct Torque {
    pub RelativeTo: ActuatorRelativeTo,
    pub Torque: Vector3,
}
#[derive(Debug, Clone)]
pub struct TorsionSpringConstraint {
    pub Coils: f32,
    pub CurrentAngle: f32,
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
pub struct TotalCountTimeIntervalItem {}
#[derive(Debug, Clone)]
pub struct TouchInputService {}
#[derive(Debug, Clone)]
pub struct TouchTransmitter {}
#[derive(Debug, Clone)]
pub struct TracerService {}
#[derive(Debug, Clone)]
pub struct TrackerLodController {
    pub AudioMode: TrackerLodFlagMode,
    pub VideoExtrapolationMode: TrackerExtrapolationFlagMode,
    pub VideoLodMode: TrackerLodValueMode,
    pub VideoMode: TrackerLodFlagMode,
}
#[derive(Debug, Clone)]
pub struct TrackerStreamAnimation {}
#[derive(Debug, Clone)]
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
    pub LocalTransparencyModifier: f32,
    pub MaxLength: f32,
    pub MinLength: f32,
    pub Texture: ContentId,
    pub TextureLength: f32,
    pub TextureMode: TextureMode,
    pub Transparency: NumberSequence,
    pub WidthScale: NumberSequence,
}
#[derive(Debug, Clone)]
pub struct Translator {
    pub LocaleId: String,
}
#[derive(Debug, Clone)]
pub struct TremoloSoundEffect {
    pub Depth: f32,
    pub Duty: f32,
    pub Frequency: f32,
}
#[derive(Debug, Clone)]
pub struct TriangleMeshPart {
    pub AeroMeshData: SharedString,
    pub CollisionFidelity: CollisionFidelity,
    pub FluidFidelity: FluidFidelity,
    pub FluidFidelityInternal: FluidFidelity,
    pub MeshSize: Vector3,
    pub PhysicalConfigData: SharedString,
    pub UnscaledCofm: Vector3,
    pub UnscaledVolInertiaDiags: Vector3,
    pub UnscaledVolInertiaOffDiags: Vector3,
    pub UnscaledVolume: f32,
}
#[derive(Debug, Clone)]
pub struct TrussPart {
    pub Style: Style,
    pub Style: Style,
}
#[derive(Debug, Clone)]
pub struct TutorialService {}
#[derive(Debug, Clone)]
pub struct Tween {
    pub Instance: Ref,
}
#[derive(Debug, Clone)]
pub struct TweenBase {
    pub PlaybackState: PlaybackState,
}
#[derive(Debug, Clone)]
pub struct TweenService {}
#[derive(Debug, Clone)]
pub struct UGCAvatarService {}
#[derive(Debug, Clone)]
pub struct UGCValidationService {}
#[derive(Debug, Clone)]
pub struct UIAspectRatioConstraint {
    pub AspectRatio: f32,
    pub AspectType: AspectType,
    pub DominantAxis: DominantAxis,
}
#[derive(Debug, Clone)]
pub struct UIBase {}
#[derive(Debug, Clone)]
pub struct UIComponent {}
#[derive(Debug, Clone)]
pub struct UIConstraint {}
#[derive(Debug, Clone)]
pub struct UICorner {
    pub CornerRadius: UDim,
}
#[derive(Debug, Clone)]
pub struct UIDragDetector {
    pub ActivatedCursorIcon: ContentId,
    pub BoundingBehavior: UIDragDetectorBoundingBehavior,
    pub BoundingUi: Ref,
    pub CursorIcon: ContentId,
    pub DragAxis: Vector2,
    pub DragRelativity: UIDragDetectorDragRelativity,
    pub DragRotation: f32,
    pub DragSpace: UIDragDetectorDragSpace,
    pub DragStyle: UIDragDetectorDragStyle,
    pub DragUDim2: UDim2,
    pub Enabled: bool,
    pub MaxDragAngle: f32,
    pub MaxDragTranslation: UDim2,
    pub MinDragAngle: f32,
    pub MinDragTranslation: UDim2,
    pub ReferenceUiInstance: Ref,
    pub ResponseStyle: UIDragDetectorResponseStyle,
    pub SelectionModeDragSpeed: UDim2,
    pub SelectionModeRotateSpeed: f32,
    pub UiDragSpeedAxisMapping: UIDragSpeedAxisMapping,
}
#[derive(Debug, Clone)]
pub struct UIDragDetectorService {}
#[derive(Debug, Clone)]
pub struct UIFlexItem {
    pub FlexMode: UIFlexMode,
    pub GrowRatio: f32,
    pub ItemLineAlignment: ItemLineAlignment,
    pub ShrinkRatio: f32,
}
#[derive(Debug, Clone)]
pub struct UIGradient {
    pub Color: ColorSequence,
    pub Enabled: bool,
    pub Offset: Vector2,
    pub Rotation: f32,
    pub Transparency: NumberSequence,
}
#[derive(Debug, Clone)]
pub struct UIGridLayout {
    pub AbsoluteCellCount: Vector2,
    pub AbsoluteCellSize: Vector2,
    pub CellPadding: UDim2,
    pub CellSize: UDim2,
    pub FillDirectionMaxCells: i32,
    pub StartCorner: StartCorner,
}
#[derive(Debug, Clone)]
pub struct UIGridStyleLayout {
    pub AbsoluteContentSize: Vector2,
    pub FillDirection: FillDirection,
    pub HorizontalAlignment: HorizontalAlignment,
    pub SortOrder: SortOrder,
    pub VerticalAlignment: VerticalAlignment,
}
#[derive(Debug, Clone)]
pub struct UILayout {}
#[derive(Debug, Clone)]
pub struct UIListLayout {
    pub HorizontalFlex: UIFlexAlignment,
    pub HorizontalPadding: UDim,
    pub ItemLineAlignment: ItemLineAlignment,
    pub Padding: UDim,
    pub VerticalFlex: UIFlexAlignment,
    pub VerticalPadding: UDim,
    pub Wraps: bool,
}
#[derive(Debug, Clone)]
pub struct UIPadding {
    pub PaddingBottom: UDim,
    pub PaddingLeft: UDim,
    pub PaddingRight: UDim,
    pub PaddingTop: UDim,
}
#[derive(Debug, Clone)]
pub struct UIPageLayout {
    pub Animated: bool,
    pub Circular: bool,
    pub CurrentPage: Ref,
    pub EasingDirection: EasingDirection,
    pub EasingStyle: EasingStyle,
    pub GamepadInputEnabled: bool,
    pub Padding: UDim,
    pub ScrollWheelInputEnabled: bool,
    pub TouchInputEnabled: bool,
    pub TweenTime: f32,
}
#[derive(Debug, Clone)]
pub struct UIScale {
    pub Scale: f32,
}
#[derive(Debug, Clone)]
pub struct UISizeConstraint {
    pub MaxSize: Vector2,
    pub MinSize: Vector2,
}
#[derive(Debug, Clone)]
pub struct UIStroke {
    pub ApplyStrokeMode: ApplyStrokeMode,
    pub Color: Color3,
    pub Enabled: bool,
    pub LineJoinMode: LineJoinMode,
    pub Thickness: f32,
    pub Transparency: f32,
}
#[derive(Debug, Clone)]
pub struct UITableLayout {
    pub FillEmptySpaceColumns: bool,
    pub FillEmptySpaceRows: bool,
    pub MajorAxis: TableMajorAxis,
    pub Padding: UDim2,
}
#[derive(Debug, Clone)]
pub struct UITextSizeConstraint {
    pub MaxTextSize: i32,
    pub MinTextSize: i32,
}
#[derive(Debug, Clone)]
pub struct UnionOperation {}
#[derive(Debug, Clone)]
pub struct UniqueIdLookupService {}
#[derive(Debug, Clone)]
pub struct UniversalConstraint {
    pub LimitsEnabled: bool,
    pub MaxAngle: f32,
    pub Radius: f32,
    pub Restitution: f32,
}
#[derive(Debug, Clone)]
pub struct UnreliableRemoteEvent {}
#[derive(Debug, Clone)]
pub struct UnvalidatedAssetService {
    pub CachedData: String,
}
#[derive(Debug, Clone)]
pub struct UserGameSettings {
    pub AllTutorialsDisabled: bool,
    pub CameraMode: CustomCameraMode,
    pub CameraYInverted: bool,
    pub ChatTranslationEnabled: bool,
    pub ChatTranslationFtuxShown: bool,
    pub ChatTranslationLocale: String,
    pub ChatTranslationToggleEnabled: bool,
    pub ChatVisible: bool,
    pub CompletedTutorials: String,
    pub ComputerCameraMovementChanged: bool,
    pub ComputerCameraMovementMode: ComputerCameraMovementMode,
    pub ComputerMovementChanged: bool,
    pub ComputerMovementMode: ComputerMovementMode,
    pub ControlMode: ControlMode,
    pub DefaultCameraId: String,
    pub FramerateCap: i32,
    pub Fullscreen: bool,
    pub GaId: String,
    pub GamepadCameraSensitivity: f32,
    pub GraphicsOptimizationMode: GraphicsOptimizationMode,
    pub GraphicsQualityLevel: i32,
    pub HapticStrength: f32,
    pub HasEverUsedVr: bool,
    pub IsUsingCameraYInverted: bool,
    pub IsUsingGamepadCameraSensitivity: bool,
    pub MasterVolume: f32,
    pub MasterVolumeStudio: f32,
    pub MaxQualityEnabled: bool,
    pub MicroProfilerWebServerEnabled: bool,
    pub MicroProfilerWebServerIp: String,
    pub MicroProfilerWebServerPort: i32,
    pub MouseSensitivity: f32,
    pub MouseSensitivityFirstPerson: Vector2,
    pub MouseSensitivityThirdPerson: Vector2,
    pub OnScreenProfilerEnabled: bool,
    pub OnboardingsCompleted: String,
    pub PartyVoiceVolume: f32,
    pub PerformanceStatsVisible: bool,
    pub PlayerHeight: f32,
    pub PreferredTextSize: PreferredTextSize,
    pub PreferredTransparency: f32,
    pub QualityResetLevel: i32,
    pub RccProfilerRecordFrameRate: i32,
    pub RccProfilerRecordTimeFrame: i32,
    pub ReducedMotion: bool,
    pub RotationType: RotationType,
    pub SavedQualityLevel: SavedQualitySetting,
    pub StartMaximized: bool,
    pub StartScreenPosition: Vector2,
    pub StartScreenSize: Vector2,
    pub TouchCameraMovementChanged: bool,
    pub TouchCameraMovementMode: TouchCameraMovementMode,
    pub TouchMovementChanged: bool,
    pub TouchMovementMode: TouchMovementMode,
    pub UiNavigationKeyBindEnabled: bool,
    pub UsedCoreGuiIsVisibleToggle: bool,
    pub UsedCustomGuiIsVisibleToggle: bool,
    pub UsedHideHudShortcut: bool,
    pub VignetteEnabled: bool,
    pub VignetteEnabledCustomOption: bool,
    pub VrComfortSetting: VRComfortSetting,
    pub VrEnabled: bool,
    pub VrRotationIntensity: i32,
    pub VrSafetyBubbleMode: VRSafetyBubbleMode,
    pub VrSmoothRotationEnabled: bool,
    pub VrSmoothRotationEnabledCustomOption: bool,
    pub VrThirdPersonFollowCamEnabled: bool,
    pub VrThirdPersonFollowCamEnabledCustomOption: bool,
}
#[derive(Debug, Clone)]
pub struct UserInputService {
    pub AccelerometerEnabled: bool,
    pub BottomBarSize: Vector2,
    pub GamepadEnabled: bool,
    pub GyroscopeEnabled: bool,
    pub KeyboardEnabled: bool,
    pub LegacyInputEventsEnabled: bool,
    pub ModalEnabled: bool,
    pub MouseBehavior: MouseBehavior,
    pub MouseDeltaSensitivity: f32,
    pub MouseEnabled: bool,
    pub MouseIcon: ContentId,
    pub MouseIconEnabled: bool,
    pub NavBarSize: Vector2,
    pub OnScreenKeyboardAnimationDuration: f64,
    pub OnScreenKeyboardPosition: Vector2,
    pub OnScreenKeyboardSize: Vector2,
    pub OnScreenKeyboardVisible: bool,
    pub OverrideMouseIconBehavior: OverrideMouseIconBehavior,
    pub RightBarSize: Vector2,
    pub StatusBarSize: Vector2,
    pub TouchEnabled: bool,
    pub UserHeadCFrame: CFrame,
    pub VrEnabled: bool,
}
#[derive(Debug, Clone)]
pub struct UserService {}
#[derive(Debug, Clone)]
pub struct UserSettings {}
#[derive(Debug, Clone)]
pub struct UserStorageService {}
#[derive(Debug, Clone)]
pub struct VRService {
    pub AutomaticScaling: VRScaling,
    pub AvatarGestures: bool,
    pub ControllerModels: VRControllerModelMode,
    pub DidPointerHit: bool,
    pub FadeOutViewOnCollision: bool,
    pub GuiInputUserCFrame: UserCFrame,
    pub LaserDistance: f32,
    pub LaserPointer: VRLaserPointerMode,
    pub PointerHitCFrame: CFrame,
    pub QuestAswState: bool,
    pub QuestDisplayRefreshRate: f32,
    pub ThirdPersonFollowCamEnabled: bool,
    pub VrDeviceAvailable: bool,
    pub VrDeviceName: String,
    pub VrEnabled: bool,
    pub VrSessionState: VRSessionState,
}
#[derive(Debug, Clone)]
pub struct VRStatusService {}
#[derive(Debug, Clone)]
pub struct ValueBase {}
#[derive(Debug, Clone)]
pub struct Vector3Curve {}
#[derive(Debug, Clone)]
pub struct Vector3Value {
    pub Value: Vector3,
}
#[derive(Debug, Clone)]
pub struct VectorForce {
    pub ApplyAtCenterOfMass: bool,
    pub Force: Vector3,
    pub RelativeTo: ActuatorRelativeTo,
}
#[derive(Debug, Clone)]
pub struct VehicleController {}
#[derive(Debug, Clone)]
pub struct VehicleSeat {
    pub AreHingesDetected: i32,
    pub Disabled: bool,
    pub HeadsUpDisplay: bool,
    pub MaxSpeed: f32,
    pub Occupant: Ref,
    pub Steer: i32,
    pub SteerFloat: f32,
    pub Throttle: i32,
    pub ThrottleFloat: f32,
    pub Torque: f32,
    pub TurnSpeed: f32,
}
#[derive(Debug, Clone)]
pub struct VelocityMotor {
    pub CurrentAngle: f32,
    pub DesiredAngle: f32,
    pub Hole: Ref,
    pub MaxVelocity: f32,
}
#[derive(Debug, Clone)]
pub struct VersionControlService {
    pub ScriptCollabEnabled: bool,
    pub ScriptCollabVersionHistoryEnabled: bool,
}
#[derive(Debug, Clone)]
pub struct VideoCaptureService {
    pub Active: bool,
    pub CameraId: String,
}
#[derive(Debug, Clone)]
pub struct VideoDeviceInput {
    pub Active: bool,
    pub CameraId: String,
    pub CaptureQuality: VideoDeviceCaptureQuality,
    pub IsReady: bool,
}
#[derive(Debug, Clone)]
pub struct VideoDisplay {
    pub ResampleMode: ResamplerMode,
    pub ScaleType: ScaleType,
    pub TileSize: UDim2,
    pub VideoColor3: Color3,
    pub VideoRectOffset: Vector2,
    pub VideoRectSize: Vector2,
    pub VideoTransparency: f32,
}
#[derive(Debug, Clone)]
pub struct VideoFrame {
    pub IsLoaded: bool,
    pub Looped: bool,
    pub Playing: bool,
    pub PlayingReplicating: bool,
    pub Resolution: Vector2,
    pub TimeLength: f64,
    pub TimePosition: f64,
    pub TimePositionReplicating: f64,
    pub Video: ContentId,
    pub Volume: f32,
}
#[derive(Debug, Clone)]
pub struct VideoPlayer {
    pub Asset: ContentId,
    pub AutoLoadInStudio: bool,
    pub AutoPlayInStudio: bool,
    pub IsLoaded: bool,
    pub IsPlaying: bool,
    pub Looping: bool,
    pub PlaybackSpeed: f32,
    pub Resolution: Vector2,
    pub TimeLength: f64,
    pub TimePosition: f64,
    pub Volume: f32,
}
#[derive(Debug, Clone)]
pub struct VideoService {}
#[derive(Debug, Clone)]
pub struct ViewportFrame {
    pub Ambient: Color3,
    pub CameraCFrame: CFrame,
    pub CameraFieldOfView: f32,
    pub CurrentCamera: Ref,
    pub ImageColor3: Color3,
    pub ImageTransparency: f32,
    pub IsMirrored: bool,
    pub LightColor: Color3,
    pub LightDirection: Vector3,
}
#[derive(Debug, Clone)]
pub struct VirtualInputManager {
    pub AdditionalLuaState: String,
}
#[derive(Debug, Clone)]
pub struct VirtualUser {}
#[derive(Debug, Clone)]
pub struct VisibilityCheckDispatcher {}
#[derive(Debug, Clone)]
pub struct Visit {}
#[derive(Debug, Clone)]
pub struct VisualizationMode {
    pub Enabled: bool,
    pub Title: String,
    pub ToolTip: String,
}
#[derive(Debug, Clone)]
pub struct VisualizationModeCategory {
    pub Enabled: bool,
    pub Title: String,
}
#[derive(Debug, Clone)]
pub struct VisualizationModeService {}
#[derive(Debug, Clone)]
pub struct VoiceChatInternal {
    pub VoiceChatState: VoiceChatState,
}
#[derive(Debug, Clone)]
pub struct VoiceChatService {
    pub DefaultDistanceAttenuation: VoiceChatDistanceAttenuationType,
    pub EnableDefaultVoice: bool,
    pub UseAudioApi: AudioApiRollout,
    pub UseNewAudioApi: bool,
    pub UseNewControlPaths: bool,
    pub UseNewJoinFlow: bool,
    pub VoiceChatEnabledForPlaceOnRcc: bool,
    pub VoiceChatEnabledForUniverseOnRcc: bool,
}
#[derive(Debug, Clone)]
pub struct WebSocketClient {
    pub ConnectionState: WebSocketState,
}
#[derive(Debug, Clone)]
pub struct WebSocketService {}
#[derive(Debug, Clone)]
pub struct WebViewService {}
#[derive(Debug, Clone)]
pub struct WedgePart {}
#[derive(Debug, Clone)]
pub struct Weld {}
#[derive(Debug, Clone)]
pub struct WeldConstraint {
    pub Active: bool,
    pub CFrame0: CFrame,
    pub CFrame1: CFrame,
    pub Enabled: bool,
    pub Part0: Ref,
    pub Part0Internal: Ref,
    pub Part1: Ref,
    pub Part1Internal: Ref,
    pub State: i32,
}
#[derive(Debug, Clone)]
pub struct Wire {
    pub Connected: bool,
    pub SourceInstance: Ref,
    pub SourceName: String,
    pub TargetInstance: Ref,
    pub TargetName: String,
}
#[derive(Debug, Clone)]
pub struct WireframeHandleAdornment {
    pub Scale: Vector3,
    pub Thickness: f32,
}
#[derive(Debug, Clone)]
pub struct Workspace {
    pub AirDensity: f32,
    pub AllowThirdPartySales: bool,
    pub AvatarUnificationMode: AvatarUnificationMode,
    pub ClientAnimatorThrottling: ClientAnimatorThrottlingMode,
    pub CollisionGroupData: BinaryString,
    pub CollisionGroups: String,
    pub CurrentCamera: Ref,
    pub DataModelPlaceVersion: i32,
    pub DistributedGameTime: f64,
    pub ExplicitAutoJoints: bool,
    pub FallHeightEnabled: bool,
    pub FallenPartsDestroyHeight: f32,
    pub FilteringEnabled: bool,
    pub FluidForces: FluidForces,
    pub GlobalWind: Vector3,
    pub Gravity: f32,
    pub IkControlConstraintSupport: IKControlConstraintSupport,
    pub InsertPoint: Vector3,
    pub InterpolationThrottling: InterpolationThrottlingMode,
    pub MeshPartHeadsAndAccessories: MeshPartHeadsAndAccessories,
    pub ModelStreamingBehavior: ModelStreamingBehavior,
    pub MoverConstraintRootBehavior: MoverConstraintRootBehaviorMode,
    pub PathfindingUseImprovedSearch: PathfindingUseImprovedSearch,
    pub PhysicsImprovedSleep: RolloutState,
    pub PhysicsSteppingMethod: PhysicsSteppingMethod,
    pub PlayerCharacterDestroyBehavior: PlayerCharacterDestroyBehavior,
    pub PrimalPhysicsSolver: PrimalPhysicsSolver,
    pub RejectCharacterDeletions: RejectCharacterDeletions,
    pub RenderingCacheOptimizations: RenderingCacheOptimizationMode,
    pub ReplicateInstanceDestroySetting: ReplicateInstanceDestroySetting,
    pub Retargeting: AnimatorRetargetingMode,
    pub SandboxedInstanceMode: SandboxedInstanceMode,
    pub SignalBehavior: SignalBehavior,
    pub SignalBehavior2: SignalBehavior,
    pub StreamOutBehavior: StreamOutBehavior,
    pub StreamingEnabled: bool,
    pub StreamingIntegrityMode: StreamingIntegrityMode,
    pub StreamingMinRadius: i32,
    pub StreamingPauseMode: StreamingPauseMode,
    pub StreamingTargetRadius: i32,
    pub Terrain: Ref,
    pub TerrainWeldsFixed: bool,
    pub TouchEventsUseCollisionGroups: RolloutState,
    pub TouchesUseCollisionGroups: bool,
}
#[derive(Debug, Clone)]
pub struct WorkspaceAnnotation {
    pub Adornee: Ref,
    pub AdorneeOffset: Vector3,
}
#[derive(Debug, Clone)]
pub struct WorldModel {}
#[derive(Debug, Clone)]
pub struct WorldRoot {}
#[derive(Debug, Clone)]
pub struct WrapDeformer {}
#[derive(Debug, Clone)]
pub struct WrapLayer {
    pub AutoSkin: WrapLayerAutoSkin,
    pub BindOffset: CFrame,
    pub Color: Color3,
    pub DebugMode: WrapLayerDebugMode,
    pub Enabled: bool,
    pub Order: i32,
    pub Puffiness: f32,
    pub ReferenceMeshContent: Content,
    pub ReferenceMeshId: ContentId,
    pub ReferenceOrigin: CFrame,
    pub ReferenceOriginWorld: CFrame,
    pub ShrinkFactor: f32,
    pub TemporaryReferenceId: ContentId,
}
#[derive(Debug, Clone)]
pub struct WrapTarget {
    pub Color: Color3,
    pub DebugMode: WrapTargetDebugMode,
    pub Stiffness: f32,
}
