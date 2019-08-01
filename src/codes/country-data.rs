/* generated from ISO-3166 data files */

fn create_region_table() -> HashMap<u16, &'static Region> {
    let mut table = HashMap::new();
    table.insert(4, &Region {
        code: 4,
        name: "Afghanistan",
    });
    table.insert(142, &Region {
        code: 142,
        name: "Asia",
    });
    table.insert(34, &Region {
        code: 34,
        name: "Southern Asia",
    });
    table.insert(248, &Region {
        code: 248,
        name: "Åland Islands",
    });
    table.insert(150, &Region {
        code: 150,
        name: "Europe",
    });
    table.insert(154, &Region {
        code: 154,
        name: "Northern Europe",
    });
    table.insert(8, &Region {
        code: 8,
        name: "Albania",
    });
    table.insert(39, &Region {
        code: 39,
        name: "Southern Europe",
    });
    table.insert(12, &Region {
        code: 12,
        name: "Algeria",
    });
    table.insert(2, &Region {
        code: 2,
        name: "Africa",
    });
    table.insert(15, &Region {
        code: 15,
        name: "Northern Africa",
    });
    table.insert(16, &Region {
        code: 16,
        name: "American Samoa",
    });
    table.insert(9, &Region {
        code: 9,
        name: "Oceania",
    });
    table.insert(61, &Region {
        code: 61,
        name: "Polynesia",
    });
    table.insert(20, &Region {
        code: 20,
        name: "Andorra",
    });
    table.insert(24, &Region {
        code: 24,
        name: "Angola",
    });
    table.insert(202, &Region {
        code: 202,
        name: "Sub-Saharan Africa",
    });
    table.insert(17, &Region {
        code: 17,
        name: "Middle Africa",
    });
    table.insert(660, &Region {
        code: 660,
        name: "Anguilla",
    });
    table.insert(19, &Region {
        code: 19,
        name: "Americas",
    });
    table.insert(419, &Region {
        code: 419,
        name: "Latin America and the Caribbean",
    });
    table.insert(29, &Region {
        code: 29,
        name: "Caribbean",
    });
    table.insert(10, &Region {
        code: 10,
        name: "Antarctica",
    });
    table.insert(28, &Region {
        code: 28,
        name: "Antigua and Barbuda",
    });
    table.insert(32, &Region {
        code: 32,
        name: "Argentina",
    });
    table.insert(5, &Region {
        code: 5,
        name: "South America",
    });
    table.insert(51, &Region {
        code: 51,
        name: "Armenia",
    });
    table.insert(145, &Region {
        code: 145,
        name: "Western Asia",
    });
    table.insert(533, &Region {
        code: 533,
        name: "Aruba",
    });
    table.insert(36, &Region {
        code: 36,
        name: "Australia",
    });
    table.insert(53, &Region {
        code: 53,
        name: "Australia and New Zealand",
    });
    table.insert(40, &Region {
        code: 40,
        name: "Austria",
    });
    table.insert(155, &Region {
        code: 155,
        name: "Western Europe",
    });
    table.insert(31, &Region {
        code: 31,
        name: "Azerbaijan",
    });
    table.insert(44, &Region {
        code: 44,
        name: "Bahamas",
    });
    table.insert(48, &Region {
        code: 48,
        name: "Bahrain",
    });
    table.insert(50, &Region {
        code: 50,
        name: "Bangladesh",
    });
    table.insert(52, &Region {
        code: 52,
        name: "Barbados",
    });
    table.insert(112, &Region {
        code: 112,
        name: "Belarus",
    });
    table.insert(151, &Region {
        code: 151,
        name: "Eastern Europe",
    });
    table.insert(56, &Region {
        code: 56,
        name: "Belgium",
    });
    table.insert(84, &Region {
        code: 84,
        name: "Belize",
    });
    table.insert(13, &Region {
        code: 13,
        name: "Central America",
    });
    table.insert(204, &Region {
        code: 204,
        name: "Benin",
    });
    table.insert(11, &Region {
        code: 11,
        name: "Western Africa",
    });
    table.insert(60, &Region {
        code: 60,
        name: "Bermuda",
    });
    table.insert(21, &Region {
        code: 21,
        name: "Northern America",
    });
    table.insert(64, &Region {
        code: 64,
        name: "Bhutan",
    });
    table.insert(68, &Region {
        code: 68,
        name: "Bolivia (Plurinational State of)",
    });
    table.insert(535, &Region {
        code: 535,
        name: "Bonaire, Sint Eustatius and Saba",
    });
    table.insert(70, &Region {
        code: 70,
        name: "Bosnia and Herzegovina",
    });
    table.insert(72, &Region {
        code: 72,
        name: "Botswana",
    });
    table.insert(18, &Region {
        code: 18,
        name: "Southern Africa",
    });
    table.insert(74, &Region {
        code: 74,
        name: "Bouvet Island",
    });
    table.insert(76, &Region {
        code: 76,
        name: "Brazil",
    });
    table.insert(86, &Region {
        code: 86,
        name: "British Indian Ocean Territory",
    });
    table.insert(14, &Region {
        code: 14,
        name: "Eastern Africa",
    });
    table.insert(96, &Region {
        code: 96,
        name: "Brunei Darussalam",
    });
    table.insert(35, &Region {
        code: 35,
        name: "South-eastern Asia",
    });
    table.insert(100, &Region {
        code: 100,
        name: "Bulgaria",
    });
    table.insert(854, &Region {
        code: 854,
        name: "Burkina Faso",
    });
    table.insert(108, &Region {
        code: 108,
        name: "Burundi",
    });
    table.insert(132, &Region {
        code: 132,
        name: "Cabo Verde",
    });
    table.insert(116, &Region {
        code: 116,
        name: "Cambodia",
    });
    table.insert(120, &Region {
        code: 120,
        name: "Cameroon",
    });
    table.insert(124, &Region {
        code: 124,
        name: "Canada",
    });
    table.insert(136, &Region {
        code: 136,
        name: "Cayman Islands",
    });
    table.insert(140, &Region {
        code: 140,
        name: "Central African Republic",
    });
    table.insert(148, &Region {
        code: 148,
        name: "Chad",
    });
    table.insert(152, &Region {
        code: 152,
        name: "Chile",
    });
    table.insert(156, &Region {
        code: 156,
        name: "China",
    });
    table.insert(30, &Region {
        code: 30,
        name: "Eastern Asia",
    });
    table.insert(162, &Region {
        code: 162,
        name: "Christmas Island",
    });
    table.insert(166, &Region {
        code: 166,
        name: "Cocos (Keeling) Islands",
    });
    table.insert(170, &Region {
        code: 170,
        name: "Colombia",
    });
    table.insert(174, &Region {
        code: 174,
        name: "Comoros",
    });
    table.insert(178, &Region {
        code: 178,
        name: "Congo",
    });
    table.insert(180, &Region {
        code: 180,
        name: "Congo, Democratic Republic of the",
    });
    table.insert(184, &Region {
        code: 184,
        name: "Cook Islands",
    });
    table.insert(188, &Region {
        code: 188,
        name: "Costa Rica",
    });
    table.insert(384, &Region {
        code: 384,
        name: "Côte d'Ivoire",
    });
    table.insert(191, &Region {
        code: 191,
        name: "Croatia",
    });
    table.insert(192, &Region {
        code: 192,
        name: "Cuba",
    });
    table.insert(531, &Region {
        code: 531,
        name: "Curaçao",
    });
    table.insert(196, &Region {
        code: 196,
        name: "Cyprus",
    });
    table.insert(203, &Region {
        code: 203,
        name: "Czechia",
    });
    table.insert(208, &Region {
        code: 208,
        name: "Denmark",
    });
    table.insert(262, &Region {
        code: 262,
        name: "Djibouti",
    });
    table.insert(212, &Region {
        code: 212,
        name: "Dominica",
    });
    table.insert(214, &Region {
        code: 214,
        name: "Dominican Republic",
    });
    table.insert(218, &Region {
        code: 218,
        name: "Ecuador",
    });
    table.insert(818, &Region {
        code: 818,
        name: "Egypt",
    });
    table.insert(222, &Region {
        code: 222,
        name: "El Salvador",
    });
    table.insert(226, &Region {
        code: 226,
        name: "Equatorial Guinea",
    });
    table.insert(232, &Region {
        code: 232,
        name: "Eritrea",
    });
    table.insert(233, &Region {
        code: 233,
        name: "Estonia",
    });
    table.insert(748, &Region {
        code: 748,
        name: "Eswatini",
    });
    table.insert(231, &Region {
        code: 231,
        name: "Ethiopia",
    });
    table.insert(238, &Region {
        code: 238,
        name: "Falkland Islands (Malvinas)",
    });
    table.insert(234, &Region {
        code: 234,
        name: "Faroe Islands",
    });
    table.insert(242, &Region {
        code: 242,
        name: "Fiji",
    });
    table.insert(54, &Region {
        code: 54,
        name: "Melanesia",
    });
    table.insert(246, &Region {
        code: 246,
        name: "Finland",
    });
    table.insert(250, &Region {
        code: 250,
        name: "France",
    });
    table.insert(254, &Region {
        code: 254,
        name: "French Guiana",
    });
    table.insert(258, &Region {
        code: 258,
        name: "French Polynesia",
    });
    table.insert(260, &Region {
        code: 260,
        name: "French Southern Territories",
    });
    table.insert(266, &Region {
        code: 266,
        name: "Gabon",
    });
    table.insert(270, &Region {
        code: 270,
        name: "Gambia",
    });
    table.insert(268, &Region {
        code: 268,
        name: "Georgia",
    });
    table.insert(276, &Region {
        code: 276,
        name: "Germany",
    });
    table.insert(288, &Region {
        code: 288,
        name: "Ghana",
    });
    table.insert(292, &Region {
        code: 292,
        name: "Gibraltar",
    });
    table.insert(300, &Region {
        code: 300,
        name: "Greece",
    });
    table.insert(304, &Region {
        code: 304,
        name: "Greenland",
    });
    table.insert(308, &Region {
        code: 308,
        name: "Grenada",
    });
    table.insert(312, &Region {
        code: 312,
        name: "Guadeloupe",
    });
    table.insert(316, &Region {
        code: 316,
        name: "Guam",
    });
    table.insert(57, &Region {
        code: 57,
        name: "Micronesia",
    });
    table.insert(320, &Region {
        code: 320,
        name: "Guatemala",
    });
    table.insert(831, &Region {
        code: 831,
        name: "Guernsey",
    });
    table.insert(830, &Region {
        code: 830,
        name: "Channel Islands",
    });
    table.insert(324, &Region {
        code: 324,
        name: "Guinea",
    });
    table.insert(624, &Region {
        code: 624,
        name: "Guinea-Bissau",
    });
    table.insert(328, &Region {
        code: 328,
        name: "Guyana",
    });
    table.insert(332, &Region {
        code: 332,
        name: "Haiti",
    });
    table.insert(334, &Region {
        code: 334,
        name: "Heard Island and McDonald Islands",
    });
    table.insert(336, &Region {
        code: 336,
        name: "Holy See",
    });
    table.insert(340, &Region {
        code: 340,
        name: "Honduras",
    });
    table.insert(344, &Region {
        code: 344,
        name: "Hong Kong",
    });
    table.insert(348, &Region {
        code: 348,
        name: "Hungary",
    });
    table.insert(352, &Region {
        code: 352,
        name: "Iceland",
    });
    table.insert(356, &Region {
        code: 356,
        name: "India",
    });
    table.insert(360, &Region {
        code: 360,
        name: "Indonesia",
    });
    table.insert(364, &Region {
        code: 364,
        name: "Iran (Islamic Republic of)",
    });
    table.insert(368, &Region {
        code: 368,
        name: "Iraq",
    });
    table.insert(372, &Region {
        code: 372,
        name: "Ireland",
    });
    table.insert(833, &Region {
        code: 833,
        name: "Isle of Man",
    });
    table.insert(376, &Region {
        code: 376,
        name: "Israel",
    });
    table.insert(380, &Region {
        code: 380,
        name: "Italy",
    });
    table.insert(388, &Region {
        code: 388,
        name: "Jamaica",
    });
    table.insert(392, &Region {
        code: 392,
        name: "Japan",
    });
    table.insert(832, &Region {
        code: 832,
        name: "Jersey",
    });
    table.insert(400, &Region {
        code: 400,
        name: "Jordan",
    });
    table.insert(398, &Region {
        code: 398,
        name: "Kazakhstan",
    });
    table.insert(143, &Region {
        code: 143,
        name: "Central Asia",
    });
    table.insert(404, &Region {
        code: 404,
        name: "Kenya",
    });
    table.insert(296, &Region {
        code: 296,
        name: "Kiribati",
    });
    table.insert(408, &Region {
        code: 408,
        name: "Korea (Democratic People's Republic of)",
    });
    table.insert(410, &Region {
        code: 410,
        name: "Korea, Republic of",
    });
    table.insert(414, &Region {
        code: 414,
        name: "Kuwait",
    });
    table.insert(417, &Region {
        code: 417,
        name: "Kyrgyzstan",
    });
    table.insert(418, &Region {
        code: 418,
        name: "Lao People's Democratic Republic",
    });
    table.insert(428, &Region {
        code: 428,
        name: "Latvia",
    });
    table.insert(422, &Region {
        code: 422,
        name: "Lebanon",
    });
    table.insert(426, &Region {
        code: 426,
        name: "Lesotho",
    });
    table.insert(430, &Region {
        code: 430,
        name: "Liberia",
    });
    table.insert(434, &Region {
        code: 434,
        name: "Libya",
    });
    table.insert(438, &Region {
        code: 438,
        name: "Liechtenstein",
    });
    table.insert(440, &Region {
        code: 440,
        name: "Lithuania",
    });
    table.insert(442, &Region {
        code: 442,
        name: "Luxembourg",
    });
    table.insert(446, &Region {
        code: 446,
        name: "Macao",
    });
    table.insert(450, &Region {
        code: 450,
        name: "Madagascar",
    });
    table.insert(454, &Region {
        code: 454,
        name: "Malawi",
    });
    table.insert(458, &Region {
        code: 458,
        name: "Malaysia",
    });
    table.insert(462, &Region {
        code: 462,
        name: "Maldives",
    });
    table.insert(466, &Region {
        code: 466,
        name: "Mali",
    });
    table.insert(470, &Region {
        code: 470,
        name: "Malta",
    });
    table.insert(584, &Region {
        code: 584,
        name: "Marshall Islands",
    });
    table.insert(474, &Region {
        code: 474,
        name: "Martinique",
    });
    table.insert(478, &Region {
        code: 478,
        name: "Mauritania",
    });
    table.insert(480, &Region {
        code: 480,
        name: "Mauritius",
    });
    table.insert(175, &Region {
        code: 175,
        name: "Mayotte",
    });
    table.insert(484, &Region {
        code: 484,
        name: "Mexico",
    });
    table.insert(583, &Region {
        code: 583,
        name: "Micronesia (Federated States of)",
    });
    table.insert(498, &Region {
        code: 498,
        name: "Moldova, Republic of",
    });
    table.insert(492, &Region {
        code: 492,
        name: "Monaco",
    });
    table.insert(496, &Region {
        code: 496,
        name: "Mongolia",
    });
    table.insert(499, &Region {
        code: 499,
        name: "Montenegro",
    });
    table.insert(500, &Region {
        code: 500,
        name: "Montserrat",
    });
    table.insert(504, &Region {
        code: 504,
        name: "Morocco",
    });
    table.insert(508, &Region {
        code: 508,
        name: "Mozambique",
    });
    table.insert(104, &Region {
        code: 104,
        name: "Myanmar",
    });
    table.insert(516, &Region {
        code: 516,
        name: "Namibia",
    });
    table.insert(520, &Region {
        code: 520,
        name: "Nauru",
    });
    table.insert(524, &Region {
        code: 524,
        name: "Nepal",
    });
    table.insert(528, &Region {
        code: 528,
        name: "Netherlands",
    });
    table.insert(540, &Region {
        code: 540,
        name: "New Caledonia",
    });
    table.insert(554, &Region {
        code: 554,
        name: "New Zealand",
    });
    table.insert(558, &Region {
        code: 558,
        name: "Nicaragua",
    });
    table.insert(562, &Region {
        code: 562,
        name: "Niger",
    });
    table.insert(566, &Region {
        code: 566,
        name: "Nigeria",
    });
    table.insert(570, &Region {
        code: 570,
        name: "Niue",
    });
    table.insert(574, &Region {
        code: 574,
        name: "Norfolk Island",
    });
    table.insert(807, &Region {
        code: 807,
        name: "North Macedonia",
    });
    table.insert(580, &Region {
        code: 580,
        name: "Northern Mariana Islands",
    });
    table.insert(578, &Region {
        code: 578,
        name: "Norway",
    });
    table.insert(512, &Region {
        code: 512,
        name: "Oman",
    });
    table.insert(586, &Region {
        code: 586,
        name: "Pakistan",
    });
    table.insert(585, &Region {
        code: 585,
        name: "Palau",
    });
    table.insert(275, &Region {
        code: 275,
        name: "Palestine, State of",
    });
    table.insert(591, &Region {
        code: 591,
        name: "Panama",
    });
    table.insert(598, &Region {
        code: 598,
        name: "Papua New Guinea",
    });
    table.insert(600, &Region {
        code: 600,
        name: "Paraguay",
    });
    table.insert(604, &Region {
        code: 604,
        name: "Peru",
    });
    table.insert(608, &Region {
        code: 608,
        name: "Philippines",
    });
    table.insert(612, &Region {
        code: 612,
        name: "Pitcairn",
    });
    table.insert(616, &Region {
        code: 616,
        name: "Poland",
    });
    table.insert(620, &Region {
        code: 620,
        name: "Portugal",
    });
    table.insert(630, &Region {
        code: 630,
        name: "Puerto Rico",
    });
    table.insert(634, &Region {
        code: 634,
        name: "Qatar",
    });
    table.insert(638, &Region {
        code: 638,
        name: "Réunion",
    });
    table.insert(642, &Region {
        code: 642,
        name: "Romania",
    });
    table.insert(643, &Region {
        code: 643,
        name: "Russian Federation",
    });
    table.insert(646, &Region {
        code: 646,
        name: "Rwanda",
    });
    table.insert(652, &Region {
        code: 652,
        name: "Saint Barthélemy",
    });
    table.insert(654, &Region {
        code: 654,
        name: "Saint Helena, Ascension and Tristan da Cunha",
    });
    table.insert(659, &Region {
        code: 659,
        name: "Saint Kitts and Nevis",
    });
    table.insert(662, &Region {
        code: 662,
        name: "Saint Lucia",
    });
    table.insert(663, &Region {
        code: 663,
        name: "Saint Martin (French part)",
    });
    table.insert(666, &Region {
        code: 666,
        name: "Saint Pierre and Miquelon",
    });
    table.insert(670, &Region {
        code: 670,
        name: "Saint Vincent and the Grenadines",
    });
    table.insert(882, &Region {
        code: 882,
        name: "Samoa",
    });
    table.insert(674, &Region {
        code: 674,
        name: "San Marino",
    });
    table.insert(678, &Region {
        code: 678,
        name: "Sao Tome and Principe",
    });
    table.insert(682, &Region {
        code: 682,
        name: "Saudi Arabia",
    });
    table.insert(686, &Region {
        code: 686,
        name: "Senegal",
    });
    table.insert(688, &Region {
        code: 688,
        name: "Serbia",
    });
    table.insert(690, &Region {
        code: 690,
        name: "Seychelles",
    });
    table.insert(694, &Region {
        code: 694,
        name: "Sierra Leone",
    });
    table.insert(702, &Region {
        code: 702,
        name: "Singapore",
    });
    table.insert(534, &Region {
        code: 534,
        name: "Sint Maarten (Dutch part)",
    });
    table.insert(703, &Region {
        code: 703,
        name: "Slovakia",
    });
    table.insert(705, &Region {
        code: 705,
        name: "Slovenia",
    });
    table.insert(90, &Region {
        code: 90,
        name: "Solomon Islands",
    });
    table.insert(706, &Region {
        code: 706,
        name: "Somalia",
    });
    table.insert(710, &Region {
        code: 710,
        name: "South Africa",
    });
    table.insert(239, &Region {
        code: 239,
        name: "South Georgia and the South Sandwich Islands",
    });
    table.insert(728, &Region {
        code: 728,
        name: "South Sudan",
    });
    table.insert(724, &Region {
        code: 724,
        name: "Spain",
    });
    table.insert(144, &Region {
        code: 144,
        name: "Sri Lanka",
    });
    table.insert(729, &Region {
        code: 729,
        name: "Sudan",
    });
    table.insert(740, &Region {
        code: 740,
        name: "Suriname",
    });
    table.insert(744, &Region {
        code: 744,
        name: "Svalbard and Jan Mayen",
    });
    table.insert(752, &Region {
        code: 752,
        name: "Sweden",
    });
    table.insert(756, &Region {
        code: 756,
        name: "Switzerland",
    });
    table.insert(760, &Region {
        code: 760,
        name: "Syrian Arab Republic",
    });
    table.insert(158, &Region {
        code: 158,
        name: "Taiwan, Province of China",
    });
    table.insert(762, &Region {
        code: 762,
        name: "Tajikistan",
    });
    table.insert(834, &Region {
        code: 834,
        name: "Tanzania, United Republic of",
    });
    table.insert(764, &Region {
        code: 764,
        name: "Thailand",
    });
    table.insert(626, &Region {
        code: 626,
        name: "Timor-Leste",
    });
    table.insert(768, &Region {
        code: 768,
        name: "Togo",
    });
    table.insert(772, &Region {
        code: 772,
        name: "Tokelau",
    });
    table.insert(776, &Region {
        code: 776,
        name: "Tonga",
    });
    table.insert(780, &Region {
        code: 780,
        name: "Trinidad and Tobago",
    });
    table.insert(788, &Region {
        code: 788,
        name: "Tunisia",
    });
    table.insert(792, &Region {
        code: 792,
        name: "Turkey",
    });
    table.insert(795, &Region {
        code: 795,
        name: "Turkmenistan",
    });
    table.insert(796, &Region {
        code: 796,
        name: "Turks and Caicos Islands",
    });
    table.insert(798, &Region {
        code: 798,
        name: "Tuvalu",
    });
    table.insert(800, &Region {
        code: 800,
        name: "Uganda",
    });
    table.insert(804, &Region {
        code: 804,
        name: "Ukraine",
    });
    table.insert(784, &Region {
        code: 784,
        name: "United Arab Emirates",
    });
    table.insert(826, &Region {
        code: 826,
        name: "United Kingdom of Great Britain and Northern Ireland",
    });
    table.insert(840, &Region {
        code: 840,
        name: "United States of America",
    });
    table.insert(581, &Region {
        code: 581,
        name: "United States Minor Outlying Islands",
    });
    table.insert(858, &Region {
        code: 858,
        name: "Uruguay",
    });
    table.insert(860, &Region {
        code: 860,
        name: "Uzbekistan",
    });
    table.insert(548, &Region {
        code: 548,
        name: "Vanuatu",
    });
    table.insert(862, &Region {
        code: 862,
        name: "Venezuela (Bolivarian Republic of)",
    });
    table.insert(704, &Region {
        code: 704,
        name: "Viet Nam",
    });
    table.insert(92, &Region {
        code: 92,
        name: "Virgin Islands (British)",
    });
    table.insert(850, &Region {
        code: 850,
        name: "Virgin Islands (U.S.)",
    });
    table.insert(876, &Region {
        code: 876,
        name: "Wallis and Futuna",
    });
    table.insert(732, &Region {
        code: 732,
        name: "Western Sahara",
    });
    table.insert(887, &Region {
        code: 887,
        name: "Yemen",
    });
    table.insert(894, &Region {
        code: 894,
        name: "Zambia",
    });
    table.insert(716, &Region {
        code: 716,
        name: "Zimbabwe",
    });
    table
}
fn add_to_lookup_table_0(table: &mut HashMap<InfoString, &'static CountryInfo>) {
    println!(">>> add_to_lookup_table_0");
    table.insert("AFG", &CountryInfo {
        code: "AFG",
        short_code: "AF",
        country_code: 4,
        region_code: Some(142),
        sub_region_code: Some(34),
        intermediate_region_code: None,
    });
    println!(">>> add_to_lookup_table_0 done");
}
fn create_country_table() -> HashMap<InfoString, &'static CountryInfo> {
    println!(">>> create_country_table");
    let mut table = HashMap::new();
    println!(">>> create_country_table created");
    add_to_lookup_table_0(&mut table);
    println!(">>> create_country_table returning {}", table.len());
    table
}
