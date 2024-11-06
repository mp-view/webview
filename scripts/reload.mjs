import { readFileSync, writeFileSync } from 'node:fs'

const targetFilePath = './ui/index.html'

try {
  const styleRow = readFileSync(targetFilePath, 'utf8')
  writeFileSync(targetFilePath, styleRow)
  console.log('File saved!')
}
catch (err) {
  console.error('Error:', err)
}
