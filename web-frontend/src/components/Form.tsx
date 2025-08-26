import { AdapterDayjs} from "@mui/x-date-pickers/AdapterDayjs";
import {DatePicker, LocalizationProvider} from "@mui/x-date-pickers";
import dayjs, { Dayjs } from "dayjs";
import {type ChangeEvent, useState} from "react";
import {
    Button,
    Checkbox,
    Container,
    FormControl, FormControlLabel, FormGroup, Grid,
    InputLabel,
    MenuItem,
    Select,
    type SelectChangeEvent,
    Stack,
    Typography,
    TableContainer, Table, Paper, TableHead, TableRow, TableCell, TableBody, TextField, InputAdornment,
} from "@mui/material";

function Form() {

    const [date, setDate] = useState<Dayjs | null>(dayjs());
    const [day, setDay] = useState("Monday");
    const [sets, setSets] = useState(1);
    const week = 7;
    const [weightTable, setWeightTable] = useState([["", "", "", ""]]);
    const [cardioTable, setCardioTable] = useState([["", "", "", "", ""]]);

    const [todayFocus, setTodayFocus] = useState({
        upper: false,
        lower: false,
        core: false,
    });

    function addWeightRow() {
        const newArray = weightTable.slice();
        const newRow = ["Group", "Action"];
        for (let i = 0; i < sets; i++) {
            newRow.push("");
        }
        newArray.push(newRow);
        setWeightTable(newArray);
    }

    const handleChange = (event: SelectChangeEvent) => {
        setDay(event.target.value as string);
    }

    const changeTableVal = (event: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>, m: number, n: number) => {
        const tempArray = weightTable.slice();
        tempArray[m][n] = event.target.value;
        setWeightTable(tempArray);
    }

    const changeCardioTableVal = (event: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>, m: number, n: number) => {
        const tempArray = cardioTable.slice();
        tempArray[m][n] = event.target.value;
        setCardioTable(tempArray);
    }

    function getDayNum(day: string) {
        switch (day) {
            case "Monday":
                return 1;
            case "Tuesday":
                return 2;
            case "Wednesday":
                return 3;
            case "Thursday":
                return 4;
            case "Friday":
                return 5;
            case "Saturday":
                return 6;
            case "Sunday":
                return 7;
            case "":
                return 1;
        }
    }

    function addSet() {
        setSets(sets+1);
        const copy = weightTable.slice();
        copy.forEach((element) => {
            element.push("");
            element.push("");
        });
        setWeightTable(copy);
        console.log(weightTable);
    }

    function getSetHeader(setNum: number) {
        return (
            <TableCell key={`header-${setNum}`}>
                <Typography textAlign="center">SET {setNum}</Typography>
                <Stack direction="row" sx={{ justifyContent: "space-between" }}>
                    <Typography>Reps</Typography>
                    <Typography>Weight</Typography>
                </Stack>
            </TableCell>
        );
    }

    function addCardio() {
        const newArray = cardioTable.slice();
        const newRow = ["", "", "", "", ""];
        newArray.push(newRow);
        console.log(cardioTable);
        setCardioTable(newArray);
    }

    const handleTodayFocus = (event: ChangeEvent<HTMLInputElement>, target: string) => {
        switch (target) {
            case "upper":
                setTodayFocus({ ...todayFocus, upper: event.target.checked });
                break;
            case "lower":
                setTodayFocus({ ...todayFocus, lower: event.target.checked });
                break;
            case "core":
                setTodayFocus({ ...todayFocus, core: event.target.checked });
                break;
        }
    };

    function getJSONObject() {
        return {
            date: date?.format("MM/DD/YYYY"),
            day: day,
            cardioTable: cardioTable,
            weightTable: weightTable,
            todayFocus: todayFocus,
        }
    }

    function logJSONObject() {
        console.log(getJSONObject());
        const request = {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(getJSONObject()),
        }
        fetch('http://127.0.0.1:8080/log', request)
            .then(response => response.json()) // Parse the JSON response
            .then(data => console.log(data)) // Handle the data
            .catch(error => console.error('Error:', error)); // Handle errors
    }

    return (
        <>
            <Container style={{minHeight: "100vh", minWidth: "100vw", alignItems: "center", justifyContent: "center", padding: "3rem" }}>
                <Container style={{maxHeight:"100%", minWidth: "40%"}}>
                    <Stack style={{ justifyContent:"space-between"}} direction="row" spacing={2}>
                        <LocalizationProvider dateAdapter={AdapterDayjs}>
                            <DatePicker sx={{ flex: 1 }} value={date} onChange={(newValue) => setDate(newValue)} />
                        </LocalizationProvider>
                        <Stack sx={{ flex: 1, alignItems: "center" }}>
                            <Typography>Week # {week}</Typography>
                            <Typography>Day # {getDayNum(day)}</Typography>
                        </Stack>
                        <FormControl sx={{ flex: 1 }}>
                            <InputLabel id="day-label">Day of Week</InputLabel>
                            <Select
                                onChange={handleChange}
                                id="day-select"
                                value={day}
                                label="Day of Week"
                                labelId="day-label"
                            >
                                <MenuItem value={"Monday"}>Monday</MenuItem>
                                <MenuItem value={"Tuesday"}>Tuesday</MenuItem>
                                <MenuItem value={"Wednesday"}>Wednesday</MenuItem>
                                <MenuItem value={"Thursday"}>Thursday</MenuItem>
                                <MenuItem value={"Friday"}>Friday</MenuItem>
                                <MenuItem value={"Saturday"}>Saturday</MenuItem>
                                <MenuItem value={"Sunday"}>Sunday</MenuItem>
                            </Select>
                        </FormControl>
                    </Stack>
                    <Container sx={{padding: "3rem"}}>
                        <Typography variant="h3">Cardio Training</Typography>
                        <TableContainer component={Paper}>
                            <Table sx={{ minWidth: 650 }} aria-label="simple table">
                                <TableHead>
                                    <TableRow>
                                        <TableCell align="left">Exercise</TableCell>
                                        <TableCell align="left">Minutes</TableCell>
                                        <TableCell align="left">Pace/Setting</TableCell>
                                        <TableCell align="left">Heart Rate</TableCell>
                                        <TableCell align="left">Calories Burned</TableCell>
                                    </TableRow>
                                </TableHead>
                                <TableBody>
                                        {
                                            cardioTable.map((row, m) => (
                                                <TableRow key={`cardio-${m}`}>
                                                    {row.map((_, n) => (
                                                    <TableCell key={`cardio-exercise-${m}-${n}`}><TextField variant="outlined" value={cardioTable[m][n]} onChange={(event) => (changeCardioTableVal(event, m, n))} /></TableCell>
                                                ))}
                                                </TableRow>
                                            ))
                                        }
                                </TableBody>
                            </Table>
                        </TableContainer>
                        <Button onClick={addCardio}>Add Cardio Exercise</Button>
                    </Container>
                    <Container sx={{padding: "3rem"}}>
                        <Stack
                            direction="column"
                            justifyContent="center"
                            alignContent="center"
                            spacing={2}
                            sx={{ minHeight: "100%" }}>
                            <Typography variant="h3">Strength Training</Typography>
                            <FormGroup  row>
                                <Typography alignSelf={"center"}>Today's Focus</Typography>
                                <FormControlLabel control={<Checkbox onChange={(event) => handleTodayFocus(event,"upper")}/>} label="Upper" />
                                <FormControlLabel control={<Checkbox onChange={(event) => handleTodayFocus(event, "lower")}/>} label="Lower" />
                                <FormControlLabel control={<Checkbox onChange={(event) => handleTodayFocus(event, "core")}/>} label="Core" />
                            </FormGroup>
                        </Stack>
                        <TableContainer component={Paper}>
                            <Table sx={{ minWidth: 650 }} aria-label="simple table">
                                <TableHead>
                                    <TableRow>
                                        <TableCell align="left">Muscle Group</TableCell>
                                        <TableCell align="left">Exercise</TableCell>
                                        {Array(sets).fill(null).map((_, index) => (getSetHeader(index+1)))}
                                    </TableRow>
                                </TableHead>
                                <TableBody>
                                    {weightTable.map((row, m) => (
                                    <TableRow
                                        key={m}
                                        sx={{ '&:last-child td, &:last-child th': { border: 0 } }}
                                    >
                                        <TableCell key={`muscle-group-${m}`} align="left"><TextField variant="outlined" value={row[0]} onChange={(event) => (changeTableVal(event, m, 0))} /></TableCell>
                                        <TableCell key={`exercise-${m}`} align="left"><TextField variant="outlined" value={row[1]} onChange={(event) => (changeTableVal(event, m, 1))} /></TableCell>
                                        {Array(sets).fill(null).map((_, n) => (
                                            <TableCell align="right" key={`set-${m}-${n}`}>
                                                <Grid container sx={{width: "100%", height: "100%", justifyContent: "space-between"}}>
                                                    <TextField sx={{width: "30%", marginRight: "auto"}} variant="standard" value={row[(n*2)+2]} onChange={(event) => (changeTableVal(event, m, (n*2) + 2))}/>
                                                    <TextField sx={{width: "30%"}} variant="standard" value={row[(n*2)+3]} onChange={(event) => (changeTableVal(event, m, (n*2) + 3))}
                                                               slotProps={{
                                                                   input: {
                                                                       endAdornment: <InputAdornment position="end">lbs</InputAdornment>,
                                                                   },
                                                               }}
                                                    />
                                                </Grid>
                                            </TableCell>
                                        ))}
                                    </TableRow>
                                    ))}
                                </TableBody>
                            </Table>
                        </TableContainer>
                        <Button onClick={addWeightRow}>Add Row</Button>
                        <Button onClick={addSet}>Add Set</Button>

                    </Container>
                        {/*<Grid container direction="row" justifyContent="space-between">*/}
                        {/*    <Grid><Typography>Muscle Group</Typography></Grid>*/}
                        {/*    <Grid><Typography>Exercise</Typography></Grid>*/}
                        {/*    {Array(sets).fill(null).map((_, index) => (getSetHeader(index+1)))}*/}
                        {/*    <Button onClick={addSet}>Add Set</Button>*/}
                        {/*</Grid>*/}
                        {/*<Grid container>*/}
                        {/*    <TextareaAutosize placeholder="Comments"></TextareaAutosize>*/}
                        {/*</Grid>*/}
                    <Button onClick={logJSONObject}>Log today!</Button>
                </Container>

            </Container>
        </>
    );
}

export default Form;