<template>
    <div class="blueprint-container">
        <h1>BluePrint</h1>
        <div class="grass-container">
            <div v-for="(year, yearIdx) in blpt" :key="yearIdx" class="grass-row">
                <p class="year-cell">{{ start_date + yearIdx * gap }} - {{ start_date + (yearIdx + 1) * gap - 1 }}</p>
                <div 
                    v-for="(cell_unit, dayIdx) in year" 
                    :key="dayIdx" 
                    class="grass-cell"
                    :style="{ backgroundColor: (start_date + yearIdx * gap) <= cur_day && cur_day <= (start_date + (yearIdx + 1) * gap - 1) ?
                                 `rgba(255, 0, 0, ${cell_unit})` : `rgba(0, 255, 0, ${cell_unit})` }">
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import { ref } from 'vue';



export default {
    name: 'BluePrintMain',
    setup() {
        const weeks = 4;
        const cell_unit = 7;
        const gap = 2;
        const cur_day = 2025;

        const pivot = cell_unit * gap;
        const cols = pivot;
        let rows = (weeks * cell_unit) / pivot;
        if ((weeks * cell_unit) - (rows * pivot) > 0) {
            rows += 1;
        }

        const blpt = ref(
        Array.from({ length: rows }, () => 
            Array.from({ length: cols }, () => Math.random())
        ));

        const start_date = 0;

        return {
            blpt,
            start_date,
            gap,
            cur_day
        }
    }
}
</script>

<style scoped>
.blueprint-container {
    display: flex;
    flex-direction: column;
    align-items: center;
}

.grid-container {
    display: flex;
    flex-direction: row;
    width: 100%;
}

.year-container {
    display: flex;
    flex-direction: column;
    align-items: center;
}

.year-row {
    background-color:aqua;
    font-size: 15px;
    margin: 0px;
    padding: 0px;
}

.year-cell {
    padding-right: 10px;
}

.grass-container {
  display: flex;
  flex-direction: column;
  padding: 5px;
}
.grass-row {
  display: flex;
}
.grass-cell {
  width: 9px;
  height: 9px;
  margin: 2px;
  background-color: brown;
}
</style>