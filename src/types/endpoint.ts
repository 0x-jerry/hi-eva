import type { IEndpointConfigItem } from '../database/endpointConfig'

export type EndpointItem = Omit<IEndpointConfigItem, 'models'> & {
  models: string[]
}
