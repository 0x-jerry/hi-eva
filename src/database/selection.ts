import { type BaseModel, BaseModelManager } from './database'

interface Selection extends BaseModel {
  selected: string
}

class SelectionTable extends BaseModelManager<Selection> {
  TABLE_NAME = 'selection'
  COLUMN_NAMES = ['selected'] 
}

export const selectionTable = new SelectionTable()
