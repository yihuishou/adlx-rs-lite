#include <ADLX.h>

// 基础定义
#include <ADLXDefines.h>
#include <ADLXStructures.h>
#include <ADLXVersion.h>

// 系统接口（含 1.5 新增的 ISystem3.h：IADLXSystem3 / IADLXGPU3 / VariableGraphicsMemory）
#include <ISystem.h>
#include <ISystem1.h>
#include <ISystem2.h>
#include <ISystem3.h>

// 集合、事件、应用
#include <ICollections.h>
#include <IChangedEvent.h>
#include <IApplications.h>

// 性能监控（1.5 新增 IPerformanceMonitoring2.h / IPerformanceMonitoring3.h）
#include <IPerformanceMonitoring.h>
#include <IPerformanceMonitoring1.h>
#include <IPerformanceMonitoring2.h>
#include <IPerformanceMonitoring3.h>

// 3D 设置（1.5 新增 I3DSettings3.h）
#include <I3DSettings.h>
#include <I3DSettings1.h>
#include <I3DSettings2.h>
#include <I3DSettings3.h>

// 显示（1.5 新增 IDisplays1.h / IDisplays2.h / IDisplays3.h）
#include <IDisplays.h>
#include <IDisplays1.h>
#include <IDisplays2.h>
#include <IDisplays3.h>
#include <IDisplaySettings.h>
#include <IDisplay3DLUT.h>
#include <IDisplayGamma.h>
#include <IDisplayGamut.h>
#include <IDesktops.h>

// 调优
#include <IGPUTuning.h>
#include <IGPUTuning1.h>
#include <IGPUAutoTuning.h>
#include <IGPUManualFanTuning.h>
#include <IGPUManualGFXTuning.h>
#include <IGPUManualPowerTuning.h>
#include <IGPUManualVRAMTuning.h>
#include <IGPUPresetTuning.h>
#include <IPowerTuning.h>
#include <IPowerTuning1.h>
#include <ISmartAccessMemory.h>

// 其他
#include <II2C.h>
#include <ILog.h>
#include <IMultiMedia.h>
