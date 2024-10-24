<script lang="ts">
	
	interface Props {
		// biome-ignore lint/suspicious/noExplicitAny: <explanation>
		messages: any[];
		isConnected: boolean;
	}

	let { messages, isConnected }: Props = $props();
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
