// @unocss-include

import type { VersionedData } from '@0x-jerry/utils'
import { type UseConfigOption, useConfig } from './useConfig'

export interface PromptConfig {
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
  endpointId?: string

  /**
   * Model from endpoint config
   */
  model?: string
}

export interface PromptsConfigV1 extends VersionedData {
  version: 1
  items: PromptConfig[]
}

export type PromptsConfigLatest = PromptsConfigV1

const builtinPromptConfigs: PromptConfig[] = [
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

export function usePromptsConfig(
  option?: UseConfigOption<PromptsConfigLatest>,
) {
  const defaultConfig: PromptsConfigLatest = {
    version: 1,
    items: builtinPromptConfigs,
  }

  return useConfig<PromptsConfigV1>('prompts.json', 'data', defaultConfig, {
    ...option,
    migrations: [
      {
        version: 1,
        upgrade(): PromptsConfigV1 {
          return defaultConfig
        },
      },
    ],
  })
}
