import Section3 from "@/components/Section3";
import Section4 from "@/components/Section4";
import Section5 from "@/components/Section5";
import SectionTwo from "@/components/SectionTwo";
import Image from "next/image";
import { MdKeyboardArrowRight } from "react-icons/md";

export default function Home() {
  return (
    <div>
      <div className='bg-gradient-to-r from-violet-500 to-fuchsia-500'>
        <div className='relative'>
          <Image
            src={"/assets/bg-banner.png"}
            alt='Banner'
            width={1920}
            height={971}
            className='object-cover w-full'
            priority
          />
          <div className='grid 2xl:grid-cols-2 absolute top-0 left-0 right-0 bottom-0 px-3 container mx-auto'>
            <div className='flex flex-col justify-center items-start text-white font-bold text-5xl tracking-wider leading-normal order-last 2xl:order-first'>
              <h2>On-chain Governance</h2>
              <h2>for your bussiness</h2>
              <div className='flex space-x-5 mt-5'>
                <button className='bg-white text-black text-sm px-4 py-2 rounded-full hover:bg-white/90 transition-colors duration-300'>
                  Get start now
                </button>
                <button className='flex items-centertext-white text-sm px-4 py-2'>
                  Join telegram
                  <span className='ml-2'>
                    <MdKeyboardArrowRight size={24} />
                  </span>
                </button>
              </div>
            </div>
            <div className='flex justify-end items-center'>
              <Image
                src={"/assets/iphone.png"}
                alt='Phone'
                width={383}
                height={665}
                className='object-cover mt-[250px] 2xl:mt-0'
                priority
              />
            </div>
          </div>
        </div>
      </div>
      <div className='mt-[1500px] lg:mt-0'>
        <SectionTwo />
        <Section3 />
        <Section4 />
        <Section5 />
      </div>
    </div>
  );
}
