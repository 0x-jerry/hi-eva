import dayjs from 'dayjs'
import { type BaseModel, BaseModelManager, COMMON_COLUMN } from './database'

export interface ISelectionModel extends BaseModel {
  selected: string
  promptName: string
}

class SelectionTable extends BaseModelManager<ISelectionModel> {
  TABLE_NAME = 'selection'
  COLUMN_NAMES = ['selected', 'promptName']

  async groupDataBySelectedText(opt: {
    start: dayjs.ConfigType
    end: dayjs.ConfigType
  }) {
    type IGroupedSelectionModel = ISelectionModel & { count: number }

    const sql = `select *, count(selected) as count 
        from ${this.TABLE_NAME} 
        where ${COMMON_COLUMN.createdDate} between $1 and $2 
        group by selected
      `

    const values = [
      //
      dayjs(opt.start).unix(),
      dayjs(opt.end).unix(),
    ]

    const datas = await this.db.select<IGroupedSelectionModel[]>(sql, values)

    return datas
  }
}

export const selectionTable = new SelectionTable()
