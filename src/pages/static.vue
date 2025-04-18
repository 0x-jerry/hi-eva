<script lang='ts' setup>
import Chart from 'primevue/chart'
import { computed, reactive, watch } from 'vue'
import { useAsyncData } from '@0x-jerry/vue-kit'
import { selectionTable } from '../database/selection'
import DatePicker from 'primevue/datepicker'
import dayjs from 'dayjs'

const query = reactive({
  dates: [
    dayjs().add(-7, 'd').startOf('d').toDate(),
    dayjs().endOf('d').toDate(),
  ],
})

const selectionData = useAsyncData(() => {
  if (query.dates.length < 2) return

  const start = dayjs(query.dates[0]).startOf('d')
  const end = dayjs(query.dates[1]).endOf('d')

  return selectionTable.groupDataBySelectedText({ start, end })
}, [])

watch(
  () => query,
  () => {
    selectionData.load()
  },
  {
    deep: true,
  },
)

const chartData = computed(() => {
  const data = selectionData.data.value || []

  return {
    labels: data.map((n) => n.promptName),
    datasets: [
      {
        data: data.map((n) => n.count),
      },
    ],
  }
})

const chartOptions = computed(() => {
  return {
    plugins: {
      legend: {
        labels: {
          usePointStyle: true,
        },
      },
    },
  }
})
</script>

<template>
  <div>
    <div class="query">
      <DatePicker v-model="query.dates" selectionMode="range" :numberOfMonths="2" :manualInput="false" />
    </div>
    <div class="selection">
      <Chart type="pie" :data="chartData" :options="chartOptions" class="w-full" />
    </div>
  </div>
</template>

<style lang='less' scoped></style>
