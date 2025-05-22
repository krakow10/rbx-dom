#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum StrongEnum {
    AccessModifierType(AccessModifierType),
    AccessoryType(AccessoryType),
    ActionOnAutoResumeSync(ActionOnAutoResumeSync),
    ActionOnStopSync(ActionOnStopSync),
    ActionType(ActionType),
    ActuatorRelativeTo(ActuatorRelativeTo),
    ActuatorType(ActuatorType),
    AdAvailabilityResult(AdAvailabilityResult),
    AdEventType(AdEventType),
    AdFormat(AdFormat),
    AdShape(AdShape),
    AdTeleportMethod(AdTeleportMethod),
    AdUIEventType(AdUIEventType),
    AdUIType(AdUIType),
    AdUnitStatus(AdUnitStatus),
    AdornCullingMode(AdornCullingMode),
    AlignType(AlignType),
    AlphaMode(AlphaMode),
    AnalyticsCustomFieldKeys(AnalyticsCustomFieldKeys),
    AnalyticsEconomyAction(AnalyticsEconomyAction),
    AnalyticsEconomyFlowType(AnalyticsEconomyFlowType),
    AnalyticsEconomyTransactionType(AnalyticsEconomyTransactionType),
    AnalyticsLogLevel(AnalyticsLogLevel),
    AnalyticsProgressionStatus(AnalyticsProgressionStatus),
    AnalyticsProgressionType(AnalyticsProgressionType),
    AnimationClipFromVideoStatus(AnimationClipFromVideoStatus),
    AnimationPriority(AnimationPriority),
    AnimatorRetargetingMode(AnimatorRetargetingMode),
    AnnotationEditingMode(AnnotationEditingMode),
    AnnotationRequestStatus(AnnotationRequestStatus),
    AnnotationRequestType(AnnotationRequestType),
    AppLifecycleManagerState(AppLifecycleManagerState),
    AppShellActionType(AppShellActionType),
    AppShellFeature(AppShellFeature),
    AppUpdateStatus(AppUpdateStatus),
    ApplyStrokeMode(ApplyStrokeMode),
    AspectType(AspectType),
    AssetCreatorType(AssetCreatorType),
    AssetFetchStatus(AssetFetchStatus),
    AssetType(AssetType),
    AssetTypeVerification(AssetTypeVerification),
    AudioApiRollout(AudioApiRollout),
    AudioChannelLayout(AudioChannelLayout),
    AudioFilterType(AudioFilterType),
    AudioSimulationFidelity(AudioSimulationFidelity),
    AudioSubType(AudioSubType),
    AudioWindowSize(AudioWindowSize),
    AutoIndentRule(AutoIndentRule),
    AutomaticSize(AutomaticSize),
    AvatarAssetType(AvatarAssetType),
    AvatarChatServiceFeature(AvatarChatServiceFeature),
    AvatarContextMenuOption(AvatarContextMenuOption),
    AvatarGenerationError(AvatarGenerationError),
    AvatarItemType(AvatarItemType),
    AvatarPromptResult(AvatarPromptResult),
    AvatarThumbnailCustomizationType(AvatarThumbnailCustomizationType),
    AvatarUnificationMode(AvatarUnificationMode),
    Axis(Axis),
    BenefitType(BenefitType),
    BinType(BinType),
    BodyPart(BodyPart),
    BodyPartR15(BodyPartR15),
    BorderMode(BorderMode),
    BreakReason(BreakReason),
    BreakpointRemoveReason(BreakpointRemoveReason),
    BulkMoveMode(BulkMoveMode),
    BundleType(BundleType),
    Button(Button),
    ButtonStyle(ButtonStyle),
    CageType(CageType),
    CameraMode(CameraMode),
    CameraPanMode(CameraPanMode),
    CameraSpeedAdjustBinding(CameraSpeedAdjustBinding),
    CameraType(CameraType),
    CatalogCategoryFilter(CatalogCategoryFilter),
    CatalogSortAggregation(CatalogSortAggregation),
    CatalogSortType(CatalogSortType),
    CellBlock(CellBlock),
    CellMaterial(CellMaterial),
    CellOrientation(CellOrientation),
    CenterDialogType(CenterDialogType),
    CharacterControlMode(CharacterControlMode),
    ChatCallbackType(ChatCallbackType),
    ChatColor(ChatColor),
    ChatMode(ChatMode),
    ChatPrivacyMode(ChatPrivacyMode),
    ChatRestrictionStatus(ChatRestrictionStatus),
    ChatStyle(ChatStyle),
    ChatVersion(ChatVersion),
    ClientAnimatorThrottlingMode(ClientAnimatorThrottlingMode),
    CloseReason(CloseReason),
    CollaboratorStatus(CollaboratorStatus),
    CollisionFidelity(CollisionFidelity),
    CommandPermission(CommandPermission),
    CompileTarget(CompileTarget),
    CompletionAcceptanceBehavior(CompletionAcceptanceBehavior),
    CompletionItemKind(CompletionItemKind),
    CompletionItemTag(CompletionItemTag),
    CompletionTriggerKind(CompletionTriggerKind),
    ComputerCameraMovementMode(ComputerCameraMovementMode),
    ComputerMovementMode(ComputerMovementMode),
    ConfigSnapshotErrorState(ConfigSnapshotErrorState),
    ConnectionError(ConnectionError),
    ConnectionState(ConnectionState),
    ContentSourceType(ContentSourceType),
    ContextActionPriority(ContextActionPriority),
    ContextActionResult(ContextActionResult),
    ControlMode(ControlMode),
    CoreGuiType(CoreGuiType),
    CreateAssetResult(CreateAssetResult),
    CreateOutfitFailure(CreateOutfitFailure),
    CreatorType(CreatorType),
    CreatorTypeFilter(CreatorTypeFilter),
    CurrencyType(CurrencyType),
    CustomCameraMode(CustomCameraMode),
    DataStoreRequestType(DataStoreRequestType),
    DebuggerEndReason(DebuggerEndReason),
    DebuggerExceptionBreakMode(DebuggerExceptionBreakMode),
    DebuggerFrameType(DebuggerFrameType),
    DebuggerPauseReason(DebuggerPauseReason),
    DebuggerStatus(DebuggerStatus),
    DevCameraOcclusionMode(DevCameraOcclusionMode),
    DevComputerCameraMovementMode(DevComputerCameraMovementMode),
    DevComputerMovementMode(DevComputerMovementMode),
    DevTouchCameraMovementMode(DevTouchCameraMovementMode),
    DevTouchMovementMode(DevTouchMovementMode),
    DeveloperMemoryTag(DeveloperMemoryTag),
    DeviceFeatureType(DeviceFeatureType),
    DeviceForm(DeviceForm),
    DeviceLevel(DeviceLevel),
    DeviceType(DeviceType),
    DialogBehaviorType(DialogBehaviorType),
    DialogPurpose(DialogPurpose),
    DialogTone(DialogTone),
    DominantAxis(DominantAxis),
    DraftStatusCode(DraftStatusCode),
    DragDetectorDragStyle(DragDetectorDragStyle),
    DragDetectorPermissionPolicy(DragDetectorPermissionPolicy),
    DragDetectorResponseStyle(DragDetectorResponseStyle),
    DraggerCoordinateSpace(DraggerCoordinateSpace),
    DraggerMovementMode(DraggerMovementMode),
    DraggingScrollBar(DraggingScrollBar),
    EasingDirection(EasingDirection),
    EasingStyle(EasingStyle),
    EditableStatus(EditableStatus),
    ElasticBehavior(ElasticBehavior),
    EnviromentalPhysicsThrottle(EnviromentalPhysicsThrottle),
    ExperienceAuthScope(ExperienceAuthScope),
    ExplosionType(ExplosionType),
    FACSDataLod(FACSDataLod),
    FacialAgeEstimationResultType(FacialAgeEstimationResultType),
    FacialAnimationStreamingState(FacialAnimationStreamingState),
    FacsActionUnit(FacsActionUnit),
    FeedRankingScoreType(FeedRankingScoreType),
    FieldOfViewMode(FieldOfViewMode),
    FillDirection(FillDirection),
    FilterErrorType(FilterErrorType),
    FilterResult(FilterResult),
    FinishRecordingOperation(FinishRecordingOperation),
    FluidFidelity(FluidFidelity),
    FluidForces(FluidForces),
    Font(Font),
    FontSize(FontSize),
    FontStyle(FontStyle),
    FontWeight(FontWeight),
    ForceLimitMode(ForceLimitMode),
    FormFactor(FormFactor),
    FrameStyle(FrameStyle),
    FramerateManagerMode(FramerateManagerMode),
    FriendRequestEvent(FriendRequestEvent),
    FriendStatus(FriendStatus),
    FunctionalTestResult(FunctionalTestResult),
    GameAvatarType(GameAvatarType),
    GamepadType(GamepadType),
    GearGenreSetting(GearGenreSetting),
    GearType(GearType),
    Genre(Genre),
    GraphicsMode(GraphicsMode),
    GraphicsOptimizationMode(GraphicsOptimizationMode),
    GuiState(GuiState),
    GuiType(GuiType),
    HandlesStyle(HandlesStyle),
    HapticEffectType(HapticEffectType),
    HighlightDepthMode(HighlightDepthMode),
    HorizontalAlignment(HorizontalAlignment),
    HoverAnimateSpeed(HoverAnimateSpeed),
    HttpCachePolicy(HttpCachePolicy),
    HttpCompression(HttpCompression),
    HttpContentType(HttpContentType),
    HttpError(HttpError),
    HttpRequestType(HttpRequestType),
    HumanoidCollisionType(HumanoidCollisionType),
    HumanoidDisplayDistanceType(HumanoidDisplayDistanceType),
    HumanoidHealthDisplayType(HumanoidHealthDisplayType),
    HumanoidRigType(HumanoidRigType),
    HumanoidStateType(HumanoidStateType),
    IKCollisionsMode(IKCollisionsMode),
    IKControlConstraintSupport(IKControlConstraintSupport),
    IKControlType(IKControlType),
    IXPLoadingStatus(IXPLoadingStatus),
    ImageAlphaType(ImageAlphaType),
    ImageCombineType(ImageCombineType),
    InOut(InOut),
    InfoType(InfoType),
    InitialDockState(InitialDockState),
    InputActionType(InputActionType),
    InputType(InputType),
    IntermediateMeshGenerationResult(IntermediateMeshGenerationResult),
    InterpolationThrottlingMode(InterpolationThrottlingMode),
    InviteState(InviteState),
    ItemLineAlignment(ItemLineAlignment),
    JoinSource(JoinSource),
    JointCreationMode(JointCreationMode),
    KeyCode(KeyCode),
    KeyInterpolationMode(KeyInterpolationMode),
    KeywordFilterType(KeywordFilterType),
    Language(Language),
    LeftRight(LeftRight),
    LexemeType(LexemeType),
    LightingStyle(LightingStyle),
    Limb(Limb),
    LineJoinMode(LineJoinMode),
    ListDisplayMode(ListDisplayMode),
    ListenerLocation(ListenerLocation),
    ListenerType(ListenerType),
    LiveEditingAtomicUpdateResponse(LiveEditingAtomicUpdateResponse),
    LiveEditingBroadcastMessageType(LiveEditingBroadcastMessageType),
    LoadCharacterLayeredClothing(LoadCharacterLayeredClothing),
    LoadDynamicHeads(LoadDynamicHeads),
    LocationType(LocationType),
    MarketplaceBulkPurchasePromptStatus(MarketplaceBulkPurchasePromptStatus),
    MarketplaceItemPurchaseStatus(MarketplaceItemPurchaseStatus),
    MarketplaceProductType(MarketplaceProductType),
    MarkupKind(MarkupKind),
    MatchmakingType(MatchmakingType),
    Material(Material),
    MaterialPattern(MaterialPattern),
    MembershipType(MembershipType),
    MeshPartDetailLevel(MeshPartDetailLevel),
    MeshPartHeadsAndAccessories(MeshPartHeadsAndAccessories),
    MeshScaleUnit(MeshScaleUnit),
    MeshType(MeshType),
    MessageType(MessageType),
    ModelLevelOfDetail(ModelLevelOfDetail),
    ModelStreamingBehavior(ModelStreamingBehavior),
    ModelStreamingMode(ModelStreamingMode),
    ModerationStatus(ModerationStatus),
    ModifierKey(ModifierKey),
    MouseBehavior(MouseBehavior),
    MoveState(MoveState),
    MoverConstraintRootBehaviorMode(MoverConstraintRootBehaviorMode),
    MuteState(MuteState),
    NameOcclusion(NameOcclusion),
    NetworkOwnership(NetworkOwnership),
    NetworkStatus(NetworkStatus),
    NoiseType(NoiseType),
    NormalId(NormalId),
    NotificationButtonType(NotificationButtonType),
    OperationType(OperationType),
    OrientationAlignmentMode(OrientationAlignmentMode),
    OutfitSource(OutfitSource),
    OutfitType(OutfitType),
    OutputLayoutMode(OutputLayoutMode),
    OverrideMouseIconBehavior(OverrideMouseIconBehavior),
    PackagePermission(PackagePermission),
    PartType(PartType),
    ParticleEmitterShape(ParticleEmitterShape),
    ParticleEmitterShapeInOut(ParticleEmitterShapeInOut),
    ParticleEmitterShapeStyle(ParticleEmitterShapeStyle),
    ParticleFlipbookLayout(ParticleFlipbookLayout),
    ParticleFlipbookMode(ParticleFlipbookMode),
    ParticleFlipbookTextureCompatible(ParticleFlipbookTextureCompatible),
    ParticleOrientation(ParticleOrientation),
    PathStatus(PathStatus),
    PathWaypointAction(PathWaypointAction),
    PathfindingUseImprovedSearch(PathfindingUseImprovedSearch),
    PermissionLevelShown(PermissionLevelShown),
    PhysicsSimulationRate(PhysicsSimulationRate),
    PhysicsSteppingMethod(PhysicsSteppingMethod),
    Platform(Platform),
    PlaybackState(PlaybackState),
    PlayerActions(PlayerActions),
    PlayerCharacterDestroyBehavior(PlayerCharacterDestroyBehavior),
    PlayerChatType(PlayerChatType),
    PlayerDataErrorState(PlayerDataErrorState),
    PlayerDataLoadFailureBehavior(PlayerDataLoadFailureBehavior),
    PoseEasingDirection(PoseEasingDirection),
    PoseEasingStyle(PoseEasingStyle),
    PositionAlignmentMode(PositionAlignmentMode),
    PreferredTextSize(PreferredTextSize),
    PrimalPhysicsSolver(PrimalPhysicsSolver),
    PrimitiveType(PrimitiveType),
    PrivilegeType(PrivilegeType),
    ProductLocationRestriction(ProductLocationRestriction),
    ProductPurchaseChannel(ProductPurchaseChannel),
    ProductPurchaseDecision(ProductPurchaseDecision),
    PromptCreateAssetResult(PromptCreateAssetResult),
    PromptCreateAvatarResult(PromptCreateAvatarResult),
    PromptPublishAssetResult(PromptPublishAssetResult),
    PropertyStatus(PropertyStatus),
    ProximityPromptExclusivity(ProximityPromptExclusivity),
    ProximityPromptInputType(ProximityPromptInputType),
    ProximityPromptStyle(ProximityPromptStyle),
    QualityLevel(QualityLevel),
    R15CollisionType(R15CollisionType),
    RaycastFilterType(RaycastFilterType),
    RejectCharacterDeletions(RejectCharacterDeletions),
    RenderFidelity(RenderFidelity),
    RenderPriority(RenderPriority),
    RenderingCacheOptimizationMode(RenderingCacheOptimizationMode),
    RenderingTestComparisonMethod(RenderingTestComparisonMethod),
    ReplicateInstanceDestroySetting(ReplicateInstanceDestroySetting),
    ResamplerMode(ResamplerMode),
    ReservedHighlightId(ReservedHighlightId),
    RestPose(RestPose),
    ReturnKeyType(ReturnKeyType),
    ReverbType(ReverbType),
    RibbonTool(RibbonTool),
    RigScale(RigScale),
    RigType(RigType),
    RollOffMode(RollOffMode),
    RolloutState(RolloutState),
    RotationOrder(RotationOrder),
    RotationType(RotationType),
    RtlTextSupport(RtlTextSupport),
    RunContext(RunContext),
    RunState(RunState),
    RuntimeUndoBehavior(RuntimeUndoBehavior),
    SafeAreaCompatibility(SafeAreaCompatibility),
    SalesTypeFilter(SalesTypeFilter),
    SandboxedInstanceMode(SandboxedInstanceMode),
    SaveAvatarThumbnailCustomizationFailure(SaveAvatarThumbnailCustomizationFailure),
    SaveFilter(SaveFilter),
    SavedQualitySetting(SavedQualitySetting),
    ScaleType(ScaleType),
    ScopeCheckResult(ScopeCheckResult),
    ScreenInsets(ScreenInsets),
    ScreenOrientation(ScreenOrientation),
    ScrollBarInset(ScrollBarInset),
    ScrollingDirection(ScrollingDirection),
    SecurityCapability(SecurityCapability),
    SelectionBehavior(SelectionBehavior),
    SelectionRenderMode(SelectionRenderMode),
    SelfViewPosition(SelfViewPosition),
    SensorMode(SensorMode),
    SensorUpdateType(SensorUpdateType),
    ServerLiveEditingMode(ServerLiveEditingMode),
    ServiceVisibility(ServiceVisibility),
    Severity(Severity),
    ShowAdResult(ShowAdResult),
    SignalBehavior(SignalBehavior),
    SizeConstraint(SizeConstraint),
    SolverConvergenceMetricType(SolverConvergenceMetricType),
    SolverConvergenceVisualizationMode(SolverConvergenceVisualizationMode),
    SortDirection(SortDirection),
    SortOrder(SortOrder),
    SpecialKey(SpecialKey),
    StartCorner(StartCorner),
    StateObjectFieldType(StateObjectFieldType),
    Status(Status),
    StreamOutBehavior(StreamOutBehavior),
    StreamingIntegrityMode(StreamingIntegrityMode),
    StreamingPauseMode(StreamingPauseMode),
    StudioCloseMode(StudioCloseMode),
    StudioDataModelType(StudioDataModelType),
    StudioPlaceUpdateFailureReason(StudioPlaceUpdateFailureReason),
    StudioScriptEditorColorCategories(StudioScriptEditorColorCategories),
    StudioScriptEditorColorPresets(StudioScriptEditorColorPresets),
    StudioStyleGuideColor(StudioStyleGuideColor),
    StudioStyleGuideModifier(StudioStyleGuideModifier),
    Style(Style),
    SubscriptionExpirationReason(SubscriptionExpirationReason),
    SubscriptionPaymentStatus(SubscriptionPaymentStatus),
    SubscriptionPeriod(SubscriptionPeriod),
    SubscriptionState(SubscriptionState),
    SurfaceConstraint(SurfaceConstraint),
    SurfaceGuiShape(SurfaceGuiShape),
    SurfaceGuiSizingMode(SurfaceGuiSizingMode),
    SurfaceType(SurfaceType),
    SwipeDirection(SwipeDirection),
    SystemThemeValue(SystemThemeValue),
    TableMajorAxis(TableMajorAxis),
    TeamCreateErrorState(TeamCreateErrorState),
    Technology(Technology),
    TeleportMethod(TeleportMethod),
    TeleportResult(TeleportResult),
    TeleportState(TeleportState),
    TeleportType(TeleportType),
    TerrainAcquisitionMethod(TerrainAcquisitionMethod),
    TerrainFace(TerrainFace),
    TextChatMessageStatus(TextChatMessageStatus),
    TextDirection(TextDirection),
    TextFilterContext(TextFilterContext),
    TextInputType(TextInputType),
    TextTruncate(TextTruncate),
    TextXAlignment(TextXAlignment),
    TextYAlignment(TextYAlignment),
    TextureMode(TextureMode),
    TextureQueryType(TextureQueryType),
    ThreadPoolConfig(ThreadPoolConfig),
    ThrottlingPriority(ThrottlingPriority),
    ThumbnailSize(ThumbnailSize),
    ThumbnailType(ThumbnailType),
    TickCountSampleMethod(TickCountSampleMethod),
    TonemapperPreset(TonemapperPreset),
    TopBottom(TopBottom),
    TouchCameraMovementMode(TouchCameraMovementMode),
    TouchMovementMode(TouchMovementMode),
    TrackerError(TrackerError),
    TrackerExtrapolationFlagMode(TrackerExtrapolationFlagMode),
    TrackerFaceTrackingStatus(TrackerFaceTrackingStatus),
    TrackerLodFlagMode(TrackerLodFlagMode),
    TrackerLodValueMode(TrackerLodValueMode),
    TrackerMode(TrackerMode),
    TrackerPromptEvent(TrackerPromptEvent),
    TrackerType(TrackerType),
    TriStateBoolean(TriStateBoolean),
    TweenStatus(TweenStatus),
    UICaptureMode(UICaptureMode),
    UIDragDetectorBoundingBehavior(UIDragDetectorBoundingBehavior),
    UIDragDetectorDragRelativity(UIDragDetectorDragRelativity),
    UIDragDetectorDragSpace(UIDragDetectorDragSpace),
    UIDragDetectorDragStyle(UIDragDetectorDragStyle),
    UIDragDetectorResponseStyle(UIDragDetectorResponseStyle),
    UIDragSpeedAxisMapping(UIDragSpeedAxisMapping),
    UIFlexAlignment(UIFlexAlignment),
    UIFlexMode(UIFlexMode),
    UITheme(UITheme),
    UiMessageType(UiMessageType),
    UsageContext(UsageContext),
    UserCFrame(UserCFrame),
    UserInputState(UserInputState),
    UserInputType(UserInputType),
    VRComfortSetting(VRComfortSetting),
    VRControllerModelMode(VRControllerModelMode),
    VRDeviceType(VRDeviceType),
    VRLaserPointerMode(VRLaserPointerMode),
    VRSafetyBubbleMode(VRSafetyBubbleMode),
    VRScaling(VRScaling),
    VRSessionState(VRSessionState),
    VRTouchpad(VRTouchpad),
    VRTouchpadMode(VRTouchpadMode),
    VelocityConstraintMode(VelocityConstraintMode),
    VerticalAlignment(VerticalAlignment),
    VerticalScrollBarPosition(VerticalScrollBarPosition),
    VibrationMotor(VibrationMotor),
    VideoDeviceCaptureQuality(VideoDeviceCaptureQuality),
    VideoError(VideoError),
    ViewMode(ViewMode),
    VirtualCursorMode(VirtualCursorMode),
    VirtualInputMode(VirtualInputMode),
    VoiceChatDistanceAttenuationType(VoiceChatDistanceAttenuationType),
    VoiceChatState(VoiceChatState),
    VoiceControlPath(VoiceControlPath),
    VolumetricAudio(VolumetricAudio),
    WaterDirection(WaterDirection),
    WaterForce(WaterForce),
    WebSocketState(WebSocketState),
    WeldConstraintPreserve(WeldConstraintPreserve),
    WhisperChatPrivacyMode(WhisperChatPrivacyMode),
    WrapLayerAutoSkin(WrapLayerAutoSkin),
    WrapLayerDebugMode(WrapLayerDebugMode),
    WrapTargetDebugMode(WrapTargetDebugMode),
    ZIndexBehavior(ZIndexBehavior),
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AccessModifierType {
    Allow = 0,
    Deny = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AccessoryType {
    Unknown = 0,
    Hat = 1,
    Hair = 2,
    Face = 3,
    Neck = 4,
    Shoulder = 5,
    Front = 6,
    Back = 7,
    Waist = 8,
    TShirt = 9,
    Shirt = 10,
    Pants = 11,
    Jacket = 12,
    Sweater = 13,
    Shorts = 14,
    LeftShoe = 15,
    RightShoe = 16,
    DressSkirt = 17,
    Eyebrow = 18,
    Eyelash = 19,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ActionOnAutoResumeSync {
    DontResume = 0,
    KeepStudio = 1,
    KeepLocal = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ActionOnStopSync {
    AlwaysAsk = 0,
    KeepLocalFiles = 1,
    DeleteLocalFiles = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ActionType {
    Nothing = 0,
    Pause = 1,
    Lose = 2,
    Draw = 3,
    Win = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ActuatorRelativeTo {
    Attachment0 = 0,
    Attachment1 = 1,
    World = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ActuatorType {
    None = 0,
    Motor = 1,
    Servo = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AdAvailabilityResult {
    IsAvailable = 1,
    DeviceIneligible = 2,
    ExperienceIneligible = 3,
    InternalError = 4,
    NoFill = 5,
    PlayerIneligible = 6,
    PublisherIneligible = 7,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AdEventType {
    VideoLoaded = 0,
    VideoRemoved = 1,
    UserCompletedVideo = 2,
    RewardedAdLoaded = 3,
    RewardedAdGrant = 4,
    RewardedAdUnloaded = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AdFormat {
    RewardedVideo = 0,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AdShape {
    HorizontalRectangle = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AdTeleportMethod {
    Undefined = 0,
    PortalForward = 1,
    InGameMenuBackButton = 2,
    UIBackButton = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AdUIEventType {
    AdLabelClicked = 0,
    VolumeButtonClicked = 1,
    FullscreenButtonClicked = 2,
    PlayButtonClicked = 3,
    PauseButtonClicked = 4,
    CloseButtonClicked = 5,
    WhyThisAdClicked = 6,
    PlayEventTriggered = 7,
    PauseEventTriggered = 8,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AdUIType {
    None = 0,
    Image = 1,
    Video = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AdUnitStatus {
    Inactive = 0,
    Active = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AdornCullingMode {
    Automatic = 0,
    Never = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AlignType {
    Parallel = 0,
    Perpendicular = 1,
    PrimaryAxisParallel = 2,
    PrimaryAxisPerpendicular = 3,
    PrimaryAxisLookAt = 4,
    AllAxes = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AlphaMode {
    Overlay = 0,
    Transparency = 1,
    TintMask = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AnalyticsCustomFieldKeys {
    CustomField01 = 0,
    CustomField02 = 1,
    CustomField03 = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AnalyticsEconomyAction {
    Default = 0,
    Acquire = 1,
    Spend = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AnalyticsEconomyFlowType {
    Sink = 0,
    Source = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AnalyticsEconomyTransactionType {
    IAP = 0,
    Shop = 1,
    Gameplay = 2,
    ContextualPurchase = 3,
    TimedReward = 4,
    Onboarding = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AnalyticsLogLevel {
    Trace = 0,
    Debug = 1,
    Information = 2,
    Warning = 3,
    Error = 4,
    Fatal = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AnalyticsProgressionStatus {
    Default = 0,
    Begin = 1,
    Complete = 2,
    Abandon = 3,
    Fail = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AnalyticsProgressionType {
    Custom = 0,
    Start = 1,
    Fail = 2,
    Complete = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AnimationClipFromVideoStatus {
    Initializing = 0,
    Pending = 1,
    Processing = 2,
    ErrorGeneric = 4,
    Success = 6,
    ErrorVideoTooLong = 7,
    ErrorNoPersonDetected = 8,
    ErrorVideoUnstable = 9,
    Timeout = 10,
    Cancelled = 11,
    ErrorMultiplePeople = 12,
    ErrorUploadingVideo = 2001,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AnimationPriority {
    Idle = 0,
    Movement = 1,
    Action = 2,
    Action2 = 3,
    Action3 = 4,
    Action4 = 5,
    Core = 1000,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AnimatorRetargetingMode {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AnnotationEditingMode {
    None = 0,
    PlacingNew = 1,
    WritingNew = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AnnotationRequestStatus {
    Success = 0,
    Loading = 1,
    ErrorInternalFailure = 2,
    ErrorNotFound = 3,
    ErrorModerated = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AnnotationRequestType {
    Unknown = 0,
    Create = 1,
    Resolve = 2,
    Delete = 3,
    Edit = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AppLifecycleManagerState {
    Detached = 0,
    Active = 1,
    Inactive = 2,
    Hidden = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AppShellActionType {
    None = 0,
    OpenApp = 1,
    TapChatTab = 2,
    TapConversationEntry = 3,
    TapAvatarTab = 4,
    ReadConversation = 5,
    TapGamePageTab = 6,
    TapHomePageTab = 7,
    GamePageLoaded = 8,
    HomePageLoaded = 9,
    AvatarEditorPageLoaded = 10,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AppShellFeature {
    None = 0,
    Chat = 1,
    AvatarEditor = 2,
    GamePage = 3,
    HomePage = 4,
    More = 5,
    Landing = 6,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AppUpdateStatus {
    Unknown = 0,
    NotSupported = 1,
    Failed = 2,
    NotAvailable = 3,
    Available = 4,
    AvailableBoundChannel = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ApplyStrokeMode {
    Contextual = 0,
    Border = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AspectType {
    FitWithinMaxSize = 0,
    ScaleWithParentSize = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AssetCreatorType {
    User = 0,
    Group = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AssetFetchStatus {
    Success = 0,
    Failure = 1,
    None = 2,
    Loading = 3,
    TimedOut = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AssetType {
    Image = 1,
    TShirt = 2,
    Audio = 3,
    Mesh = 4,
    Lua = 5,
    Hat = 8,
    Place = 9,
    Model = 10,
    Shirt = 11,
    Pants = 12,
    Decal = 13,
    Head = 17,
    Face = 18,
    Gear = 19,
    Badge = 21,
    Animation = 24,
    Torso = 27,
    RightArm = 28,
    LeftArm = 29,
    LeftLeg = 30,
    RightLeg = 31,
    Package = 32,
    GamePass = 34,
    Plugin = 38,
    MeshPart = 40,
    HairAccessory = 41,
    FaceAccessory = 42,
    NeckAccessory = 43,
    ShoulderAccessory = 44,
    FrontAccessory = 45,
    BackAccessory = 46,
    WaistAccessory = 47,
    ClimbAnimation = 48,
    DeathAnimation = 49,
    FallAnimation = 50,
    IdleAnimation = 51,
    JumpAnimation = 52,
    RunAnimation = 53,
    SwimAnimation = 54,
    WalkAnimation = 55,
    PoseAnimation = 56,
    EarAccessory = 57,
    EyeAccessory = 58,
    EmoteAnimation = 61,
    Video = 62,
    TShirtAccessory = 64,
    ShirtAccessory = 65,
    PantsAccessory = 66,
    JacketAccessory = 67,
    SweaterAccessory = 68,
    ShortsAccessory = 69,
    LeftShoeAccessory = 70,
    RightShoeAccessory = 71,
    DressSkirtAccessory = 72,
    FontFamily = 73,
    EyebrowAccessory = 76,
    EyelashAccessory = 77,
    MoodAnimation = 78,
    DynamicHead = 79,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AssetTypeVerification {
    Default = 1,
    ClientOnly = 2,
    Always = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AudioApiRollout {
    Disabled = 0,
    Automatic = 1,
    Enabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AudioChannelLayout {
    Mono = 0,
    Stereo = 1,
    Quad = 2,
    Surround_5 = 3,
    Surround_5_1 = 4,
    Surround_7_1 = 5,
    Surround_7_1_4 = 6,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AudioFilterType {
    Peak = 0,
    LowShelf = 1,
    HighShelf = 2,
    Lowpass12dB = 3,
    Lowpass24dB = 4,
    Lowpass48dB = 5,
    Highpass12dB = 6,
    Highpass24dB = 7,
    Highpass48dB = 8,
    Bandpass = 9,
    Notch = 10,
    Lowpass6dB = 11,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AudioSimulationFidelity {
    None = 0,
    Automatic = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AudioSubType {
    Music = 1,
    SoundEffect = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AudioWindowSize {
    Small = 0,
    Medium = 1,
    Large = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AutoIndentRule {
    Off = 0,
    Absolute = 1,
    Relative = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AutomaticSize {
    None = 0,
    X = 1,
    Y = 2,
    XY = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AvatarAssetType {
    TShirt = 2,
    Hat = 8,
    Shirt = 11,
    Pants = 12,
    Head = 17,
    Face = 18,
    Gear = 19,
    Torso = 27,
    RightArm = 28,
    LeftArm = 29,
    LeftLeg = 30,
    RightLeg = 31,
    HairAccessory = 41,
    FaceAccessory = 42,
    NeckAccessory = 43,
    ShoulderAccessory = 44,
    FrontAccessory = 45,
    BackAccessory = 46,
    WaistAccessory = 47,
    ClimbAnimation = 48,
    FallAnimation = 50,
    IdleAnimation = 51,
    JumpAnimation = 52,
    RunAnimation = 53,
    SwimAnimation = 54,
    WalkAnimation = 55,
    EmoteAnimation = 61,
    TShirtAccessory = 64,
    ShirtAccessory = 65,
    PantsAccessory = 66,
    JacketAccessory = 67,
    SweaterAccessory = 68,
    ShortsAccessory = 69,
    LeftShoeAccessory = 70,
    RightShoeAccessory = 71,
    DressSkirtAccessory = 72,
    EyebrowAccessory = 76,
    EyelashAccessory = 77,
    MoodAnimation = 78,
    DynamicHead = 79,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AvatarChatServiceFeature {
    None = 0,
    UniverseAudio = 1,
    UniverseVideo = 2,
    PlaceAudio = 4,
    PlaceVideo = 8,
    UserAudioEligible = 16,
    UserAudio = 32,
    UserVideoEligible = 64,
    UserVideo = 128,
    UserBanned = 256,
    UserVerifiedForVoice = 512,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AvatarContextMenuOption {
    Friend = 0,
    Chat = 1,
    Emote = 2,
    InspectMenu = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AvatarGenerationError {
    None = 0,
    Unknown = 1,
    DownloadFailed = 2,
    Canceled = 3,
    Offensive = 4,
    Timeout = 5,
    JobNotFound = 6,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AvatarItemType {
    Asset = 1,
    Bundle = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AvatarPromptResult {
    Success = 1,
    PermissionDenied = 2,
    Failed = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AvatarThumbnailCustomizationType {
    Closeup = 1,
    FullBody = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum AvatarUnificationMode {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum BenefitType {
    DeveloperProduct = 0,
    AvatarAsset = 1,
    AvatarBundle = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum BinType {
    Script = 0,
    GameTool = 1,
    Grab = 2,
    Clone = 3,
    Hammer = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum BodyPart {
    Head = 0,
    Torso = 1,
    LeftArm = 2,
    RightArm = 3,
    LeftLeg = 4,
    RightLeg = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum BodyPartR15 {
    Head = 0,
    UpperTorso = 1,
    LowerTorso = 2,
    LeftFoot = 3,
    LeftLowerLeg = 4,
    LeftUpperLeg = 5,
    RightFoot = 6,
    RightLowerLeg = 7,
    RightUpperLeg = 8,
    LeftHand = 9,
    LeftLowerArm = 10,
    LeftUpperArm = 11,
    RightHand = 12,
    RightLowerArm = 13,
    RightUpperArm = 14,
    RootPart = 15,
    Unknown = 17,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum BorderMode {
    Outline = 0,
    Middle = 1,
    Inset = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum BreakReason {
    Other = 0,
    Error = 1,
    SpecialBreakpoint = 2,
    UserBreakpoint = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum BreakpointRemoveReason {
    Requested = 0,
    ScriptChanged = 1,
    ScriptRemoved = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum BulkMoveMode {
    FireAllEvents = 0,
    FireCFrameChanged = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum BundleType {
    BodyParts = 1,
    Animations = 2,
    Shoes = 3,
    DynamicHead = 4,
    DynamicHeadAvatar = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum Button {
    Dismount = 8,
    Jump = 32,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ButtonStyle {
    Custom = 0,
    RobloxButtonDefault = 1,
    RobloxButton = 2,
    RobloxRoundButton = 3,
    RobloxRoundDefaultButton = 4,
    RobloxRoundDropdownButton = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CageType {
    Inner = 0,
    Outer = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CameraMode {
    Classic = 0,
    LockFirstPerson = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CameraPanMode {
    Classic = 0,
    EdgeBump = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CameraSpeedAdjustBinding {
    None = 0,
    RmbScroll = 1,
    AltScroll = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CameraType {
    Fixed = 0,
    Attach = 1,
    Watch = 2,
    Track = 3,
    Follow = 4,
    Custom = 5,
    Scriptable = 6,
    Orbital = 7,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CatalogCategoryFilter {
    None = 1,
    Featured = 2,
    Collectibles = 3,
    CommunityCreations = 4,
    Premium = 5,
    Recommended = 6,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CatalogSortAggregation {
    Past12Hours = 1,
    PastDay = 2,
    Past3Days = 3,
    PastWeek = 4,
    PastMonth = 5,
    AllTime = 6,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CatalogSortType {
    Relevance = 1,
    PriceHighToLow = 2,
    PriceLowToHigh = 3,
    MostFavorited = 5,
    RecentlyCreated = 6,
    Bestselling = 7,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CellBlock {
    Solid = 0,
    VerticalWedge = 1,
    CornerWedge = 2,
    InverseCornerWedge = 3,
    HorizontalWedge = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CellMaterial {
    Empty = 0,
    Grass = 1,
    Sand = 2,
    Brick = 3,
    Granite = 4,
    Asphalt = 5,
    Iron = 6,
    Aluminum = 7,
    Gold = 8,
    WoodPlank = 9,
    WoodLog = 10,
    Gravel = 11,
    CinderBlock = 12,
    MossyStone = 13,
    Cement = 14,
    RedPlastic = 15,
    BluePlastic = 16,
    Water = 17,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CellOrientation {
    NegZ = 0,
    X = 1,
    Z = 2,
    NegX = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CenterDialogType {
    UnsolicitedDialog = 1,
    PlayerInitiatedDialog = 2,
    ModalDialog = 3,
    QuitDialog = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CharacterControlMode {
    Default = 0,
    Legacy = 1,
    NoCharacterController = 2,
    LuaCharacterController = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ChatCallbackType {
    OnCreatingChatWindow = 1,
    OnClientSendingMessage = 2,
    OnClientFormattingMessage = 3,
    OnServerReceivingMessage = 17,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ChatColor {
    Blue = 0,
    Green = 1,
    Red = 2,
    White = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ChatMode {
    Menu = 0,
    TextAndMenu = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ChatPrivacyMode {
    AllUsers = 0,
    NoOne = 1,
    Friends = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ChatRestrictionStatus {
    Unknown = 0,
    NotRestricted = 1,
    Restricted = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ChatStyle {
    Classic = 0,
    Bubble = 1,
    ClassicAndBubble = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ChatVersion {
    LegacyChatService = 0,
    TextChatService = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ClientAnimatorThrottlingMode {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CloseReason {
    Unknown = 0,
    RobloxMaintenance = 1,
    DeveloperShutdown = 2,
    DeveloperUpdate = 3,
    ServerEmpty = 4,
    OutOfMemory = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CollaboratorStatus {
    None = 0,
    Editing3D = 1,
    Scripting = 2,
    PrivateScripting = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CollisionFidelity {
    Default = 0,
    Hull = 1,
    Box = 2,
    PreciseConvexDecomposition = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CommandPermission {
    Plugin = 0,
    LocalUser = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CompileTarget {
    Client = 0,
    CoreScript = 1,
    Studio = 2,
    CoreScriptRaw = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CompletionAcceptanceBehavior {
    Insert = 0,
    Replace = 1,
    ReplaceOnEnterInsertOnTab = 2,
    InsertOnEnterReplaceOnTab = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Unit = 11,
    Value = 12,
    Enum = 13,
    Keyword = 14,
    Snippet = 15,
    Color = 16,
    File = 17,
    Reference = 18,
    Folder = 19,
    EnumMember = 20,
    Constant = 21,
    Struct = 22,
    Event = 23,
    Operator = 24,
    TypeParameter = 25,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CompletionItemTag {
    Deprecated = 1,
    IncorrectIndexType = 2,
    PluginPermissions = 3,
    CommandLinePermissions = 4,
    RobloxPermissions = 5,
    AddParens = 6,
    PutCursorInParens = 7,
    TypeCorrect = 8,
    ClientServerBoundaryViolation = 9,
    Invalidated = 10,
    PutCursorBeforeEnd = 11,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CompletionTriggerKind {
    Invoked = 1,
    TriggerCharacter = 2,
    TriggerForIncompleteCompletions = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ComputerCameraMovementMode {
    Default = 0,
    Classic = 1,
    Follow = 2,
    Orbital = 3,
    CameraToggle = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ComputerMovementMode {
    Default = 0,
    KeyboardMouse = 1,
    ClickToMove = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ConfigSnapshotErrorState {
    None = 0,
    LoadFailed = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ConnectionError {
    OK = 0,
    Unknown = 1,
    DisconnectErrors = 256,
    DisconnectBadhash = 257,
    DisconnectSecurityKeyMismatch = 258,
    DisconnectProtocolMismatch = 259,
    DisconnectReceivePacketError = 260,
    DisconnectReceivePacketStreamError = 261,
    DisconnectSendPacketError = 262,
    DisconnectIllegalTeleport = 263,
    DisconnectDuplicatePlayer = 264,
    DisconnectDuplicateTicket = 265,
    DisconnectTimeout = 266,
    DisconnectLuaKick = 267,
    DisconnectOnRemoteSysStats = 268,
    DisconnectHashTimeout = 269,
    DisconnectCloudEditKick = 270,
    DisconnectPlayerless = 271,
    DisconnectNewSecurityKeyMismatch = 272,
    DisconnectEvicted = 273,
    DisconnectDevMaintenance = 274,
    DisconnectRobloxMaintenance = 275,
    DisconnectRejoin = 276,
    DisconnectConnectionLost = 277,
    DisconnectIdle = 278,
    DisconnectRaknetErrors = 279,
    DisconnectWrongVersion = 280,
    DisconnectBySecurityPolicy = 281,
    DisconnectBlockedIP = 282,
    DisconnectClientFailure = 284,
    DisconnectClientRequest = 285,
    DisconnectPrivateServerKickout = 286,
    DisconnectModeratedGame = 287,
    ServerShutdown = 288,
    ReplicatorTimeout = 290,
    PlayerRemoved = 291,
    DisconnectOutOfMemoryKeepPlayingLeave = 292,
    DisconnectRomarkEndOfTest = 293,
    DisconnectCollaboratorPermissionRevoked = 294,
    DisconnectCollaboratorUnderage = 295,
    NetworkInternal = 296,
    NetworkSend = 297,
    NetworkTimeout = 298,
    NetworkMisbehavior = 299,
    NetworkSecurity = 300,
    ReplacementReady = 301,
    ServerEmpty = 302,
    PhantomFreeze = 303,
    AndroidAnticheatKick = 304,
    AndroidEmulatorKick = 305,
    PlacelaunchErrors = 512,
    PlacelaunchDisabled = 515,
    PlacelaunchError = 516,
    PlacelaunchGameEnded = 517,
    PlacelaunchGameFull = 518,
    PlacelaunchUserLeft = 522,
    PlacelaunchRestricted = 523,
    PlacelaunchUnauthorized = 524,
    PlacelaunchFlooded = 525,
    PlacelaunchHashExpired = 526,
    PlacelaunchHashException = 527,
    PlacelaunchPartyCannotFit = 528,
    PlacelaunchHttpError = 529,
    PlacelaunchUserPrivacyUnauthorized = 533,
    PlacelaunchCreatorBan = 600,
    PlacelaunchCustomMessage = 610,
    PlacelaunchOtherError = 611,
    TeleportErrors = 768,
    TeleportFailure = 769,
    TeleportGameNotFound = 770,
    TeleportGameEnded = 771,
    TeleportGameFull = 772,
    TeleportUnauthorized = 773,
    TeleportFlooded = 774,
    TeleportIsTeleporting = 775,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ConnectionState {
    Connected = 0,
    Disconnected = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ContentSourceType {
    None = 0,
    Uri = 1,
    Object = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ContextActionPriority {
    Low = 1000,
    Medium = 2000,
    High = 3000,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ContextActionResult {
    Sink = 0,
    Pass = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ControlMode {
    Classic = 0,
    MouseLockSwitch = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CoreGuiType {
    PlayerList = 0,
    Health = 1,
    Backpack = 2,
    Chat = 3,
    All = 4,
    EmotesMenu = 5,
    SelfView = 6,
    Captures = 7,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CreateAssetResult {
    Success = 1,
    PermissionDenied = 2,
    UploadFailed = 3,
    Unknown = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CreateOutfitFailure {
    InvalidName = 1,
    OutfitLimitReached = 2,
    Other = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CreatorType {
    User = 0,
    Group = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CreatorTypeFilter {
    User = 0,
    Group = 1,
    All = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CurrencyType {
    Default = 0,
    Robux = 1,
    Tix = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum CustomCameraMode {
    Default = 0,
    Classic = 1,
    Follow = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DataStoreRequestType {
    GetAsync = 0,
    SetIncrementAsync = 1,
    UpdateAsync = 2,
    GetSortedAsync = 3,
    SetIncrementSortedAsync = 4,
    OnUpdate = 5,
    ListAsync = 6,
    GetVersionAsync = 7,
    RemoveVersionAsync = 8,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DebuggerEndReason {
    ClientRequest = 0,
    Timeout = 1,
    InvalidHost = 2,
    Disconnected = 3,
    ServerShutdown = 4,
    ServerProtocolMismatch = 5,
    ConfigurationFailed = 6,
    RpcError = 7,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DebuggerExceptionBreakMode {
    Never = 0,
    Always = 1,
    Unhandled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DebuggerFrameType {
    C = 0,
    Lua = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DebuggerPauseReason {
    Unknown = 0,
    Requested = 1,
    Breakpoint = 2,
    Exception = 3,
    SingleStep = 4,
    Entrypoint = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DebuggerStatus {
    Success = 0,
    Timeout = 1,
    ConnectionLost = 2,
    InvalidResponse = 3,
    InternalError = 4,
    InvalidState = 5,
    RpcError = 6,
    InvalidArgument = 7,
    ConnectionClosed = 8,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DevCameraOcclusionMode {
    Zoom = 0,
    Invisicam = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DevComputerCameraMovementMode {
    UserChoice = 0,
    Classic = 1,
    Follow = 2,
    Orbital = 3,
    CameraToggle = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DevComputerMovementMode {
    UserChoice = 0,
    KeyboardMouse = 1,
    ClickToMove = 2,
    Scriptable = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DevTouchCameraMovementMode {
    UserChoice = 0,
    Classic = 1,
    Follow = 2,
    Orbital = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DevTouchMovementMode {
    UserChoice = 0,
    Thumbstick = 1,
    DPad = 2,
    Thumbpad = 3,
    ClickToMove = 4,
    Scriptable = 5,
    DynamicThumbstick = 6,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DeveloperMemoryTag {
    Internal = 0,
    HttpCache = 1,
    Instances = 2,
    Signals = 3,
    LuaHeap = 4,
    Script = 5,
    PhysicsCollision = 6,
    PhysicsParts = 7,
    GraphicsSolidModels = 8,
    GraphicsMeshParts = 10,
    GraphicsParticles = 11,
    GraphicsParts = 12,
    GraphicsSpatialHash = 13,
    GraphicsTerrain = 14,
    GraphicsTexture = 15,
    GraphicsTextureCharacter = 16,
    Sounds = 17,
    StreamingSounds = 18,
    TerrainVoxels = 19,
    Gui = 21,
    Animation = 22,
    Navigation = 23,
    GeometryCSG = 24,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DeviceFeatureType {
    DeviceCapture = 0,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DeviceForm {
    Console = 0,
    Phone = 1,
    Tablet = 2,
    Desktop = 3,
    VR = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DeviceLevel {
    Low = 0,
    Medium = 1,
    High = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DeviceType {
    Unknown = 0,
    Desktop = 1,
    Tablet = 2,
    Phone = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DialogBehaviorType {
    SinglePlayer = 0,
    MultiplePlayers = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DialogPurpose {
    Quest = 0,
    Help = 1,
    Shop = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DialogTone {
    Neutral = 0,
    Friendly = 1,
    Enemy = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DominantAxis {
    Width = 0,
    Height = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DraftStatusCode {
    OK = 0,
    DraftOutdated = 1,
    ScriptRemoved = 2,
    DraftCommitted = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DragDetectorDragStyle {
    TranslateLine = 0,
    TranslatePlane = 1,
    TranslatePlaneOrLine = 2,
    TranslateLineOrPlane = 3,
    TranslateViewPlane = 4,
    RotateAxis = 5,
    RotateTrackball = 6,
    Scriptable = 7,
    BestForDevice = 8,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DragDetectorPermissionPolicy {
    Nobody = 0,
    Everybody = 1,
    Scriptable = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DragDetectorResponseStyle {
    Geometric = 0,
    Physical = 1,
    Custom = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DraggerCoordinateSpace {
    Object = 0,
    World = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DraggerMovementMode {
    Geometric = 0,
    Physical = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum DraggingScrollBar {
    None = 0,
    Horizontal = 1,
    Vertical = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum EasingDirection {
    In = 0,
    Out = 1,
    InOut = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum EasingStyle {
    Linear = 0,
    Sine = 1,
    Back = 2,
    Quad = 3,
    Quart = 4,
    Quint = 5,
    Bounce = 6,
    Elastic = 7,
    Exponential = 8,
    Circular = 9,
    Cubic = 10,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum EditableStatus {
    Unknown = 0,
    Allowed = 1,
    Disallowed = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ElasticBehavior {
    WhenScrollable = 0,
    Always = 1,
    Never = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum EnviromentalPhysicsThrottle {
    DefaultAuto = 0,
    Disabled = 1,
    Always = 2,
    Skip2 = 3,
    Skip4 = 4,
    Skip8 = 5,
    Skip16 = 6,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ExperienceAuthScope {
    DefaultScope = 0,
    CreatorAssetsCreate = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ExplosionType {
    NoCraters = 0,
    Craters = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum FACSDataLod {
    LOD0 = 0,
    LOD1 = 1,
    LODCount = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum FacialAgeEstimationResultType {
    Complete = 0,
    Cancel = 1,
    Error = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum FacialAnimationStreamingState {
    None = 0,
    Audio = 1,
    Video = 2,
    Place = 4,
    Server = 8,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum FacsActionUnit {
    ChinRaiserUpperLip = 0,
    ChinRaiser = 1,
    FlatPucker = 2,
    Funneler = 3,
    LowerLipSuck = 4,
    LipPresser = 5,
    LipsTogether = 6,
    MouthLeft = 7,
    MouthRight = 8,
    Pucker = 9,
    UpperLipSuck = 10,
    LeftCheekPuff = 11,
    LeftDimpler = 12,
    LeftLipCornerDown = 13,
    LeftLowerLipDepressor = 14,
    LeftLipCornerPuller = 15,
    LeftLipStretcher = 16,
    LeftUpperLipRaiser = 17,
    RightCheekPuff = 18,
    RightDimpler = 19,
    RightLipCornerDown = 20,
    RightLowerLipDepressor = 21,
    RightLipCornerPuller = 22,
    RightLipStretcher = 23,
    RightUpperLipRaiser = 24,
    JawDrop = 25,
    JawLeft = 26,
    JawRight = 27,
    Corrugator = 28,
    LeftBrowLowerer = 29,
    LeftOuterBrowRaiser = 30,
    LeftNoseWrinkler = 31,
    LeftInnerBrowRaiser = 32,
    RightBrowLowerer = 33,
    RightOuterBrowRaiser = 34,
    RightInnerBrowRaiser = 35,
    RightNoseWrinkler = 36,
    EyesLookDown = 37,
    EyesLookLeft = 38,
    EyesLookUp = 39,
    EyesLookRight = 40,
    LeftCheekRaiser = 41,
    LeftEyeUpperLidRaiser = 42,
    LeftEyeClosed = 43,
    RightCheekRaiser = 44,
    RightEyeUpperLidRaiser = 45,
    RightEyeClosed = 46,
    TongueDown = 47,
    TongueOut = 48,
    TongueUp = 49,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum FeedRankingScoreType {
    Content = 0,
    Final = 1,
    GameJoin = 2,
    Interaction = 3,
    Invalid = 4,
    Sharing = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum FieldOfViewMode {
    Vertical = 0,
    Diagonal = 1,
    MaxAxis = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum FillDirection {
    Horizontal = 0,
    Vertical = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum FilterErrorType {
    BackslashNotEscapingAnything = 0,
    BadBespokeFilter = 1,
    BadName = 2,
    IncompleteOr = 3,
    IncompleteParenthesis = 4,
    InvalidDoubleStar = 5,
    InvalidTilde = 6,
    PropertyBadOperator = 7,
    PropertyDoesNotExist = 8,
    PropertyInvalidField = 9,
    PropertyInvalidValue = 10,
    PropertyUnsupportedFields = 11,
    PropertyUnsupportedProperty = 12,
    UnexpectedNameIndex = 13,
    UnexpectedToken = 14,
    UnfinishedBinaryOperator = 15,
    UnfinishedQuote = 16,
    UnknownBespokeFilter = 17,
    WildcardInProperty = 18,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum FilterResult {
    Accepted = 0,
    Rejected = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum FinishRecordingOperation {
    Cancel = 0,
    Commit = 1,
    Append = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum FluidFidelity {
    Automatic = 0,
    UseCollisionGeometry = 1,
    UsePreciseGeometry = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum FluidForces {
    Default = 0,
    Experimental = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum Font {
    Legacy = 0,
    Arial = 1,
    ArialBold = 2,
    SourceSans = 3,
    SourceSansBold = 4,
    SourceSansLight = 5,
    SourceSansItalic = 6,
    Bodoni = 7,
    Garamond = 8,
    Cartoon = 9,
    Code = 10,
    Highway = 11,
    SciFi = 12,
    Arcade = 13,
    Fantasy = 14,
    Antique = 15,
    SourceSansSemibold = 16,
    Gotham = 17,
    GothamMedium = 18,
    GothamBold = 19,
    GothamBlack = 20,
    AmaticSC = 21,
    Bangers = 22,
    Creepster = 23,
    DenkOne = 24,
    Fondamento = 25,
    FredokaOne = 26,
    GrenzeGotisch = 27,
    IndieFlower = 28,
    JosefinSans = 29,
    Jura = 30,
    Kalam = 31,
    LuckiestGuy = 32,
    Merriweather = 33,
    Michroma = 34,
    Nunito = 35,
    Oswald = 36,
    PatrickHand = 37,
    PermanentMarker = 38,
    Roboto = 39,
    RobotoCondensed = 40,
    RobotoMono = 41,
    Sarpanch = 42,
    SpecialElite = 43,
    TitilliumWeb = 44,
    Ubuntu = 45,
    BuilderSans = 46,
    BuilderSansMedium = 47,
    BuilderSansBold = 48,
    BuilderSansExtraBold = 49,
    Arimo = 50,
    ArimoBold = 51,
    Unknown = 100,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum FontSize {
    Size8 = 0,
    Size9 = 1,
    Size10 = 2,
    Size11 = 3,
    Size12 = 4,
    Size14 = 5,
    Size18 = 6,
    Size24 = 7,
    Size36 = 8,
    Size48 = 9,
    Size28 = 10,
    Size32 = 11,
    Size42 = 12,
    Size60 = 13,
    Size96 = 14,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum FontStyle {
    Normal = 0,
    Italic = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Regular = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Heavy = 900,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ForceLimitMode {
    Magnitude = 0,
    PerAxis = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum FormFactor {
    Symmetric = 0,
    Brick = 1,
    Plate = 2,
    Custom = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum FrameStyle {
    Custom = 0,
    ChatBlue = 1,
    RobloxSquare = 2,
    RobloxRound = 3,
    ChatGreen = 4,
    ChatRed = 5,
    DropShadow = 6,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum FramerateManagerMode {
    Automatic = 0,
    On = 1,
    Off = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum FriendRequestEvent {
    Issue = 0,
    Revoke = 1,
    Accept = 2,
    Deny = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum FriendStatus {
    Unknown = 0,
    NotFriend = 1,
    Friend = 2,
    FriendRequestSent = 3,
    FriendRequestReceived = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum FunctionalTestResult {
    Passed = 0,
    Warning = 1,
    Error = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum GameAvatarType {
    R6 = 0,
    R15 = 1,
    PlayerChoice = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum GamepadType {
    Unknown = 0,
    PS4 = 1,
    PS5 = 2,
    XboxOne = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum GearGenreSetting {
    AllGenres = 0,
    MatchingGenreOnly = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum GearType {
    MeleeWeapons = 0,
    RangedWeapons = 1,
    Explosives = 2,
    PowerUps = 3,
    NavigationEnhancers = 4,
    MusicalInstruments = 5,
    SocialItems = 6,
    BuildingTools = 7,
    Transport = 8,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum Genre {
    All = 0,
    TownAndCity = 1,
    Fantasy = 2,
    SciFi = 3,
    Ninja = 4,
    Scary = 5,
    Pirate = 6,
    Adventure = 7,
    Sports = 8,
    Funny = 9,
    WildWest = 10,
    War = 11,
    SkatePark = 12,
    Tutorial = 13,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum GraphicsMode {
    Automatic = 1,
    Direct3D11 = 2,
    OpenGL = 4,
    Metal = 5,
    Vulkan = 6,
    NoGraphics = 9,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum GraphicsOptimizationMode {
    Performance = 0,
    Balanced = 1,
    Quality = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum GuiState {
    Idle = 0,
    Hover = 1,
    Press = 2,
    NonInteractable = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum GuiType {
    Core = 0,
    Custom = 1,
    PlayerNameplates = 2,
    CustomBillboards = 3,
    CoreBillboards = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum HandlesStyle {
    Resize = 0,
    Movement = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum HapticEffectType {
    Custom = 0,
    UIHover = 1,
    UIClick = 2,
    UINotification = 3,
    GameplayExplosion = 4,
    GameplayCollision = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum HighlightDepthMode {
    AlwaysOnTop = 0,
    Occluded = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum HorizontalAlignment {
    Center = 0,
    Left = 1,
    Right = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum HoverAnimateSpeed {
    VerySlow = 0,
    Slow = 1,
    Medium = 2,
    Fast = 3,
    VeryFast = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum HttpCachePolicy {
    None = 0,
    Full = 1,
    DataOnly = 2,
    Default = 3,
    InternalRedirectRefresh = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum HttpCompression {
    None = 0,
    Gzip = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum HttpContentType {
    ApplicationJson = 0,
    ApplicationXml = 1,
    ApplicationUrlEncoded = 2,
    TextPlain = 3,
    TextXml = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum HttpError {
    OK = 0,
    InvalidUrl = 1,
    DnsResolve = 2,
    ConnectFail = 3,
    OutOfMemory = 4,
    TimedOut = 5,
    TooManyRedirects = 6,
    InvalidRedirect = 7,
    NetFail = 8,
    Aborted = 9,
    SslConnectFail = 10,
    SslVerificationFail = 11,
    Unknown = 12,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum HttpRequestType {
    Default = 0,
    MarketplaceService = 2,
    Players = 7,
    Chat = 15,
    Avatar = 16,
    Analytics = 23,
    Localization = 25,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum HumanoidCollisionType {
    OuterBox = 0,
    InnerBox = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum HumanoidDisplayDistanceType {
    Viewer = 0,
    Subject = 1,
    None = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum HumanoidHealthDisplayType {
    DisplayWhenDamaged = 0,
    AlwaysOn = 1,
    AlwaysOff = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum HumanoidRigType {
    R6 = 0,
    R15 = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum HumanoidStateType {
    FallingDown = 0,
    Ragdoll = 1,
    GettingUp = 2,
    Jumping = 3,
    Swimming = 4,
    Freefall = 5,
    Flying = 6,
    Landed = 7,
    Running = 8,
    RunningNoPhysics = 10,
    StrafingNoPhysics = 11,
    Climbing = 12,
    Seated = 13,
    PlatformStanding = 14,
    Dead = 15,
    Physics = 16,
    None = 18,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum IKCollisionsMode {
    NoCollisions = 0,
    OtherMechanismsAnchored = 1,
    IncludeContactedMechanisms = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum IKControlConstraintSupport {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum IKControlType {
    Transform = 0,
    Position = 1,
    Rotation = 2,
    LookAt = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum IXPLoadingStatus {
    None = 0,
    Pending = 1,
    Initialized = 2,
    ErrorInvalidUser = 3,
    ErrorConnection = 4,
    ErrorJsonParse = 5,
    ErrorTimedOut = 6,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ImageAlphaType {
    Default = 1,
    LockCanvasAlpha = 2,
    LockCanvasColor = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ImageCombineType {
    BlendSourceOver = 1,
    Overwrite = 2,
    Add = 3,
    Multiply = 4,
    AlphaBlend = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum InOut {
    Edge = 0,
    Inset = 1,
    Center = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum InfoType {
    Asset = 0,
    Product = 1,
    GamePass = 2,
    Subscription = 3,
    Bundle = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum InitialDockState {
    Top = 0,
    Bottom = 1,
    Left = 2,
    Right = 3,
    Float = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum InputActionType {
    Bool = 0,
    Float = 1,
    Direction2D = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum InputType {
    NoInput = 0,
    Constant = 12,
    Sin = 13,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum IntermediateMeshGenerationResult {
    HighQualityMesh = 0,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum InterpolationThrottlingMode {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum InviteState {
    Placed = 0,
    Accepted = 1,
    Declined = 2,
    Missed = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ItemLineAlignment {
    Automatic = 0,
    Start = 1,
    Center = 2,
    End = 3,
    Stretch = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum JoinSource {
    CreatedItemAttribution = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum JointCreationMode {
    All = 0,
    Surface = 1,
    None = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum KeyCode {
    Unknown = 0,
    Backspace = 8,
    Tab = 9,
    Clear = 12,
    Return = 13,
    Pause = 19,
    Escape = 27,
    Space = 32,
    QuotedDouble = 34,
    Hash = 35,
    Dollar = 36,
    Percent = 37,
    Ampersand = 38,
    Quote = 39,
    LeftParenthesis = 40,
    RightParenthesis = 41,
    Asterisk = 42,
    Plus = 43,
    Comma = 44,
    Minus = 45,
    Period = 46,
    Slash = 47,
    Zero = 48,
    One = 49,
    Two = 50,
    Three = 51,
    Four = 52,
    Five = 53,
    Six = 54,
    Seven = 55,
    Eight = 56,
    Nine = 57,
    Colon = 58,
    Semicolon = 59,
    LessThan = 60,
    Equals = 61,
    GreaterThan = 62,
    Question = 63,
    At = 64,
    LeftBracket = 91,
    BackSlash = 92,
    RightBracket = 93,
    Caret = 94,
    Underscore = 95,
    Backquote = 96,
    A = 97,
    B = 98,
    C = 99,
    D = 100,
    E = 101,
    F = 102,
    G = 103,
    H = 104,
    I = 105,
    J = 106,
    K = 107,
    L = 108,
    M = 109,
    N = 110,
    O = 111,
    P = 112,
    Q = 113,
    R = 114,
    S = 115,
    T = 116,
    U = 117,
    V = 118,
    W = 119,
    X = 120,
    Y = 121,
    Z = 122,
    LeftCurly = 123,
    Pipe = 124,
    RightCurly = 125,
    Tilde = 126,
    Delete = 127,
    World0 = 160,
    World1 = 161,
    World2 = 162,
    World3 = 163,
    World4 = 164,
    World5 = 165,
    World6 = 166,
    World7 = 167,
    World8 = 168,
    World9 = 169,
    World10 = 170,
    World11 = 171,
    World12 = 172,
    World13 = 173,
    World14 = 174,
    World15 = 175,
    World16 = 176,
    World17 = 177,
    World18 = 178,
    World19 = 179,
    World20 = 180,
    World21 = 181,
    World22 = 182,
    World23 = 183,
    World24 = 184,
    World25 = 185,
    World26 = 186,
    World27 = 187,
    World28 = 188,
    World29 = 189,
    World30 = 190,
    World31 = 191,
    World32 = 192,
    World33 = 193,
    World34 = 194,
    World35 = 195,
    World36 = 196,
    World37 = 197,
    World38 = 198,
    World39 = 199,
    World40 = 200,
    World41 = 201,
    World42 = 202,
    World43 = 203,
    World44 = 204,
    World45 = 205,
    World46 = 206,
    World47 = 207,
    World48 = 208,
    World49 = 209,
    World50 = 210,
    World51 = 211,
    World52 = 212,
    World53 = 213,
    World54 = 214,
    World55 = 215,
    World56 = 216,
    World57 = 217,
    World58 = 218,
    World59 = 219,
    World60 = 220,
    World61 = 221,
    World62 = 222,
    World63 = 223,
    World64 = 224,
    World65 = 225,
    World66 = 226,
    World67 = 227,
    World68 = 228,
    World69 = 229,
    World70 = 230,
    World71 = 231,
    World72 = 232,
    World73 = 233,
    World74 = 234,
    World75 = 235,
    World76 = 236,
    World77 = 237,
    World78 = 238,
    World79 = 239,
    World80 = 240,
    World81 = 241,
    World82 = 242,
    World83 = 243,
    World84 = 244,
    World85 = 245,
    World86 = 246,
    World87 = 247,
    World88 = 248,
    World89 = 249,
    World90 = 250,
    World91 = 251,
    World92 = 252,
    World93 = 253,
    World94 = 254,
    World95 = 255,
    KeypadZero = 256,
    KeypadOne = 257,
    KeypadTwo = 258,
    KeypadThree = 259,
    KeypadFour = 260,
    KeypadFive = 261,
    KeypadSix = 262,
    KeypadSeven = 263,
    KeypadEight = 264,
    KeypadNine = 265,
    KeypadPeriod = 266,
    KeypadDivide = 267,
    KeypadMultiply = 268,
    KeypadMinus = 269,
    KeypadPlus = 270,
    KeypadEnter = 271,
    KeypadEquals = 272,
    Up = 273,
    Down = 274,
    Right = 275,
    Left = 276,
    Insert = 277,
    Home = 278,
    End = 279,
    PageUp = 280,
    PageDown = 281,
    F1 = 282,
    F2 = 283,
    F3 = 284,
    F4 = 285,
    F5 = 286,
    F6 = 287,
    F7 = 288,
    F8 = 289,
    F9 = 290,
    F10 = 291,
    F11 = 292,
    F12 = 293,
    F13 = 294,
    F14 = 295,
    F15 = 296,
    NumLock = 300,
    CapsLock = 301,
    ScrollLock = 302,
    RightShift = 303,
    LeftShift = 304,
    RightControl = 305,
    LeftControl = 306,
    RightAlt = 307,
    LeftAlt = 308,
    RightMeta = 309,
    LeftMeta = 310,
    LeftSuper = 311,
    RightSuper = 312,
    Mode = 313,
    Compose = 314,
    Help = 315,
    Print = 316,
    SysReq = 317,
    Break = 318,
    Menu = 319,
    Power = 320,
    Euro = 321,
    Undo = 322,
    ButtonX = 1000,
    ButtonY = 1001,
    ButtonA = 1002,
    ButtonB = 1003,
    ButtonR1 = 1004,
    ButtonL1 = 1005,
    ButtonR2 = 1006,
    ButtonL2 = 1007,
    ButtonR3 = 1008,
    ButtonL3 = 1009,
    ButtonStart = 1010,
    ButtonSelect = 1011,
    DPadLeft = 1012,
    DPadRight = 1013,
    DPadUp = 1014,
    DPadDown = 1015,
    Thumbstick1 = 1016,
    Thumbstick2 = 1017,
    MouseLeftButton = 1018,
    MouseRightButton = 1019,
    MouseMiddleButton = 1020,
    MouseBackButton = 1021,
    MouseNoButton = 1022,
    MouseX = 1023,
    MouseY = 1024,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum KeyInterpolationMode {
    Constant = 0,
    Linear = 1,
    Cubic = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum KeywordFilterType {
    Include = 0,
    Exclude = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum Language {
    Default = 0,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum LeftRight {
    Left = 0,
    Center = 1,
    Right = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum LexemeType {
    Eof = 0,
    Name = 1,
    QuotedString = 2,
    Number = 3,
    And = 4,
    Or = 5,
    Equal = 6,
    TildeEqual = 7,
    GreaterThan = 8,
    GreaterThanEqual = 9,
    LessThan = 10,
    LessThanEqual = 11,
    Colon = 12,
    Dot = 13,
    LeftParenthesis = 14,
    RightParenthesis = 15,
    Star = 16,
    DoubleStar = 17,
    ReservedSpecial = 18,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum LightingStyle {
    Realistic = 0,
    Soft = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum Limb {
    Head = 0,
    Torso = 1,
    LeftArm = 2,
    RightArm = 3,
    LeftLeg = 4,
    RightLeg = 5,
    Unknown = 6,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum LineJoinMode {
    Round = 0,
    Bevel = 1,
    Miter = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ListDisplayMode {
    Horizontal = 0,
    Vertical = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ListenerLocation {
    Default = 0,
    None = 1,
    Character = 2,
    Camera = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ListenerType {
    Camera = 0,
    CFrame = 1,
    ObjectPosition = 2,
    ObjectCFrame = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum LiveEditingAtomicUpdateResponse {
    Success = 0,
    FailureGuidNotFound = 1,
    FailureHashMismatch = 2,
    FailureOperationIllegal = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum LiveEditingBroadcastMessageType {
    Normal = 0,
    Warning = 1,
    Error = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum LoadCharacterLayeredClothing {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum LoadDynamicHeads {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum LocationType {
    Character = 0,
    Camera = 1,
    ObjectPosition = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum MarketplaceBulkPurchasePromptStatus {
    Completed = 1,
    Aborted = 2,
    Error = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum MarketplaceItemPurchaseStatus {
    Success = 1,
    SystemError = 2,
    AlreadyOwned = 3,
    InsufficientRobux = 4,
    QuantityLimitExceeded = 5,
    QuotaExceeded = 6,
    NotForSale = 7,
    NotAvailableForPurchaser = 8,
    PriceMismatch = 9,
    SoldOut = 10,
    PurchaserIsSeller = 11,
    InsufficientMembership = 12,
    PlaceInvalid = 13,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum MarketplaceProductType {
    AvatarAsset = 1,
    AvatarBundle = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum MarkupKind {
    PlainText = 0,
    Markdown = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum MatchmakingType {
    Default = 1,
    XboxOnly = 2,
    PlayStationOnly = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum Material {
    Plastic = 256,
    SmoothPlastic = 272,
    Neon = 288,
    Wood = 512,
    WoodPlanks = 528,
    Marble = 784,
    Basalt = 788,
    Slate = 800,
    CrackedLava = 804,
    Concrete = 816,
    Limestone = 820,
    Granite = 832,
    Pavement = 836,
    Brick = 848,
    Pebble = 864,
    Cobblestone = 880,
    Rock = 896,
    Sandstone = 912,
    CorrodedMetal = 1040,
    DiamondPlate = 1056,
    Foil = 1072,
    Metal = 1088,
    Grass = 1280,
    LeafyGrass = 1284,
    Sand = 1296,
    Fabric = 1312,
    Snow = 1328,
    Mud = 1344,
    Ground = 1360,
    Asphalt = 1376,
    Salt = 1392,
    Ice = 1536,
    Glacier = 1552,
    Glass = 1568,
    ForceField = 1584,
    Air = 1792,
    Water = 2048,
    Cardboard = 2304,
    Carpet = 2305,
    CeramicTiles = 2306,
    ClayRoofTiles = 2307,
    RoofShingles = 2308,
    Leather = 2309,
    Plaster = 2310,
    Rubber = 2311,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum MaterialPattern {
    Regular = 0,
    Organic = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum MembershipType {
    None = 0,
    BuildersClub = 1,
    TurboBuildersClub = 2,
    OutrageousBuildersClub = 3,
    Premium = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum MeshPartDetailLevel {
    DistanceBased = 0,
    Level00 = 1,
    Level01 = 2,
    Level02 = 3,
    Level03 = 4,
    Level04 = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum MeshPartHeadsAndAccessories {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum MeshScaleUnit {
    Stud = 0,
    Meter = 1,
    CM = 2,
    MM = 3,
    Foot = 4,
    Inch = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum MeshType {
    Head = 0,
    Torso = 1,
    Wedge = 2,
    Sphere = 3,
    Cylinder = 4,
    FileMesh = 5,
    Brick = 6,
    Prism = 7,
    Pyramid = 8,
    ParallelRamp = 9,
    RightAngleRamp = 10,
    CornerWedge = 11,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum MessageType {
    MessageOutput = 0,
    MessageInfo = 1,
    MessageWarning = 2,
    MessageError = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ModelLevelOfDetail {
    Automatic = 0,
    StreamingMesh = 1,
    Disabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ModelStreamingBehavior {
    Default = 0,
    Legacy = 1,
    Improved = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ModelStreamingMode {
    Default = 0,
    Atomic = 1,
    Persistent = 2,
    PersistentPerPlayer = 3,
    Nonatomic = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ModerationStatus {
    ReviewedApproved = 1,
    ReviewedRejected = 2,
    NotReviewed = 3,
    NotApplicable = 4,
    Invalid = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ModifierKey {
    Shift = 0,
    Ctrl = 1,
    Alt = 2,
    Meta = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum MouseBehavior {
    Default = 0,
    LockCenter = 1,
    LockCurrentPosition = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum MoveState {
    Stopped = 0,
    Coasting = 1,
    Pushing = 2,
    Stopping = 3,
    AirFree = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum MoverConstraintRootBehaviorMode {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum MuteState {
    Unmuted = 0,
    Muted = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum NameOcclusion {
    NoOcclusion = 0,
    EnemyOcclusion = 1,
    OccludeAll = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum NetworkOwnership {
    Automatic = 0,
    Manual = 1,
    OnContact = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum NetworkStatus {
    Unknown = 0,
    Connected = 1,
    Disconnected = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum NoiseType {
    SimplexGabor = 0,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum NormalId {
    Right = 0,
    Top = 1,
    Back = 2,
    Left = 3,
    Bottom = 4,
    Front = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum NotificationButtonType {
    Primary = 0,
    Secondary = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum OperationType {
    Null = 0,
    Union = 1,
    Subtraction = 2,
    Intersection = 3,
    Primitive = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum OrientationAlignmentMode {
    OneAttachment = 0,
    TwoAttachment = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum OutfitSource {
    All = 1,
    Created = 2,
    Purchased = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum OutfitType {
    All = 1,
    Avatar = 2,
    DynamicHead = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum OutputLayoutMode {
    Horizontal = 0,
    Vertical = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum OverrideMouseIconBehavior {
    None = 0,
    ForceShow = 1,
    ForceHide = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PackagePermission {
    None = 0,
    NoAccess = 1,
    Revoked = 2,
    UseView = 3,
    Edit = 4,
    Own = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PartType {
    Ball = 0,
    Block = 1,
    Cylinder = 2,
    Wedge = 3,
    CornerWedge = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ParticleEmitterShape {
    Box = 0,
    Sphere = 1,
    Cylinder = 2,
    Disc = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ParticleEmitterShapeInOut {
    Outward = 0,
    Inward = 1,
    InAndOut = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ParticleEmitterShapeStyle {
    Volume = 0,
    Surface = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ParticleFlipbookLayout {
    None = 0,
    Grid2x2 = 1,
    Grid4x4 = 2,
    Grid8x8 = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ParticleFlipbookMode {
    Loop = 0,
    OneShot = 1,
    PingPong = 2,
    Random = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ParticleFlipbookTextureCompatible {
    NotCompatible = 0,
    Compatible = 1,
    Unknown = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ParticleOrientation {
    FacingCamera = 0,
    FacingCameraWorldUp = 1,
    VelocityParallel = 2,
    VelocityPerpendicular = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PathStatus {
    Success = 0,
    ClosestNoPath = 1,
    ClosestOutOfRange = 2,
    FailStartNotEmpty = 3,
    FailFinishNotEmpty = 4,
    NoPath = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PathWaypointAction {
    Walk = 0,
    Jump = 1,
    Custom = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PathfindingUseImprovedSearch {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PermissionLevelShown {
    Game = 0,
    RobloxGame = 1,
    RobloxScript = 2,
    Studio = 3,
    Roblox = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PhysicsSimulationRate {
    Fixed240Hz = 0,
    Fixed120Hz = 1,
    Fixed60Hz = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PhysicsSteppingMethod {
    Default = 0,
    Fixed = 1,
    Adaptive = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum Platform {
    Windows = 0,
    OSX = 1,
    IOS = 2,
    Android = 3,
    XBoxOne = 4,
    PS4 = 5,
    PS3 = 6,
    XBox360 = 7,
    WiiU = 8,
    NX = 9,
    Ouya = 10,
    AndroidTV = 11,
    Chromecast = 12,
    Linux = 13,
    SteamOS = 14,
    WebOS = 15,
    DOS = 16,
    BeOS = 17,
    UWP = 18,
    PS5 = 19,
    MetaOS = 20,
    None = 21,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PlaybackState {
    Begin = 0,
    Delayed = 1,
    Playing = 2,
    Paused = 3,
    Completed = 4,
    Cancelled = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PlayerActions {
    CharacterForward = 0,
    CharacterBackward = 1,
    CharacterLeft = 2,
    CharacterRight = 3,
    CharacterJump = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PlayerCharacterDestroyBehavior {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PlayerChatType {
    All = 0,
    Team = 1,
    Whisper = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PlayerDataErrorState {
    LoadFailed = 0,
    FlushFailed = 1,
    ReleaseFailed = 2,
    None = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PlayerDataLoadFailureBehavior {
    Failure = 0,
    FallbackToDefault = 1,
    Kick = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PoseEasingDirection {
    In = 0,
    Out = 1,
    InOut = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PoseEasingStyle {
    Linear = 0,
    Constant = 1,
    Elastic = 2,
    Cubic = 3,
    Bounce = 4,
    CubicV2 = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PositionAlignmentMode {
    OneAttachment = 0,
    TwoAttachment = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PreferredTextSize {
    Medium = 1,
    Large = 2,
    Larger = 3,
    Largest = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PrimalPhysicsSolver {
    Default = 0,
    Experimental = 1,
    Disabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PrimitiveType {
    Null = 0,
    Ball = 1,
    Cylinder = 2,
    Block = 3,
    Wedge = 4,
    CornerWedge = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PrivilegeType {
    Banned = 0,
    Visitor = 10,
    Member = 128,
    Admin = 240,
    Owner = 255,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ProductLocationRestriction {
    AvatarShop = 0,
    AllowedGames = 1,
    AllGames = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ProductPurchaseChannel {
    InExperience = 1,
    ExperienceDetailsPage = 2,
    AdReward = 3,
    CommerceProduct = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ProductPurchaseDecision {
    NotProcessedYet = 0,
    PurchaseGranted = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PromptCreateAssetResult {
    Success = 1,
    PermissionDenied = 2,
    Timeout = 3,
    UploadFailed = 4,
    NoUserInput = 5,
    UnknownFailure = 6,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PromptCreateAvatarResult {
    Success = 1,
    PermissionDenied = 2,
    Timeout = 3,
    UploadFailed = 4,
    NoUserInput = 5,
    InvalidHumanoidDescription = 6,
    UGCValidationFailed = 7,
    ModeratedName = 8,
    MaxOutfits = 9,
    PurchaseFailure = 10,
    UnknownFailure = 11,
    TokenInvalid = 12,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PromptPublishAssetResult {
    Success = 1,
    PermissionDenied = 2,
    Timeout = 3,
    UploadFailed = 4,
    NoUserInput = 5,
    UnknownFailure = 6,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum PropertyStatus {
    Ok = 0,
    Warning = 1,
    Error = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ProximityPromptExclusivity {
    OnePerButton = 0,
    OneGlobally = 1,
    AlwaysShow = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ProximityPromptInputType {
    Keyboard = 0,
    Gamepad = 1,
    Touch = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ProximityPromptStyle {
    Default = 0,
    Custom = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum QualityLevel {
    Automatic = 0,
    Level01 = 1,
    Level02 = 2,
    Level03 = 3,
    Level04 = 4,
    Level05 = 5,
    Level06 = 6,
    Level07 = 7,
    Level08 = 8,
    Level09 = 9,
    Level10 = 10,
    Level11 = 11,
    Level12 = 12,
    Level13 = 13,
    Level14 = 14,
    Level15 = 15,
    Level16 = 16,
    Level17 = 17,
    Level18 = 18,
    Level19 = 19,
    Level20 = 20,
    Level21 = 21,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum R15CollisionType {
    OuterBox = 0,
    InnerBox = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum RaycastFilterType {
    Exclude = 0,
    Include = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum RejectCharacterDeletions {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum RenderFidelity {
    Automatic = 0,
    Precise = 1,
    Performance = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum RenderPriority {
    First = 0,
    Input = 100,
    Camera = 200,
    Character = 300,
    Last = 2000,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum RenderingCacheOptimizationMode {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum RenderingTestComparisonMethod {
    psnr = 0,
    diff = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ReplicateInstanceDestroySetting {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ResamplerMode {
    Default = 0,
    Pixelated = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ReservedHighlightId {
    Standard = 0,
    Active = 131072,
    Hover = 262144,
    Selection = 524288,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum RestPose {
    Default = 0,
    RotationsReset = 1,
    Custom = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ReturnKeyType {
    Default = 0,
    Done = 1,
    Go = 2,
    Next = 3,
    Search = 4,
    Send = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ReverbType {
    NoReverb = 0,
    GenericReverb = 1,
    PaddedCell = 2,
    Room = 3,
    Bathroom = 4,
    LivingRoom = 5,
    StoneRoom = 6,
    Auditorium = 7,
    ConcertHall = 8,
    Cave = 9,
    Arena = 10,
    Hangar = 11,
    CarpettedHallway = 12,
    Hallway = 13,
    StoneCorridor = 14,
    Alley = 15,
    Forest = 16,
    City = 17,
    Mountains = 18,
    Quarry = 19,
    Plain = 20,
    ParkingLot = 21,
    SewerPipe = 22,
    UnderWater = 23,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum RibbonTool {
    Select = 0,
    Scale = 1,
    Rotate = 2,
    Move = 3,
    Transform = 4,
    ColorPicker = 5,
    MaterialPicker = 6,
    Group = 7,
    Ungroup = 8,
    None = 9,
    PivotEditor = 10,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum RigScale {
    Default = 0,
    Rthro = 1,
    RthroNarrow = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum RigType {
    R15 = 0,
    Custom = 1,
    None = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum RollOffMode {
    Inverse = 0,
    Linear = 1,
    LinearSquare = 2,
    InverseTapered = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum RolloutState {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum RotationOrder {
    XYZ = 0,
    XZY = 1,
    YZX = 2,
    YXZ = 3,
    ZXY = 4,
    ZYX = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum RotationType {
    MovementRelative = 0,
    CameraRelative = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum RtlTextSupport {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum RunContext {
    Legacy = 0,
    Server = 1,
    Client = 2,
    Plugin = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum RunState {
    Stopped = 0,
    Running = 1,
    Paused = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum RuntimeUndoBehavior {
    Aggregate = 0,
    Snapshot = 1,
    Hybrid = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SafeAreaCompatibility {
    None = 0,
    FullscreenExtension = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SalesTypeFilter {
    All = 1,
    Collectibles = 2,
    Premium = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SandboxedInstanceMode {
    Default = 0,
    Experimental = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SaveAvatarThumbnailCustomizationFailure {
    BadThumbnailType = 1,
    BadYRotDeg = 2,
    BadFieldOfViewDeg = 3,
    BadDistanceScale = 4,
    Other = 5,
    Throttled = 6,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SaveFilter {
    SaveWorld = 0,
    SaveGame = 1,
    SaveAll = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SavedQualitySetting {
    Automatic = 0,
    QualityLevel1 = 1,
    QualityLevel2 = 2,
    QualityLevel3 = 3,
    QualityLevel4 = 4,
    QualityLevel5 = 5,
    QualityLevel6 = 6,
    QualityLevel7 = 7,
    QualityLevel8 = 8,
    QualityLevel9 = 9,
    QualityLevel10 = 10,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ScaleType {
    Stretch = 0,
    Slice = 1,
    Tile = 2,
    Fit = 3,
    Crop = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ScopeCheckResult {
    ConsentAccepted = 0,
    InvalidScopes = 1,
    Timeout = 2,
    NoUserInput = 3,
    BackendError = 4,
    UnexpectedError = 5,
    InvalidArgument = 6,
    ConsentDenied = 7,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ScreenInsets {
    None = 0,
    DeviceSafeInsets = 1,
    CoreUISafeInsets = 2,
    TopbarSafeInsets = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ScreenOrientation {
    LandscapeLeft = 0,
    LandscapeRight = 1,
    LandscapeSensor = 2,
    Portrait = 3,
    Sensor = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ScrollBarInset {
    None = 0,
    ScrollBar = 1,
    Always = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ScrollingDirection {
    X = 1,
    Y = 2,
    XY = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SecurityCapability {
    RunClientScript = 0,
    RunServerScript = 1,
    AccessOutsideWrite = 2,
    AssetRequire = 3,
    LoadString = 4,
    ScriptGlobals = 5,
    CreateInstances = 6,
    Basic = 7,
    Audio = 8,
    DataStore = 9,
    Network = 10,
    Physics = 11,
    UI = 12,
    CSG = 13,
    Chat = 14,
    Animation = 15,
    Avatar = 16,
    Input = 17,
    Environment = 18,
    RemoteEvent = 19,
    LegacySound = 20,
    Players = 21,
    CapabilityControl = 22,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SelectionBehavior {
    Escape = 0,
    Stop = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SelectionRenderMode {
    Outlines = 0,
    BoundingBoxes = 1,
    Both = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SelfViewPosition {
    LastPosition = 0,
    TopLeft = 1,
    TopRight = 2,
    BottomLeft = 3,
    BottomRight = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SensorMode {
    Floor = 0,
    Ladder = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SensorUpdateType {
    OnRead = 0,
    Manual = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ServerLiveEditingMode {
    Uninitialized = 0,
    Enabled = 1,
    Disabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ServiceVisibility {
    Always = 0,
    Off = 1,
    WithChildren = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum Severity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ShowAdResult {
    ShowCompleted = 1,
    AdNotReady = 2,
    AdAlreadyShowing = 3,
    InternalError = 4,
    ShowInterrupted = 5,
    InsufficientMemory = 6,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SignalBehavior {
    Default = 0,
    Immediate = 1,
    Deferred = 2,
    AncestryDeferred = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SizeConstraint {
    RelativeXY = 0,
    RelativeXX = 1,
    RelativeYY = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SolverConvergenceMetricType {
    IterationBased = 0,
    AlgorithmAgnostic = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SolverConvergenceVisualizationMode {
    Disabled = 0,
    PerIsland = 1,
    PerEdge = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SortDirection {
    Ascending = 0,
    Descending = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SortOrder {
    Name = 0,
    Custom = 1,
    LayoutOrder = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SpecialKey {
    Insert = 0,
    Home = 1,
    End = 2,
    PageUp = 3,
    PageDown = 4,
    ChatHotkey = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum StartCorner {
    TopLeft = 0,
    TopRight = 1,
    BottomLeft = 2,
    BottomRight = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum StateObjectFieldType {
    Boolean = 0,
    CFrame = 1,
    Color3 = 2,
    Float = 3,
    Instance = 4,
    Random = 5,
    Vector2 = 6,
    Vector3 = 7,
    INVALID = 8,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum Status {
    Poison = 0,
    Confusion = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum StreamOutBehavior {
    Default = 0,
    LowMemory = 1,
    Opportunistic = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum StreamingIntegrityMode {
    Default = 0,
    Disabled = 1,
    MinimumRadiusPause = 2,
    PauseOutsideLoadedArea = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum StreamingPauseMode {
    Default = 0,
    Disabled = 1,
    ClientPhysicsPause = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum StudioCloseMode {
    None = 0,
    CloseStudio = 1,
    CloseDoc = 2,
    LogOut = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum StudioDataModelType {
    Edit = 0,
    PlayClient = 1,
    PlayServer = 2,
    Standalone = 3,
    None = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum StudioPlaceUpdateFailureReason {
    Other = 0,
    TeamCreateConflict = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum StudioScriptEditorColorCategories {
    Default = 0,
    Operator = 1,
    Number = 2,
    String = 3,
    Comment = 4,
    Keyword = 5,
    Builtin = 6,
    Method = 7,
    Property = 8,
    Nil = 9,
    Bool = 10,
    Function = 11,
    Local = 12,
    Self_ = 13,
    LuauKeyword = 14,
    FunctionName = 15,
    TODO = 16,
    Background = 17,
    SelectionText = 18,
    SelectionBackground = 19,
    FindSelectionBackground = 20,
    MatchingWordBackground = 21,
    Warning = 22,
    Error = 23,
    Info = 24,
    Hint = 25,
    Whitespace = 26,
    ActiveLine = 27,
    DebuggerCurrentLine = 28,
    DebuggerErrorLine = 29,
    Ruler = 30,
    Bracket = 31,
    Type = 32,
    MenuPrimaryText = 33,
    MenuSecondaryText = 34,
    MenuSelectedText = 35,
    MenuBackground = 36,
    MenuSelectedBackground = 37,
    MenuScrollbarBackground = 38,
    MenuScrollbarHandle = 39,
    MenuBorder = 40,
    DocViewCodeBackground = 41,
    AICOOverlayText = 42,
    AICOOverlayButtonBackground = 43,
    AICOOverlayButtonBackgroundHover = 44,
    AICOOverlayButtonBackgroundPressed = 45,
    IndentationRuler = 46,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum StudioScriptEditorColorPresets {
    RobloxDefault = 0,
    Extra1 = 1,
    Extra2 = 2,
    Custom = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum StudioStyleGuideColor {
    MainBackground = 0,
    Titlebar = 1,
    Dropdown = 2,
    Tooltip = 3,
    Notification = 4,
    ScrollBar = 5,
    ScrollBarBackground = 6,
    TabBar = 7,
    Tab = 8,
    FilterButtonDefault = 9,
    FilterButtonHover = 10,
    FilterButtonChecked = 11,
    FilterButtonAccent = 12,
    FilterButtonBorder = 13,
    FilterButtonBorderAlt = 14,
    RibbonTab = 15,
    RibbonTabTopBar = 16,
    Button = 17,
    MainButton = 18,
    RibbonButton = 19,
    ViewPortBackground = 20,
    InputFieldBackground = 21,
    Item = 22,
    TableItem = 23,
    CategoryItem = 24,
    GameSettingsTableItem = 25,
    GameSettingsTooltip = 26,
    EmulatorBar = 27,
    EmulatorDropDown = 28,
    ColorPickerFrame = 29,
    CurrentMarker = 30,
    Border = 31,
    DropShadow = 32,
    Shadow = 33,
    Light = 34,
    Dark = 35,
    Mid = 36,
    MainText = 37,
    SubText = 38,
    TitlebarText = 39,
    BrightText = 40,
    DimmedText = 41,
    LinkText = 42,
    WarningText = 43,
    ErrorText = 44,
    InfoText = 45,
    SensitiveText = 46,
    ScriptSideWidget = 47,
    ScriptBackground = 48,
    ScriptText = 49,
    ScriptSelectionText = 50,
    ScriptSelectionBackground = 51,
    ScriptFindSelectionBackground = 52,
    ScriptMatchingWordSelectionBackground = 53,
    ScriptOperator = 54,
    ScriptNumber = 55,
    ScriptString = 56,
    ScriptComment = 57,
    ScriptKeyword = 58,
    ScriptBuiltInFunction = 59,
    ScriptWarning = 60,
    ScriptError = 61,
    ScriptInformation = 62,
    ScriptHint = 63,
    ScriptWhitespace = 64,
    ScriptRuler = 65,
    DocViewCodeBackground = 66,
    DebuggerCurrentLine = 67,
    DebuggerErrorLine = 68,
    DiffFilePathText = 69,
    DiffTextHunkInfo = 70,
    DiffTextNoChange = 71,
    DiffTextAddition = 72,
    DiffTextDeletion = 73,
    DiffTextSeparatorBackground = 74,
    DiffTextNoChangeBackground = 75,
    DiffTextAdditionBackground = 76,
    DiffTextDeletionBackground = 77,
    DiffLineNum = 78,
    DiffLineNumSeparatorBackground = 79,
    DiffLineNumNoChangeBackground = 80,
    DiffLineNumAdditionBackground = 81,
    DiffLineNumDeletionBackground = 82,
    DiffFilePathBackground = 83,
    DiffFilePathBorder = 84,
    ChatIncomingBgColor = 85,
    ChatIncomingTextColor = 86,
    ChatOutgoingBgColor = 87,
    ChatOutgoingTextColor = 88,
    ChatModeratedMessageColor = 89,
    Separator = 90,
    ButtonBorder = 91,
    ButtonText = 92,
    InputFieldBorder = 93,
    CheckedFieldBackground = 94,
    CheckedFieldBorder = 95,
    CheckedFieldIndicator = 96,
    HeaderSection = 97,
    Midlight = 98,
    StatusBar = 99,
    DialogButton = 100,
    DialogButtonText = 101,
    DialogButtonBorder = 102,
    DialogMainButton = 103,
    DialogMainButtonText = 104,
    InfoBarWarningBackground = 105,
    InfoBarWarningText = 106,
    ScriptEditorCurrentLine = 107,
    ScriptMethod = 108,
    ScriptProperty = 109,
    ScriptNil = 110,
    ScriptBool = 111,
    ScriptFunction = 112,
    ScriptLocal = 113,
    ScriptSelf = 114,
    ScriptLuauKeyword = 115,
    ScriptFunctionName = 116,
    ScriptTodo = 117,
    ScriptBracket = 118,
    AttributeCog = 119,
    AICOOverlayText = 128,
    AICOOverlayButtonBackground = 129,
    AICOOverlayButtonBackgroundHover = 130,
    AICOOverlayButtonBackgroundPressed = 131,
    OnboardingCover = 132,
    OnboardingHighlight = 133,
    OnboardingShadow = 134,
    BreakpointMarker = 136,
    DiffLineNumHover = 137,
    DiffLineNumSeparatorBackgroundHover = 138,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum StudioStyleGuideModifier {
    Default = 0,
    Selected = 1,
    Pressed = 2,
    Disabled = 3,
    Hover = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum Style {
    AlternatingSupports = 0,
    BridgeStyleSupports = 1,
    NoSupports = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SubscriptionExpirationReason {
    ProductInactive = 0,
    ProductDeleted = 1,
    SubscriberCancelled = 2,
    SubscriberRefunded = 3,
    Lapsed = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SubscriptionPaymentStatus {
    Paid = 0,
    Refunded = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SubscriptionPeriod {
    Month = 0,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SubscriptionState {
    NeverSubscribed = 0,
    SubscribedWillRenew = 1,
    SubscribedWillNotRenew = 2,
    SubscribedRenewalPaymentPending = 3,
    Expired = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SurfaceConstraint {
    None = 0,
    Hinge = 1,
    SteppingMotor = 2,
    Motor = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SurfaceGuiShape {
    Flat = 0,
    CurvedHorizontally = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SurfaceGuiSizingMode {
    FixedSize = 0,
    PixelsPerStud = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SurfaceType {
    Smooth = 0,
    Glue = 1,
    Weld = 2,
    Studs = 3,
    Inlet = 4,
    Universal = 5,
    Hinge = 6,
    Motor = 7,
    SteppingMotor = 8,
    SmoothNoOutlines = 10,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SwipeDirection {
    Right = 0,
    Left = 1,
    Up = 2,
    Down = 3,
    None = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum SystemThemeValue {
    error = 0,
    light = 1,
    dark = 2,
    systemLight = 3,
    systemDark = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TableMajorAxis {
    RowMajor = 0,
    ColumnMajor = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TeamCreateErrorState {
    PlaceSizeTooLarge = 0,
    PlaceSizeApproachingLimit = 1,
    NoError = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum Technology {
    Legacy = 0,
    Voxel = 1,
    Compatibility = 2,
    ShadowMap = 3,
    Future = 4,
    Unified = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TeleportMethod {
    TeleportToSpawnByName = 0,
    TeleportToPlaceInstance = 1,
    TeleportToPrivateServer = 2,
    TeleportPartyAsync = 3,
    TeleportToVIPServer = 4,
    TeleportToInstanceBack = 5,
    TeleportUnknown = 6,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TeleportResult {
    Success = 0,
    Failure = 1,
    GameNotFound = 2,
    GameEnded = 3,
    GameFull = 4,
    Unauthorized = 5,
    Flooded = 6,
    IsTeleporting = 7,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TeleportState {
    RequestedFromServer = 0,
    Started = 1,
    WaitingForServer = 2,
    Failed = 3,
    InProgress = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TeleportType {
    ToPlace = 0,
    ToInstance = 1,
    ToReservedServer = 2,
    ToVIPServer = 3,
    ToInstanceBack = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TerrainAcquisitionMethod {
    None = 0,
    Legacy = 1,
    Template = 2,
    Generate = 3,
    Import = 4,
    Convert = 5,
    EditAddTool = 6,
    EditSeaLevelTool = 7,
    EditReplaceTool = 8,
    RegionFillTool = 9,
    RegionPasteTool = 10,
    Other = 11,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TerrainFace {
    Top = 0,
    Side = 1,
    Bottom = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TextChatMessageStatus {
    Unknown = 1,
    Success = 2,
    Sending = 3,
    TextFilterFailed = 4,
    Floodchecked = 5,
    InvalidPrivacySettings = 6,
    InvalidTextChannelPermissions = 7,
    MessageTooLong = 8,
    ModerationTimeout = 9,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TextDirection {
    Auto = 0,
    LeftToRight = 1,
    RightToLeft = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TextFilterContext {
    PublicChat = 1,
    PrivateChat = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TextInputType {
    Default = 0,
    NoSuggestions = 1,
    Number = 2,
    Email = 3,
    Phone = 4,
    Password = 5,
    PasswordShown = 6,
    Username = 7,
    OneTimePassword = 8,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TextTruncate {
    None = 0,
    AtEnd = 1,
    SplitWord = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TextXAlignment {
    Left = 0,
    Right = 1,
    Center = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TextYAlignment {
    Top = 0,
    Center = 1,
    Bottom = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TextureMode {
    Stretch = 0,
    Wrap = 1,
    Static = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TextureQueryType {
    NonHumanoid = 0,
    NonHumanoidOrphaned = 1,
    Humanoid = 2,
    HumanoidOrphaned = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ThreadPoolConfig {
    Auto = 0,
    Threads1 = 1,
    Threads2 = 2,
    Threads3 = 3,
    Threads4 = 4,
    Threads8 = 8,
    Threads16 = 16,
    PerCore1 = 101,
    PerCore2 = 102,
    PerCore3 = 103,
    PerCore4 = 104,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ThrottlingPriority {
    Default = 0,
    ElevatedOnServer = 1,
    Extreme = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ThumbnailSize {
    Size48x48 = 0,
    Size180x180 = 1,
    Size420x420 = 2,
    Size60x60 = 3,
    Size100x100 = 4,
    Size150x150 = 5,
    Size352x352 = 6,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ThumbnailType {
    HeadShot = 0,
    AvatarBust = 1,
    AvatarThumbnail = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TickCountSampleMethod {
    Fast = 0,
    Benchmark = 1,
    Precise = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TonemapperPreset {
    Default = 0,
    Retro = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TopBottom {
    Top = 0,
    Center = 1,
    Bottom = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TouchCameraMovementMode {
    Default = 0,
    Classic = 1,
    Follow = 2,
    Orbital = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TouchMovementMode {
    Default = 0,
    Thumbstick = 1,
    DPad = 2,
    Thumbpad = 3,
    ClickToMove = 4,
    DynamicThumbstick = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TrackerError {
    Ok = 0,
    NoService = 1,
    InitFailed = 2,
    NoVideo = 3,
    VideoError = 4,
    VideoNoPermission = 5,
    VideoUnsupported = 6,
    NoAudio = 7,
    AudioError = 8,
    AudioNoPermission = 9,
    UnsupportedDevice = 10,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TrackerExtrapolationFlagMode {
    ForceDisabled = 0,
    ExtrapolateFacsAndPose = 1,
    ExtrapolateFacsOnly = 2,
    Auto = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TrackerFaceTrackingStatus {
    FaceTrackingSuccess = 0,
    FaceTrackingNoFaceFound = 1,
    FaceTrackingUnknown = 2,
    FaceTrackingLost = 3,
    FaceTrackingHasTrackingError = 4,
    FaceTrackingIsOccluded = 5,
    FaceTrackingUninitialized = 6,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TrackerLodFlagMode {
    ForceFalse = 0,
    ForceTrue = 1,
    Auto = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TrackerLodValueMode {
    Force0 = 0,
    Force1 = 1,
    Auto = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TrackerMode {
    None = 0,
    Audio = 1,
    Video = 2,
    AudioVideo = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TrackerPromptEvent {
    LODCameraRecommendDisable = 0,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TrackerType {
    None = 0,
    Face = 1,
    UpperBody = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TriStateBoolean {
    Unknown = 0,
    True = 1,
    False = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum TweenStatus {
    Canceled = 0,
    Completed = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum UICaptureMode {
    All = 0,
    None = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum UIDragDetectorBoundingBehavior {
    Automatic = 0,
    EntireObject = 1,
    HitPoint = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum UIDragDetectorDragRelativity {
    Absolute = 0,
    Relative = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum UIDragDetectorDragSpace {
    Parent = 0,
    LayerCollector = 1,
    Reference = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum UIDragDetectorDragStyle {
    TranslatePlane = 0,
    TranslateLine = 1,
    Rotate = 2,
    Scriptable = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum UIDragDetectorResponseStyle {
    Offset = 0,
    Scale = 1,
    CustomOffset = 2,
    CustomScale = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum UIDragSpeedAxisMapping {
    XY = 0,
    XX = 1,
    YY = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum UIFlexAlignment {
    None = 0,
    Fill = 1,
    SpaceAround = 2,
    SpaceBetween = 3,
    SpaceEvenly = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum UIFlexMode {
    None = 0,
    Grow = 1,
    Shrink = 2,
    Fill = 3,
    Custom = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum UITheme {
    Light = 0,
    Dark = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum UiMessageType {
    UiMessageError = 0,
    UiMessageInfo = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum UsageContext {
    Default = 0,
    Preview = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum UserCFrame {
    Head = 0,
    LeftHand = 1,
    RightHand = 2,
    Floor = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum UserInputState {
    Begin = 0,
    Change = 1,
    End = 2,
    Cancel = 3,
    None = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum UserInputType {
    MouseButton1 = 0,
    MouseButton2 = 1,
    MouseButton3 = 2,
    MouseWheel = 3,
    MouseMovement = 4,
    Touch = 7,
    Keyboard = 8,
    Focus = 9,
    Accelerometer = 10,
    Gyro = 11,
    Gamepad1 = 12,
    Gamepad2 = 13,
    Gamepad3 = 14,
    Gamepad4 = 15,
    Gamepad5 = 16,
    Gamepad6 = 17,
    Gamepad7 = 18,
    Gamepad8 = 19,
    TextInput = 20,
    InputMethod = 21,
    None = 22,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum VRComfortSetting {
    Comfort = 0,
    Normal = 1,
    Expert = 2,
    Custom = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum VRControllerModelMode {
    Disabled = 0,
    Transparent = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum VRDeviceType {
    Unknown = 0,
    OculusRift = 1,
    HTCVive = 2,
    ValveIndex = 3,
    OculusQuest = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum VRLaserPointerMode {
    Disabled = 0,
    Pointer = 1,
    DualPointer = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum VRSafetyBubbleMode {
    NoOne = 0,
    OnlyFriends = 1,
    Anyone = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum VRScaling {
    World = 0,
    Off = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum VRSessionState {
    Undefined = 0,
    Idle = 1,
    Visible = 2,
    Focused = 3,
    Stopping = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum VRTouchpad {
    Left = 0,
    Right = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum VRTouchpadMode {
    Touch = 0,
    VirtualThumbstick = 1,
    ABXY = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum VelocityConstraintMode {
    Line = 0,
    Plane = 1,
    Vector = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum VerticalAlignment {
    Center = 0,
    Top = 1,
    Bottom = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum VerticalScrollBarPosition {
    Right = 0,
    Left = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum VibrationMotor {
    Large = 0,
    Small = 1,
    LeftTrigger = 2,
    RightTrigger = 3,
    LeftHand = 4,
    RightHand = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum VideoDeviceCaptureQuality {
    Default = 0,
    Low = 1,
    Medium = 2,
    High = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum VideoError {
    Ok = 0,
    Eof = 1,
    EAgain = 2,
    BadParameter = 3,
    AllocFailed = 4,
    CodecInitFailed = 5,
    CodecCloseFailed = 6,
    DecodeFailed = 7,
    ParsingFailed = 8,
    Unsupported = 9,
    Generic = 10,
    DownloadFailed = 11,
    StreamNotFound = 12,
    EncodeFailed = 13,
    CreateFailed = 14,
    NoPermission = 15,
    NoService = 16,
    ReleaseFailed = 17,
    Unknown = 18,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ViewMode {
    None = 0,
    GeometryComplexity = 1,
    Transparent = 2,
    Decal = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum VirtualCursorMode {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum VirtualInputMode {
    None = 0,
    Recording = 1,
    Playing = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum VoiceChatDistanceAttenuationType {
    Inverse = 0,
    Legacy = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum VoiceChatState {
    Idle = 0,
    Joining = 1,
    JoiningRetry = 2,
    Joined = 3,
    Leaving = 4,
    Ended = 5,
    Failed = 6,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum VoiceControlPath {
    Publish = 0,
    Subscribe = 1,
    Join = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum VolumetricAudio {
    Disabled = 0,
    Automatic = 1,
    Enabled = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum WaterDirection {
    NegX = 0,
    X = 1,
    NegY = 2,
    Y = 3,
    NegZ = 4,
    Z = 5,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum WaterForce {
    None = 0,
    Small = 1,
    Medium = 2,
    Strong = 3,
    Max = 4,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum WebSocketState {
    Connecting = 0,
    Open = 1,
    Closing = 2,
    Closed = 3,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum WeldConstraintPreserve {
    All = 0,
    None = 1,
    Touching = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum WhisperChatPrivacyMode {
    AllUsers = 0,
    NoOne = 1,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum WrapLayerAutoSkin {
    Disabled = 0,
    EnabledPreserve = 1,
    EnabledOverride = 2,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum WrapLayerDebugMode {
    None = 0,
    BoundCage = 1,
    LayerCage = 2,
    BoundCageAndLinks = 3,
    Reference = 4,
    Rbf = 5,
    OuterCage = 6,
    ReferenceMeshAfterMorph = 7,
    HSROuterDetail = 8,
    HSROuter = 9,
    HSRInner = 10,
    HSRInnerReverse = 11,
    LayerCageFittedToBase = 12,
    LayerCageFittedToPrev = 13,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum WrapTargetDebugMode {
    None = 0,
    TargetCageOriginal = 1,
    TargetCageCompressed = 2,
    TargetCageInterface = 3,
    TargetLayerCageOriginal = 4,
    TargetLayerCageCompressed = 5,
    TargetLayerInterface = 6,
    Rbf = 7,
    OuterCageDetail = 8,
}
#[derive(Debug, Clone)]
#[allow(nonstandard_style)]
pub enum ZIndexBehavior {
    Global = 0,
    Sibling = 1,
}
