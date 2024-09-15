import { useCallback, useMemo, useState } from 'react'
import './App.css'
import { generate_truth_table, TruthTableEntry } from 'truth-table-gen'
import { Alert, Button, Container, Input, Paper, Stack, Table, TableBody, TableCell, TableContainer, TableHead, TableRow } from '@mui/material'
import { table } from 'table';
function App() {
  const [prop, setProp] = useState("")
  const [data, setData] = useState<TruthTableEntry[] | null>(null)
  const [err, setErr] = useState("")
  const gen = useCallback(() => {
    try {
      setErr("")
      setData(generate_truth_table(prop))
    } catch (e: any) {
      setErr(e)
    }
  }, [prop])
  const asciiTableText = useMemo(() => {
    if (!data) return null;
    const tableData = [
      [...data[0].bv.map(key => key.symbol), '', ...data[0].comp.map(key => key.symbol)],
      ...data.map(entry => (
        [...entry.bv.map(key => key.value ? "T" : "F"), '', ...entry.comp.map(key => key.value ? "T" : "F")]
      )),
    ];
    const config = {
      border: {
        topBody: ``,
        topJoin: ``,
        topLeft: ``,
        topRight: ``,
        bottomBody: `-`,
        bottomJoin: ``,
        bottomLeft: ``,
        bottomRight: ``,
        bodyLeft: `│`,
        bodyRight: `│`,
        bodyJoin: `│`,
        joinBody: `-`,
        joinLeft: ` `,
        joinRight: ` `,
        joinJoin: '-',
      },
      columns: {
        [data[0].bv.length]: {
          width: 1,
          paddingLeft: 0,
          paddingRight: 0,
        }
      },
      columnDefault: {
        alignment: 'center',
      },
      drawHorizontalLine: (index: number) => {
        return index === 1
      },
      drawVerticalLine: (index: number, size: number) => {
        return index !== 0 && index !== size
      }
    }
    // @ts-ignore
    return table(tableData, config);
  }, [data])
  return (
    <Container>
      <Stack spacing={2} alignItems={"center"}>
        <Stack direction={"row"} spacing={2}>
          <Input placeholder="Enter a proposition" onChange={e => setProp(e.target.value)} value={prop} />
          <Button onClick={gen}>Generate</Button>
        </Stack>
        {err && <Alert severity="error"><pre>{err}</pre></Alert>}
        {data && <>
          <TableContainer component={Paper} style={{maxHeight: 500}}>
            <Table stickyHeader>
              <TableHead>
                <TableRow>
                  {data[0].bv.map((key, _) => (
                    <TableCell key={key.symbol}>{key.symbol}</TableCell>
                  ))}
                  {data[0].comp.map((key, _) => (
                    <TableCell key={key.symbol}>{key.symbol}</TableCell>
                  ))}
                </TableRow>
              </TableHead>
              <TableBody>
                {data.map((entry, i) => (
                  <TableRow key={i}>
                    {entry.bv.map((key, _) => (
                      <TableCell key={key.symbol}>{key.value ? "T" : "F"}</TableCell>
                    ))}
                    {entry.comp.map((key, _) => (
                      <TableCell key={key.symbol}>{key.value ? "T" : "F"}</TableCell>
                    ))}
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </TableContainer>
        </>}
        {asciiTableText && <pre>{asciiTableText}</pre>}
      </Stack>
    </Container>
  )
}

export default App
