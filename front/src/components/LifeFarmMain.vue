<template>
    <div class="lifefarm-container">
        <h1>LifeFarm</h1>
        <div class="grass-container">
            <div v-for="(year, yearIdx) in grass" :key="yearIdx" class="grass-row">
                <p class="year-cell">{{ birth_year + yearIdx * year_gap }} - {{ birth_year + (yearIdx + 1) * year_gap - 1 }}</p>
                <div 
                    v-for="(cell_unit, dayIdx) in year" 
                    :key="dayIdx" 
                    class="grass-cell"
                    :style="{ backgroundColor: (birth_year + yearIdx * year_gap) <= cur_year && cur_year <= (birth_year + (yearIdx + 1) * year_gap - 1) ?
                                 `rgba(255, 0, 0, ${cell_unit})` : `rgba(0, 255, 0, ${cell_unit})` }">
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import { ref } from 'vue';



export default {
    name: 'LifeFarmMain',
    setup() {
        const years = 80;
        const cell_unit = 52; // per day - 365, per week - 52, per month - 12
        const year_gap = 2;
        const cur_year = 2025;

        const pivot = cell_unit * year_gap;
        const cols = pivot;
        let rows = (years * cell_unit) / pivot;
        if ((years * cell_unit) - (rows * pivot) > 0) {
            rows += 1;
        }

        const grass = ref(
        Array.from({ length: rows }, () => 
            Array.from({ length: cols }, () => Math.random())
        ));

        const birth_year = 1992;

        return {
            grass,
            birth_year,
            year_gap,
            cur_year
        }
    }
}
</script>

<style scoped>
.lifefarm-container {
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