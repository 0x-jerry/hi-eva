import dayjs from 'dayjs'
import { type BaseModel, BaseModelManager } from './database'

interface ISelectionModel extends BaseModel {
  selected: string
  promptName: string
}

class SelectionTable extends BaseModelManager<ISelectionModel> {
  TABLE_NAME = 'selection'
  COLUMN_NAMES = ['selected', 'count', 'promptName']

  async addCountForSelected(selected: string, promptName: string) {
    const datas = await this.db.select<ISelectionModel[]>(
      `select * from ${this.TABLE_NAME} where selected = $1 and prompt_name = $2`,
      [selected, promptName],
    )

    const data = datas.at(0)

    if (!data) {
      return await this.createOne({
        promptName,
        selected,
      })
    }

    await this.updateOne(data)

    return data
  }

  async groupDataBySelectedText(opt: {
    start: dayjs.ConfigType
    end: dayjs.ConfigType
  }) {
    type IGroupedSelectionModel = ISelectionModel & { count: number }

    const datas = await this.db.select<IGroupedSelectionModel[]>(
      `select *, count(selected) as count 
        from ${this.TABLE_NAME} 
        where created_date between $1 and $2 
        group by selected
      `,
      [dayjs(opt.start).unix(), dayjs(opt.end).unix()],
    )

    return datas
  }
}

export const selectionTable = new SelectionTable()
