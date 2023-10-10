<script setup lang="ts">
  import {Ref, ref} from 'vue';

  let props = defineProps({
    text: {type: String, required: true},
    date: {type: Date, required: true},
  });

  let text = props.text;

  let dialog: Ref<boolean> = ref<boolean>(false);
  const oldDate = new Date(props.date);
  let now = new Date(props.date);

  let dateTime: Ref<string> = ref<string>(now.toISOString().split('T')[0]);
  let hours: Ref<number> = ref<number>(now.getHours());
  let minutes: Ref<number> = ref<number>(now.getMinutes());

  useIntervalFn(log, 5 * 1000);

  function log() {
    console.log("Date: " + dateTime.value + ", H: " + hours.value + ":" + minutes.value);
  }
  function pickDateTime() {
    let valid = true;
    if (dateTime.value.length <= 0) {
      valid = false;
    } else if (hours.value < 0 && hours.value >= 24) {
      valid = false;
    } else if (minutes.value < 0 && minutes.value >= 60) {
      valid = false;
    }

    if (valid) {
      console.log("Valid date time.");
      let date = new Date(dateTime.value);
      date.setHours(hours.value, minutes.value, 0, 0);

      // TODO: Use store for start and end date
      //props.date = date;
    } else {
      console.log("Invalid date time.");
    }

    closeDialog();
  }

  function closeDialog() {
    // TODO: Add values

    dialog.value = false;
  }
</script>

<template>
  <v-dialog
      v-model="dialog"
      width="800px"
  >
    <template v-slot:activator="{ props }">
      <v-btn
          block
          color="primary"
          v-bind="props"
      >
        {{ text }}: {{date.getFullYear()}}.{{date.getMonth().toString().padStart(2, '0')}}.{{date.getDay().toString().padStart(2, '0')}} - {{hours.toString().padStart(2, '0')}}:{{minutes.toString().padStart(2, '0')}}
      </v-btn>
    </template>

    <v-card>
      <v-card-title>Kategorie hinzufügen</v-card-title>

      <v-divider/>

      <v-card-text>
        <v-row>
          <v-text-field
            v-model="dateTime"
            type="date"
            label="Date"
          ></v-text-field>
          <v-text-field
            v-model="hours"
            type="number"
            label="Stunde"
            suffix="H"
          ></v-text-field>
          <v-text-field
            v-model="minutes"
            type="number"
            label="Minute"
            suffix="M"
          ></v-text-field>
        </v-row>
      </v-card-text>

      <v-divider/>

      <v-card-actions>
        <v-spacer/>
        <v-btn
            @click="closeDialog()"
        >
          Abbrechen
        </v-btn>
        <v-btn
            @click="pickDateTime()"
        >
          Auswählen
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<style scoped>

</style>