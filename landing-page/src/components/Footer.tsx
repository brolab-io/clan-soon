import Link from "next/link";
import { MdOutlineCopyright } from "react-icons/md";

export default function Footer() {
  return (
    <footer className=' text-white py-10'>
      <div className='container mx-auto px-4'>
        <div className='lg:flex justify-between items-center'>
          <h2 className='font-bold text-3xl tracking-wider uppercase'>
            SolClan
          </h2>
          <ul className='w-full mt-4 lg:flex justify-end space-x-10'>
            <Link
              href='/'
              className='py-5'
            >
              <li>Home</li>
            </Link>
            <Link
              href='/'
              className='py-5'
            >
              <li>About us</li>
            </Link>
            <Link
              href='/'
              className='py-5'
            >
              <li>Destination</li>
            </Link>
            <Link
              href='/'
              className='py-5'
            >
              <li>Contact</li>
            </Link>
            <Link
              href='/'
              className='py-5'
            >
              <li>Privacy Policy</li>
            </Link>
            <Link
              href='/'
              className='py-5'
            >
              <li>Terms Condition</li>
            </Link>
          </ul>
        </div>
        <div className='flex justify-center items-center py-2'>
          <MdOutlineCopyright size={16} /> 2023 Brolab
        </div>
      </div>
    </footer>
  );
}
