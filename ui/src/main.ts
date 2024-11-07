// @unocss-includes

import { TitleBar } from './components/TitleBar.ts'
import '@unocss/reset/tailwind.css'
import 'uno.css'
import './style.css'

document.querySelector<HTMLDivElement>('#app')!.innerHTML = /* html */`
  <div id="title-bar"></div>
`
TitleBar(document.querySelector<HTMLDivElement>('#title-bar')!)
