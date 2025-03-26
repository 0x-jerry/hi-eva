import { computed } from 'vue'
import { usePromptsConfig } from '../../composables'

const _promptConfigs = usePromptsConfig()

export const promptConfigs = computed(() => _promptConfigs.value.items)

export function getPromptConf(id: string) {
  return promptConfigs.value.find((n) => n.id === id)
}
