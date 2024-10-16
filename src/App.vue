<template>
  <div class="p-8">
    <h1 class="text-3xl font-bold mb-4">GPU Video Transcoder</h1>

    <!-- Raw Files Folder Selection -->
    <div class="mb-4">
      <label for="raw-folder" class="block mb-2">Select Raw Files Folder</label>
      <button @click="selectRawFolder" class="bg-blue-500 text-white px-4 py-2 rounded">Browse</button>
      <p class="mt-2 text-gray-600">{{ rawFolder }}</p>
    </div>

    <!-- Destination Folder Selection -->
    <div class="mb-4">
      <label for="destination-folder" class="block mb-2">Select Destination Folder</label>
      <button @click="selectDestinationFolder" class="bg-blue-500 text-white px-4 py-2 rounded">Browse</button>
      <p class="mt-2 text-gray-600">{{ destinationFolder }}</p>
    </div>

    <!-- GPU Selection -->
    <div class="mb-4">
      <label for="gpu" class="block mb-2">Select Primary GPU</label>
      <select v-model="gpu" class="w-full border border-gray-300 p-2 rounded">
        <option value="0">GPU 0</option>
        <option value="1">GPU 1</option>
      </select>
    </div>

    <!-- Queue List -->
    <h2 class="text-2xl font-semibold mb-2">Queue</h2>
    <ul class="mb-4">
      <li v-for="file in queue" :key="file" class="mb-2">{{ file }}</li>
    </ul>

    <!-- Start Transcoding Button -->
    <button @click="startTranscoding" class="bg-green-500 text-white px-4 py-2 rounded">Start Transcoding</button>

    <!-- Progress Bars -->
    <div class="mt-6">
      <h3 class="font-semibold">Transcoding Progress</h3>
      <progress class="w-full" :value="transcodeProgress" max="100"></progress>
      <p>{{ transcodeProgress }}%</p>
    </div>

    <div class="mt-6">
      <h3 class="font-semibold">S3 Upload Progress</h3>
      <progress class="w-full" :value="uploadProgress" max="100"></progress>
      <p>{{ uploadProgress }}%</p>
    </div>
  </div>
</template>

<script>
import { invoke } from "@tauri-apps/api"; // Import Tauri API for backend communication

export default {
  data() {
    return {
      rawFolder: "",
      destinationFolder: "",
      gpu: "0", // Default GPU
      queue: [],
      transcodeProgress: 0,
      uploadProgress: 0,
    };
  },
  methods: {
    async selectRawFolder() {
      const folder = await invoke("select_folder");
      this.rawFolder = folder;
    },
    async selectDestinationFolder() {
      const folder = await invoke("select_folder");
      this.destinationFolder = folder;
    },
    async startTranscoding() {
      // Start the transcoding process
      await invoke("start_transcoding", {
        rawFolder: this.rawFolder,
        destinationFolder: this.destinationFolder,
        gpu: this.gpu,
      });

      // Listen for progress updates (if available)
      this.listenToProgress();
    },
    listenToProgress() {
      // Simulate progress updates (replace with real backend events)
      const interval = setInterval(() => {
        if (this.transcodeProgress < 100) {
          this.transcodeProgress += 10; // Update with real progress
        } else {
          clearInterval(interval);
        }
      }, 1000);

      const uploadInterval = setInterval(() => {
        if (this.uploadProgress < 100) {
          this.uploadProgress += 10; // Update with real progress
        } else {
          clearInterval(uploadInterval);
        }
      }, 1000);
    },
  },
};
</script>

<style>
/* Add your Tailwind classes directly in the template */
</style>
