<script lang="ts">
// biome-ignore lint/suspicious/noExplicitAny: <explanation>
export let messages: any[];
export let isConnected: boolean;
</script>

<div class="p-8">
	<ul>
		{#each messages as message}
			<li>
				Clients: {message.clients_count}, Time: {new Date(
					message.dateTime,
				).toLocaleString()}, API Status: {message.is_up ? 'Up' : 'Down'}
			</li>
		{/each}
	</ul>
	<div>
		Connection status: {isConnected ? 'Connected' : 'Disconnected'}
	</div>
</div>
