import { render } from 'preact'
import { App } from './app.tsx'
import '@unocss/reset/tailwind.css'
import 'uno.css'
import './style.css'

render(<App />, document.getElementById('app')!)
