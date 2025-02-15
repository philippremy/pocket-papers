import { Head } from "$fresh/runtime.ts";
import { ComponentChildren } from "preact/src/index.d.ts";

export default function NavBar(props: { children?: ComponentChildren }) {
    return(
        <>
        <Head>
            <link rel="stylesheet" href="/navbar.css" ></link>
        </Head>
        <nav class="navbar-header">
            <input id="globalnav-menutrigger-button" 
            type="checkbox" class="navbar-hamburger-input" hidden />
            <div class="navbar-logo-container">
                <svg xmlns="http://www.w3.org/2000/svg" id="Ebene_1" width="70%" height="70%" data-name="Ebene 1" viewBox="0 0 2047.32 2048.03">
                    <title>Unbenannt-1</title>
                    <g>
                        <path d="M917,2043.78c0,9.1-2.64,10.9-11.18,10.87-94.69-.3-189.36-.18-284.05-.18h-9.55V1800.4H2.53V1546.1H611.19V1371.78H2.44v-254.4H12.2q446.92,0,893.83-.21c8.87,0,11,2.29,11,11Q916.67,1586,917,2043.78Z" transform="translate(-2.44 -6.63)" style="fill: currentColor"/>
                        <path d="M1440.38,1545.86h608.74v253.83H1439.82v11.53c0,77.6-.13,155.19.17,232.8,0,8.3-2.13,10.66-10.54,10.64-95-.3-189.94-.2-284.9-.2h-9.12V1117.37h902.13c12.13,0,12.14,0,12.14,12.15q0,113.8,0,227.58c0,16.37,2.26,14.49-14.09,14.49q-291.42.06-582.85,0h-12.34Z" transform="translate(-2.44 -6.63)" style="fill: currentColor"/>
                        <polygon points="913.82 0.26 913.82 935.51 0.02 935.51 0.02 682.58 608.88 682.58 608.88 507.28 0.06 507.28 0.06 253.75 609.15 253.75 609.15 0.26 913.82 0.26" style="fill: currentColor"/>
                        <polygon points="1438.13 506.97 1438.13 682.13 2046.71 682.13 2046.71 935.76 1132.95 935.76 1132.95 0 1437.35 0 1437.35 253.54 2046.71 253.54 2046.71 506.97 1438.13 506.97" style="fill: currentColor"/>
                    </g>
                </svg>
            </div>
            <div class="navbar-link-holder">
                {props.children}
            </div>
            <label for="globalnav-menutrigger-button" class="navbar-hamburger-container">
                <svg width="18" height="18" viewBox="0 0 18 18" class="navbar-hamburger-svg">
                    <polyline fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round" points="2 12, 16 12" class="bottom">
                    </polyline>
                    <polyline fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round" points="2 5, 16 5" class="top">
                    </polyline>
                </svg>
            </label>
        </nav>
        </>
    )
}

export function NavBarTextLink(props: { text: string, href: string }) {
    return(
        <div class="navbar-linktext-wrapper">
            <a href={props.href} class="navbar-text-link">{props.text}</a>
            <p class="navbar-linktext-chevron">{">"}</p>
        </div>
    )
}