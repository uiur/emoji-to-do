import { Link } from "react-router-dom";

export function NavigationHeader() {
  return (
    <header className="container mx-auto h-8 flex flex-row items-center space-x-4">
      <Link to='/'>Emojis</Link>
      <Link to='/settings'>Settings</Link>
    </header>
  )
}
