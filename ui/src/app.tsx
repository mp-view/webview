import { DeviceInfo } from './components/deviceInfo.tsx'
import { Menu } from './components/menu.tsx'
import { TrafficLight } from './components/trafficLight.tsx'
import { phoneInfoData } from './constants/phoneInfo.ts'
import { USER_EVENT } from './constants/USER_EVENT.ts'

export function App() {
  return (
    <div
      class="flex items-center justify-between py2.5"
      onMouseMove={() => window.ipc.postMessage(USER_EVENT.DRAG_WINDOW)}
    >
      <div class="ml3 flex items-center space-x-6px">
        <TrafficLight />
        <DeviceInfo info={phoneInfoData[0]} />
      </div>
      <Menu />
    </div>
  )
}
