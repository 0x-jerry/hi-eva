import { type BaseModel, BaseModelManager } from './database'
import type { GetPureModelType } from './types'

export interface IEndpointConfigModel extends BaseModel {
  isBuiltin?: number
  name?: string
  baseUrl?: string
  apiKey?: string
  model?: string
}

export type IEndpointConfigItem = GetPureModelType<IEndpointConfigModel>

class EndpointConfigTable extends BaseModelManager<IEndpointConfigModel> {
  TABLE_NAME = 'endpoint_config'
  COLUMN_NAMES = ['name', 'baseUrl', 'apiKey', 'model', 'isBuiltin']
}

export const endpointConfigTable = new EndpointConfigTable()
