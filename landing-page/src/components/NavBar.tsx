import Link from "next/link";

export default function NavBar() {
  return (
    <div className='bg-white/10 fixed xl:py-4 w-full z-30'>
      <div className='container mx-auto px-3 flex items-center justify-between '>
        <h1 className='uppercase text-2xl text-white font-bold tracking-widest'>
          SolClan
        </h1>
        <nav>
          <ul className='justify-center space-x-10 2xl:flex invisible 2xl:visible'>
            <Link href='/'>
              <li className='text-white font-bold tracking-widest text-md'>
                About us
              </li>
            </Link>
            <Link href='/'>
              <li className='text-white font-bold tracking-widest text-md'>
                USP
              </li>
            </Link>
            <Link href='/'>
              <li className='text-white font-bold tracking-widest text-md'>
                Onboarding
              </li>
            </Link>
          </ul>
        </nav>
        <div>
          <a
            href={process.env.NEXT_PUBLIC_APP_URL}
            target='_blank'
            className='flex whitespace-nowrap border-2 border-white text-white tracking-widest text-sm px-4 py-2 rounded-full hover:bg-white/20 transition-colors duration-300'
          >
            Download app
          </a>
        </div>
      </div>
    </div>
  );
}
