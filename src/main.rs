use yew::prelude::*;

#[function_component(App)]
fn index() -> Html{
    let hidden = use_state(|| true);
    let prod = {
        let hidden = hidden.clone();
        Callback::from(move |_|
            hidden.set(!(*hidden))
        )
    };
    html!{
        <main class={"wave"}>
        <navbar >
        <nav class={"relative flex items-center justify-between content-center bg-gray-200"}>
        <ul class={"flex mx-auto max-w-2/6 w-2/6 justify-center items-center"}>
            <li>
                <input type="text" placeholder="Que estas buscando?" class={"border border-solid border-gray-600"}/>
            </li>
            <li>
            <button type="image">
                <svg class={"w-8"} viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M15.7955 15.8111L21 21M18 10.5C18 14.6421 14.6421 18 10.5 18C6.35786 18 3 14.6421 3 10.5C3 6.35786 6.35786 3 10.5 3C14.6421 3 18 6.35786 18 10.5Z" stroke="#000000" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
            </button>
            </li>
        </ul>
        <img src="./img/logo_n_L.svg" class={"max-h-24 transition-all max-w-24 w-24  scale-50 hover:scale-100"}/>
        <ul class={"flex mx-auto max-w-2/6 w-2/6 justify-center items-center gap-5"}>
            <li>
                <ul class={"flex"}>
                <li>
                    <svg class={"size-5"} viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path d="M8 7C9.65685 7 11 5.65685 11 4C11 2.34315 9.65685 1 8 1C6.34315 1 5 2.34315 5 4C5 5.65685 6.34315 7 8 7Z" fill="#000000"></path> <path d="M14 12C14 10.3431 12.6569 9 11 9H5C3.34315 9 2 10.3431 2 12V15H14V12Z" fill="#000000"></path> </g></svg>
                </li>
                <li>
                    {"Ingresar"}
                </li>
                </ul>
            </li>
                <li>
                <ul class={"flex"}>
                <li>
                    <svg class={"size-5"} fill="#000000" viewBox="0 0 32 32" version="1.1" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path d="M31.739 8.875c-0.186-0.264-0.489-0.422-0.812-0.422h-21.223l-1.607-5.54c-0.63-2.182-2.127-2.417-2.741-2.417h-4.284c-0.549 0-0.993 0.445-0.993 0.993s0.445 0.993 0.993 0.993h4.283c0.136 0 0.549 0 0.831 0.974l5.527 20.311c0.12 0.428 0.511 0.724 0.956 0.724h13.499c0.419 0 0.793-0.262 0.934-0.657l4.758-14.053c0.11-0.304 0.064-0.643-0.122-0.907zM25.47 22.506h-12.046l-3.161-12.066h19.253zM23.5 26.504c-1.381 0-2.5 1.119-2.5 2.5s1.119 2.5 2.5 2.5 2.5-1.119 2.5-2.5c0-1.381-1.119-2.5-2.5-2.5zM14.5 26.504c-1.381 0-2.5 1.119-2.5 2.5s1.119 2.5 2.5 2.5 2.5-1.119 2.5-2.5c0-1.381-1.119-2.5-2.5-2.5z"></path> </g></svg>
                </li>
                <li>
                    {"Carrito (0)"}
                </li>
            </ul>
            </li>
        </ul>
        </nav>
        <nav class={"shadow-lg"}>
            <div class={"relative flex align-center items-center justify-center overflow-hidden bg-gray-200 shadown"}>
                <button type="button" class={"flex items-center gap-1 text-sm/6 text-grey-900"} aria-expanded="false" onclick={prod}>
                    {"PRODUCTOS"}
                    <svg class="size-5 flex-none text-gray-400" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true" data-slot="icon">
                    <path fill-rule="evenodd" d="M5.22 8.22a.75.75 0 0 1 1.06 0L10 11.94l3.72-3.72a.75.75 0 1 1 1.06 1.06l-4.25 4.25a.75.75 0 0 1-1.06 0L5.22 9.28a.75.75 0 0 1 0-1.06Z" clip-rule="evenodd" />
                    </svg>
                </button>
            </div>
            <div id="productos" class={"absolute left-1/3 top-fill w-1/3 rounded-3xl shadown-lg overflow-hidden ring-1 ring-gray-900/5 bg-clip-padding backdrop-filter backdrop-blur-sm bg-opacity-50 z-10 absolute" } style={"z-index: 1"}hidden = {*hidden}>
                    <div class={"flex group items-center  h-24 max-h-24 overflow-hidden bg-cat "}>
        <img src="./img/Okarun_simple.png" class={" overflow-hidden w-1/12 scale-150 transition-all  group-hover:w-2/12 "}/>
                    <a class={"text-sm transition-all group-hover:text-lg group-hover:text-gray-50"}>{"Anime"}</a>
                    
                </div>
                    <div class={"flex group items-center  h-24 max-h-24  overflow-hidden bg-vintage"}>
        <img src="./img/vintage.jpg" class={" w-1/12 transition-all group-hover:w-2/12"}/>
                    <a class={"text-sm transition-all group-hover:text-lg group-hover:text-gray-800"}>{"Vintage"}</a>
                    
                    </div>
                    <div class={"flex group items-center  h-24 max-h-24 overflow-hidden bg-bts"}>
        <img src="./img/bts.png" class={"w-1/12 transition-all group-hover:w-2/12"}/>
                    <a class={"text-sm transition-all group-hover:text-lg"}>{"BTS"}</a>
                    </div>
                    <div class={" flex group items-center   h-24 max-h-24 overflow-hidden bg-memes"}>
        <img src="./img/mesi.jpg" class={"w-1/12 transition-all group-hover:w-2/12"}/>
                    <a class={"text-sm transition-all group-hover:text-lg group-hover:text-gray-50"}>{"Memes"}</a>
                    
            </div>
                    </div>
                
        </nav>
        </navbar>

        <div class={"flex w-full h-full justify-evenly gap-10 my-12"}>
        <div class={"overflow-hidden block  w-full max-w-full h-full max-h-full  "} >
        <img src="./img/banner_anime2.png" class={"relative px-0 transition-all hover:scale-100 scale-110 w-full "}/>
        </div>
        <div class={"overflow-hidden block  w-full max-w-full h-full max-h-full  "} >
        <img src="./img/banner_memes.png" class={"relative px-0 transition-all hover:scale-100 scale-110 w-full "}/>
        </div>
        <div class={"overflow-hidden block  w-full max-w-full h-full max-h-full  "} >
        <img src="./img/banner_mas.png" class={"relative px-0 transition-all hover:scale-100 scale-110 w-full "}/>
        </div>

        </div>

        <br/>
        <hr/>
        <div class={"bg-cat-text bg-opacity-90 bg-clip-text my-10"}>
            <h1 class={"text-9xl text-transparent text-center font-black font-seriff "}> {"NUEVOS AÑADIDOS"} </h1>
        </div>
        <div class={"flex flex-row"}>

        <div class={"flex overflow-visible "}>
        <div class={"flex flex-col ring-1 ring-stone-600 ml-8 mt-8 mb-8 rounded-md w-full bg-gray-500 bg-opacity-50"}>
        <div class={"flex items-center justify-center "}>
        <div class={"group flex items-center justify-center  -mt-5 mb-5 -ml-5"}>
            <img src="./img/okarun2.png" class={"relative p-0 transition-all group-hover:bg-clip-padding group-hover:backdrop-filter group-hover:blur rounded-2xl w-full h-full"}/>
            <button type="image" class={"hidden absolute rounded-3xl shadown-lg ring-1 ring-black group-hover:inline "}><svg class={"w-8 "} viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M15.7955 15.8111L21 21M18 10.5C18 14.6421 14.6421 18 10.5 18C6.35786 18 3 14.6421 3 10.5C3 6.35786 6.35786 3 10.5 3C14.6421 3 18 6.35786 18 10.5Z" stroke="#000000" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            </button>
            </div>
            <div class={"flex flex-col justify-evenly items-center"}>
                <img src="./img/okarun2.png" class={"relative p-0 transition-all hover:p-2"} style={"width:50px; height:50px"}/>
                <img src="./img/dandadan.png" class={"relative p-0 transition-all hover:p-2"} style={"width:50px; height:50px"}/>
            </div>
        </div>
        <p class={"text-center text-md text-gray-100 font-serif"}>{"Colores"}</p>
        <div class={"flex justify-evenly w-full"}>
        <button  class={"rounded-3xl shadown-lg ring-1 ring-black bg-stone-900 w-7 h-7 "}/>
        </div>
        <p class={"text-center text-md text-gray-100 font-serif"}>{"Tamaños"}</p>
        <div class={"flex justify-evenly w-full"}>
        <button  class={"rounded-3xl shadown-lg ring-1 ring-black w-5 h-5 text-sm"}>{"XL"}</button>
        <button  class={"rounded-3xl shadown-lg ring-1 ring-black w-5 h-5 text-sm"}>{"M"}</button>
        </div>
        </div>
        <div class={"mt-8 ring-1 ring-stone-600 p-5 mb-8 mr-8 rounded-md bg-gray-50"}>
            <p class={"text-lg mt-4 mb-5"}>{"Titulo"}</p>
            <p class={"text-md text-gray-600 mb-56"}>{"Descripcion"}</p>
            <p class={"text-sm"}>{"Enviado en ~3 semanas"}</p>
            <p class={"text-md text-emerald-400 "}>{"$10.000"}</p>
            <p class={"text-sm text text-gray-400 m-2"}>{"Cantidad"}</p>
            <input type="number" class={"ml-2"} value="1" min="1" max="10"/>
            <button class={"text-lg bg-amber-600 ring-1 ring-amber-300 transition-all shadown-lg rounded-2xl hover:bg-emerald-400 hover:underline p-2 w-full"}>
            {"Guardar"}
            </button>
            <button class={"text-lg bg-purple-600 ring-1 ring-amber-300 transition-all shadown-lg rounded-2xl hover:bg-emerald-400 hover:underline p-2 w-full"}>
            {"Comprar"}
            </button>
        </div>
        </div>

        <div class={"flex overflow-visible"}>
        <div class={"flex flex-col ring-1 ring-stone-600 ml-8 mt-8 mb-8 rounded-md w-full bg-gray-500 bg-opacity-50"}>
        <div class={"flex items-center justify-center "}>
        <div class={"group flex items-center justify-center  -mt-5 mb-5 -ml-5"}>
            <img src="./img/okarun1.png" class={"relative transition-all group-hover:bg-clip-padding group-hover:backdrop-filter group-hover:blur rounded-2xl w-full h-full"}/>
            <button type="image" class={"hidden absolute rounded-3xl shadown-lg ring-1 ring-black group-hover:inline "}><svg class={"w-8 "} viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M15.7955 15.8111L21 21M18 10.5C18 14.6421 14.6421 18 10.5 18C6.35786 18 3 14.6421 3 10.5C3 6.35786 6.35786 3 10.5 3C14.6421 3 18 6.35786 18 10.5Z" stroke="#000000" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            </button>
            </div>
            <div class={"flex flex-col justify-evenly items-center"}>
            <img src="./img/okarun1.png" class={"relative p-0 transition-all hover:p-2"} style={"width:50px; height:50px"}/>

                <img src="./img/dandadan.png" class={"relative p-0 transition-all hover:p-2"} style={"width:50px; height:50px"}/>
            </div>
        </div>
        <p class={"text-center text-md text-gray-100 font-serif"}>{"Colores"}</p>
        <div class={"flex justify-evenly w-full"}>
        <button  class={"rounded-3xl shadown-lg ring-1 ring-black bg-stone-900 w-7 h-7 "}/>

        </div>
        <p class={"text-center text-md text-gray-100 font-serif"}>{"Tamaños"}</p>
        <div class={"flex justify-evenly w-full"}>
        <button  class={"rounded-3xl shadown-lg ring-1 ring-black w-5 h-5 text-sm"}>{"XL"}</button>
        <button  class={"rounded-3xl shadown-lg ring-1 ring-black w-5 h-5 text-sm"}>{"M"}</button>

        </div>
        </div>
        <div class={"mt-8 ring-1 ring-stone-600 p-5 mb-8 mr-8 rounded-md bg-gray-50"}>
            <p class={"text-lg mt-4 mb-5"}>{"Titulo"}</p>
            <p class={"text-md text-gray-600 mb-56"}>{"Descripcion"}</p>
            <p class={"text-sm"}>{"Enviado en ~3 semanas"}</p>
            <p class={"text-md text-emerald-400 "}>{"$10.000"}</p>
            <p class={"text-sm text text-gray-400 m-2"}>{"Cantidad"}</p>
            <input type="number" class={"ml-2"} value="1" min="1" max="10"/>
            <button class={"text-lg bg-amber-600 ring-1 ring-amber-300 transition-all shadown-lg rounded-2xl hover:bg-emerald-400 hover:underline p-2 w-full"}>
            {"Guardar"}
            </button>
            <button class={"text-lg bg-purple-600 ring-1 ring-amber-300 transition-all shadown-lg rounded-2xl hover:bg-emerald-400 hover:underline p-2 w-full"}>
            {"Comprar"}
            </button>
        </div>
        </div>
        
        </div>

        

        <footer class={"bg-stone-800"}>
            <p class={"text-lg text-neutral-300"}>{"Este es un prototipo de pagina, no presenta ningun API ni esta pensada para varios tamaños de ventanas"}</p>
            <a class={"text-md  group text-neutral-300 block w-fit"} href={"#"}>
                {"Instagram"}
                <div class={"transition-all bg-red-500 h-[2px] w-0 group-hover:w-full"}></div></a>
            <a class={"text-md group  w-fit text-neutral-300 block"} href={"www.google.com"}>
                {"TikTok"}
                <div class={"transition-all bg-purple-500 h-[2px] w-0 group-hover:w-full"}></div></a>
            <a class={" text-md group  w-fit text-neutral-300 block"} href={"www.google.com"}>{"Ubicacion"}
                <div class={"transition-all bg-green-500 h-[2px] w-0 group-hover:w-full"}></div></a>
            <a class={" text-md group  w-fit text-neutral-300 block"} href={"www.google.com"}>{"Legal"}
            <div class={"transition-all bg-amber-500 h-[2px] w-0 group-hover:w-full"}></div></a>
        </footer>
        </main>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
