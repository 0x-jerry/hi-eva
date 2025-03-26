import { computed } from 'vue'
import { useEndpointsConfig } from '../../composables'

const _endPointConfigs = useEndpointsConfig()

export const endPointConfigs = computed(() => _endPointConfigs.value.items)

export function getEndpointConf(id: string) {
  return endPointConfigs.value.find((n) => n.id === id)
}
