import { Spotlight } from "../components/ui/Spotlight"

export default function HomePage() {

    return (
        <div className="h-screen w-full rounded-md flex md:items-center md:justify-center bg-black/[0.96] antialiased bg-grid-white/[0.02] relative overflow-hidden">
            <Spotlight
                className="-top-40 left-0 md:left-60 md:-top-20"
                fill="white"
            />
            <div className=" p-4 max-w-7xl  mx-auto relative z-10  w-full pt-20 md:pt-0">
                <h1 className="text-4xl md:text-7xl font-bold text-center bg-clip-text text-transparent bg-gradient-to-b from-neutral-50 to-neutral-400 bg-opacity-50">
                    Welcome to <br /> create C.R.A.P App
                </h1>
                <p className="mt-5 font-normal text-base text-neutral-300 max-w-lg text-center mx-auto">
                    Use Rust to build a fast, reliable and secure backend with the power of Actix Web.
                    Use the power and reliablity of React combined with tools like ShadCn Ui and tailwind.css
                    to create good looking websites with a streamlined developer experience.
                </p>
            </div>
        </div>
    )
}
