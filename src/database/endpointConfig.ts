import { type BaseModel, BaseModelManager } from './database'

export interface IEndpointConfigModel extends BaseModel {
  isBuiltin: number
  name: string
  url: string
  apiKey: string
  /**
   * json value for string array
   */
  models: string
}

class EndpointConfigTable extends BaseModelManager<IEndpointConfigModel> {
  TABLE_NAME = 'endpoint_config'
  COLUMN_NAMES = ['name', 'url', 'apiKey', 'models', 'isBuiltin']
}

export const promptConfigTable = new EndpointConfigTable()
