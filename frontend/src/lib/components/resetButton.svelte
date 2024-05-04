<script lang="ts">
	export let handleReset: () => void;
</script>

<button
	on:click={handleReset}
	class="bg-cyan-50 my-4 hover:bg-gray-100 text-gray-900 font-semibold py-2 px-4 border border-gray-400 rounded shadow"
>
	Reset board
</button>
