import { type BaseModel, BaseModelManager } from './database'

interface ISelectionModel extends BaseModel {
  selected: string
  count: number
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
        count: 1,
      })
    }

    data.count += 1
    await this.updateOne(data)

    return data
  }
}

export const selectionTable = new SelectionTable()
