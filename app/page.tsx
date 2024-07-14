export default function Home() {
  return (
    <main className="flex flex-col items-center justify-between h-full p-24 bg-gray-800">
        <header className="text-4xl font-bold mb-8">Welcome to Puzzle Solver!</header>
        <section className="flex flex-row items-center justify-around text-2xl mb-8">
          <img src="logo.png" alt="Logo" className="w-64 h-64 mb-8" />
          <div className="flex flex-col gap-6 w-1/2">
            <p>
              Are you ready to solve some puzzles?
            </p>
            <p>
              Solve Sudoku and Nonograms, and share your creations with others!
            </p>
          </div>
        </section>
        <a href="/create">
          <button className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-4 px-8 rounded-lg">
            Get Started
          </button>
        </a>
    </main>
  );
}
