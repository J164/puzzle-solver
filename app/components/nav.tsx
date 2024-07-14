import React from 'react';

export default function NavBar() {
    return (
        <nav className="flex justify-between items-center fixed top-0 left-0 w-full bg-black p-4">
            <a href="/" className="fixed flex items-center w-fit gap-4">
                <img src="logo.png" alt="Website Logo" className="w-10" />
                <h1 className="text-xl font-bold">Puzzle Solver</h1>
            </a>
            <ul className="flex justify-center list-none m-0 p-0 w-full gap-10">
                <li className="mr-2">
                    <a href="/create">Create</a>
                </li>
                <li className="mr-2">
                    <a href="/library">Library</a>
                </li>
                <li>
                    <a href="/community">Community</a>
                </li>
            </ul>
        </nav>
    );
}
