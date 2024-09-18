<script lang="ts">
	export let color: 'white' | 'black';
	export let pieces: {
		pawns: number;
		knights: number;
		bishops: number;
		rooks: number;
		queens: number;
		kings: number;
	};
</script>

<div class="hidden md:flex flex-col items-start">
	<h3 class="h3">{color.charAt(0).toUpperCase() + color.slice(1)}</h3>
	<ul>
		<li>Pawns: {pieces.pawns}</li>
		<li>Knights: {pieces.knights}</li>
		<li>Bishops: {pieces.bishops}</li>
		<li>Rooks: {pieces.rooks}</li>
		<li>Queens: {pieces.queens}</li>
		<!-- <li>Kings: {pieces.kings}</li> -->
	</ul>
</div>
