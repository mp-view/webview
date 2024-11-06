// @unocss-includes

import { setupCounter } from './counter.ts'
import 'uno.css'
import './style.css'

document.querySelector<HTMLDivElement>('#app')!.innerHTML = /* html */`
  <div class="bg-zinc-4/80">
    <h1>Vite + TypeScript</h1>
    <div class="card">
      <button id="counter" type="button"></button>
    </div>
    <p class="read-the-docs c-red">
      Click on the Vite and TypeScript logos to learn more
    </p>
  </div>
`
setupCounter(document.querySelector<HTMLButtonElement>('#counter')!)
