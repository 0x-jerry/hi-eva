// @unocss-include
import { useLocalStorage } from '@vueuse/core'

export interface ToolbarPromptConfig {
  id: string
  name: string
  builtin?: boolean

  /**
   * Prompt template, support use variable
   */
  prompt: string

  /**
   * https://icones.js.org/collection/carbon
   *
   * Format is: ${name}
   * Example: ibm-watson-language-translator
   */
  icon?: string

  /**
   * ID for endpoint
   */
  enpointId?: string

  /**
   * Model from endpoint config
   */
  model?: string
}

const builtinToolbarConfigs: ToolbarPromptConfig[] = [
  {
    id: 'translate',
    builtin: true,
    name: '翻译',
    icon: 'ibm-watson-language-translator',
    prompt: `结合语境翻译以下文本：\n'''\n{{selection}}\n'''`,
  },
  {
    id: 'dictionary',
    builtin: true,
    name: '字典',
    icon: 'book',
    prompt: `请用中文回复我以下单词的音标或者发音、词性、意思以及示例\n'''\n{{selection}}\n'''`,
  },
]

export const promptConfigs = useLocalStorage<ToolbarPromptConfig[]>(
  'prompt-configs',
  builtinToolbarConfigs,
)

export function getPromptConf(id: string) {
  return promptConfigs.value.find((n) => n.id === id)
}
