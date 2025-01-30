<template>
    <div class="farm-container">
        <h1>Life Farm</h1>

        <div v-if="cell_matrix" class="blpt-container">
            <div class="flag-container">
                {{ goal }}
            </div>
            <div class="grass-container">
                <div class="grass-week">
                    <div class="grass-weekday" v-for="weekNum in 7" :key="weekNum">{{ weekdays[weekNum - 1] }}</div>
                </div>
                <div v-for="(week, weekIdx) in cell_matrix" :key="weekIdx" class="grass-row">
                    <div v-for="(day, dayIdx) in week" :key="dayIdx"
                        @mouseover="showTooltip($event, day)"
                        @mouseleave="hideTooltip">
                        <div v-if="day.status == 'Pass'" class="grass-cell" :style="{backgroundColor: `rgba(100, 155, 50, 1)`}"></div>
                        <div v-if="day.status == 'Fail'" class="grass-cell" :style="{backgroundColor: `rgb(255, 0, 0, 1)`}"></div>
                        <div v-if="day.status == 'Padding'" class="grass-cell" :style="{backgroundColor: `rgb(255, 255, 255, 0)`}"></div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Tooltip -->
        <div v-if="tooltip.show" class="tooltip" :style="tooltipStyle" @mouseleave="hideTooltip">
            <p>date: {{ tooltip.date }}</p>
            <p>status: {{ tooltip.status }}</p>
        </div>
    </div>
</template>

<script>
import { ref, onMounted, computed } from 'vue';
import axios from 'axios';

export default {
    name: 'BluePrintMain',
    setup() {
        //// fetch BLPT cell data
        const cell_matrix = ref([]);
        const goal = ref("");
        const desc = ref("");
        const weekdays = ref(['S', 'M', 'T', 'W', 'T', 'F', 'S']);

        const chunk = (array, chunk_size) => {
            const result = [];
            for (let i = 0; i < array.length; i += chunk_size) {
                result.push(array.slice(i, i + chunk_size));
            }
            return result;
        };

        const fetchData = async () => {

            try {
                const response = await axios.get('https://127.0.0.1:30443/front/blpt/1/cell');
                console.log("response: " + JSON.stringify(response.data));

                cell_matrix.value = chunk(response.data.cells, 7);
                goal.value = response.data.blpt.goal;
                desc.value = response.data.blpt.desc;
                
            } catch (err) {
                console.error(err);
            }
        };

        onMounted(fetchData);
        //// fetch end

        //// tooltip
        const tooltip_show = ref(false);
        const tooltip = ref({ show: false, date: '', status: '', x: 0, y: 0 });

        const showTooltip = (event, item) => {
            if (tooltip_show.value == true) { return ; }

            tooltip.value = {
                show: true,
                date: item.date,
                status: item.status,
                x: event.pageX + 10,
                y: event.pageY + 10
            };
        };

        const hideTooltip = () => {
            if (tooltip_show.value == false) { return ; }

            tooltip.value.show = false;
        };

        const tooltipStyle = computed(() => ({
            position: 'absolute',
            left: tooltip.value.x + 'px',
            top: tooltip.value.y + 'px',
            display: tooltip.value.show ? 'block' : 'none'
        }));
        //// tooltip end

        return {
            cell_matrix,
            goal,
            desc,
            weekdays,
            tooltip,
            tooltipStyle,
            showTooltip,
            hideTooltip
        }
    }
}
</script>

<style scoped>
.farm-container {
    display: flex;
    flex-direction: column;
    align-items: center;
}

.flag-container {
    background-color: burlywood;
    padding: 10px;
    margin-bottom: 5px;
}

.blpt-container {
    display: flex;
    flex-direction: column;
    padding: 5px;
}

.grass-week {
    display: flex;
    flex-direction: row;
    justify-content: space-around;
}

.grass-weekday {
    font-size: 2vh;
}

.grass-container {
    border: 2px solid black; /* 두께 5px */
    padding: 5px;
}

.grass-row {
    display: flex;
}
.grass-cell {
    width: 2vh;
    height: 2vh;
    margin: 2px;
    border: 1px;
    cursor: pointer;
}

.tooltip {
    position: absolute;
    background-color: black;
    color: white;
    padding: 8px;
    border-radius: 5px;
    font-size: 14px;
    white-space: nowrap;
    transform: translate(-50%, -100%);
}
</style>